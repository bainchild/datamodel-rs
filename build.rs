// this is the messiest rust that I've ever made (specifically the codegen)
// and it became this way primarily due to trial and error +
// the addition of new features without cleanup between or after
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    collections::HashMap,
    env::var_os,
    ffi::OsStr,
    path::{Path, PathBuf},
};
use strum::{AsRefStr, Display};
type VersionTag = [u32; 4];
#[derive(Debug, Clone, Serialize, Deserialize)]
enum SecurityQualification {
    None,
    PluginSecurity,
    LocalUserSecurity,
    RobloxScriptSecurity,
    RobloxSecurity,
    NotAccessibleSecurity,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum DocTagString {
    Deprecated,
    NotCreatable,
    Yields,
    NoYield,
    ReadOnly,
    NotReplicated,
    NotScriptable,
    NotBrowsable,
    CanYield,
    Hidden,
    Service,
    UserSettings,
    Settings,
    WriteOnly,
    CustomLuaState,
    PlayerReplicated,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct DocTagObject {
    preferred_descriptor_name: String,
    thread_safety: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
enum DocTag {
    String(DocTagString),
    Object(DocTagObject),
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
enum ClassMemberType {
    Property,
    Callback,
    Function,
    Event,
}
#[derive(Debug, Clone, AsRefStr, Display, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
enum TypeName {
    string,
    bool,
    int,
    int64,
    void,
    float,
    double,
    //
    Objects,
    Instance,
    Array,
    Map,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
enum TypeType {
    Tracked(TypeName),
    NotTracked(String),
}
#[derive(Debug, Clone, Serialize, Deserialize)]
enum TypeCategory {
    Enum,
    Primitive,
    Class,
    DataType,
    Group,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct TypeDef {
    category: TypeCategory,
    name: TypeType,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct ClassMemberAccessSecurity {
    read: SecurityQualification,
    write: SecurityQualification,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
enum ClassMemberSecurity {
    FunctionSecurity(SecurityQualification),
    AccessSecurity(ClassMemberAccessSecurity),
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct ClassMemberProperty {
    default: Option<String>,
    name: String,
    r#type: TypeDef,
    version: Option<VersionTag>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct ClassMemberSerialization {
    can_load: bool,
    can_save: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
enum ReturnType {
    Multiple(Vec<TypeDef>),
    Single(TypeDef),
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct ClassMember {
    category: Option<String>,
    member_type: ClassMemberType,
    name: String,
    security: Option<ClassMemberSecurity>,
    serialization: Option<ClassMemberSerialization>,
    tags: Option<Vec<DocTag>>,
    value_type: Option<TypeDef>,
    parameters: Option<Vec<ClassMemberProperty>>,
    return_type: Option<ReturnType>,
    version: Option<VersionTag>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Class {
    members: Vec<ClassMember>,
    memory_category: Option<String>,
    name: String,
    superclass: String,
    tags: Option<Vec<DocTag>>,
    version: Option<VersionTag>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct EnumItem {
    name: String,
    value: u64,
    version: Option<VersionTag>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Enum {
    items: Vec<EnumItem>,
    name: String,
    version: Option<VersionTag>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct APIDump {
    classes: Vec<Class>,
    enums: Vec<Enum>,
    version: u16,
}
fn sanitize_for_syntax(s: String) -> String {
    s.replace(&['-', ' ', '(', '/'], "_")
        .replace(&[')', '"', '>', '<'], "")
}
fn sanitize_for_indexing(s: String) -> String {
    match s.to_lowercase().as_str() {
        "type" => "r#type".to_string(),
        "loop" => "r#loop".to_string(),
        _ => s.to_lowercase().clone(),
    }
}
fn typedef_to_typestring(membername: String, val_type: TypeDef) -> String {
    match val_type.category {
        TypeCategory::Enum => {
            "Enum".to_string()
                + match val_type.name {
                    TypeType::Tracked(name) => name.to_string(),
                    TypeType::NotTracked(str) => str.to_string(),
                }
                .as_str()
        }
        TypeCategory::Primitive => match val_type.name {
            TypeType::Tracked(type_name) => match type_name {
                TypeName::int => "i32".to_string(),
                TypeName::int64 => "i64".to_string(),
                TypeName::float => "f32".to_string(),
                TypeName::double => "f64".to_string(),
                TypeName::string => "String".to_string(),
                TypeName::bool => "bool".to_string(),
                TypeName::void => "Option<i32>/*void*/".to_string(),
                _ => unimplemented!(),
            },
            TypeType::NotTracked(str) => str,
        },
        TypeCategory::Class => match val_type.name {
            TypeType::Tracked(type_name) => match type_name {
                TypeName::Instance => "Option<NodeId>".to_string(),
                _ => unimplemented!(),
            },
            TypeType::NotTracked(str) => {
                if str == membername {
                    "Option<NodeId>/*Box<".to_string() + str.as_str() + ">*/"
                } else if str == "BinaryString" {
                    "BString".to_string()
                } else {
                    "Option<NodeId>/*Box<".to_string() + str.as_str() + ">*/"
                }
            }
        },
        TypeCategory::DataType => match val_type.name {
            TypeType::NotTracked(type_name) => match type_name.as_str() {
                "BinaryString" => "BString".to_string(),
                "ProtectedString" => "String/*protected*/".to_string(),
                _ => type_name,
            },
            _ => unimplemented!(),
        },
        _ => match val_type.name {
            TypeType::Tracked(type_name) => type_name.to_string(),
            TypeType::NotTracked(str) => str,
        },
    }
}
fn main() {
    println!("cargo::rerun-if-changed=scripts/changes");
    let out_dir = var_os("OUT_DIR").unwrap();
    let dest = Path::new(&out_dir).join("versioned_instances.rs");
    // let dest2 = Path::new(&out_dir).join("root_impls.rs");
    // todo: diff the versions for version compatibility
    let mut latest_version: VersionTag = [0, 0, 0, 0];
    let mut versioned: APIDump = APIDump {
        classes: Vec::new(),
        enums: Vec::new(),
        version: 1,
    };
    {
        let mut sorted: Vec<(VersionTag, PathBuf)> = Vec::with_capacity(100);
        for file in std::fs::read_dir("scripts/changes/").unwrap() {
            if let Ok(f) = file {
                // these are all fatal errors
                // the unwraps are purposeful I swear
                let path = f.path();
                if path.extension().is_some_and(|x| x == OsStr::new("json")) {
                    let stem = path.file_stem().unwrap().to_string_lossy();
                    let version: VersionTag = stem
                        .rsplit_terminator("-")
                        .take(1)
                        .collect::<String>()
                        .split_terminator(".")
                        .map(|x| x.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>()
                        .try_into()
                        .unwrap();
                    sorted.push((version, path.clone()));
                }
            }
        }
        sorted.sort_by(|a, b| {
            let av = a.0;
            let bv = b.0;
            // 100.123.123.12345
            // usually like 0.600.1.12345, but can be 3 digits for the first 3 (probably)
            // this should be math but I can't make it
            // the simple solution of comparing the elements of the slice fails from
            // non-strong total ordering
            let ascore: u64 = av
                .iter()
                .map(|x| format!("{:0<6}", x))
                .collect::<Vec<String>>()
                .concat()
                .parse()
                .unwrap();
            let bscore: u64 = bv
                .iter()
                .map(|x| format!("{:0<6}", x))
                .collect::<Vec<String>>()
                .concat()
                .parse()
                .unwrap();
            // println!("cargo::warning={:?} <=> {:?}", ascore, bscore);
            ascore.cmp(&bscore)
        });
        let last = sorted.last().unwrap();
        latest_version = last.0;
        versioned = serde_json::from_reader(std::fs::File::open(last.1.clone()).unwrap()).unwrap();
    }
    let mut class2super: HashMap<String, String> = HashMap::with_capacity(100);
    for class in versioned.classes.iter() {
        class2super.insert(class.name.clone(), class.superclass.clone());
    }
    let mut generated: String = String::with_capacity(500);
    generated += format!(
        "pub(crate) const VERSION: [u32;4] = [{},{},{},{}];\n",
        latest_version[0], latest_version[1], latest_version[2], latest_version[3]
    )
    .as_str();
    for en in versioned.enums {
        generated += format!(
            "#[derive(Debug, Default, Clone, Copy, EnumString, strum_macros::VariantNames)]\npub(crate) enum Enum{} {{\n",
            en.name
        )
        .as_str();
        let mut first = true;
        if en.items.iter().any(|x| x.name == "Default") {
            first = false;
        }
        if en.items.len() == 0 {
            generated += "\t#[default]\n\t#[strum(disabled)]\n\tEnumIsUnsetAndNoGoodValues = 0,\n";
        } else {
            for item in en.items {
                if first || item.name == "Default" {
                    generated += "\t#[default]\n";
                    first = false;
                }
                generated += format!(
                    "\t#[strum(serialize = \"Enum.{}.{}\")]\n\t{} = {},\n",
                    en.name,
                    item.name.clone(),
                    if item.name == "Self" {
                        "Selfish/*Self*/".to_string()
                    } else {
                        item.name
                    },
                    item.value
                )
                .as_str();
            }
        }
        generated += "}\n";
    }
    let mut classnames = Vec::with_capacity(versioned.classes.len() - 1);
    // let object_not_introduced = !versioned.classes.clone().iter().any(|x| x.name=="Object");
    for member in versioned.classes.clone() {
        // if member.superclass != "<<<ROOT>>>" {// <<<ROOT>>> classes should be implemented manually.
            // assert_ne!(member.superclass, "<<<ROOT>>>"); 
            classnames.push(member.name.clone());
            let mut class = format!(
                "#[derive(Debug,Clone,Default)]\npub struct {} {{\n",
                member.name
            );
            let mut class_impl = "".to_string(); //format!("impl {}Trait for {0} {{\n", member.name);
            let mut dedup_members: Vec<ClassMember> = member.members.clone();
            dedup_members.sort_by(|a, b| {
                let newa = sanitize_for_syntax(a.name.clone()).to_lowercase();
                let newb = sanitize_for_syntax(b.name.clone()).to_lowercase();
                (&newa).cmp(&newb)
            });
            dedup_members.dedup_by(|a, b| {
                sanitize_for_syntax(a.name.clone()).to_lowercase()
                    == sanitize_for_syntax(b.name.clone()).to_lowercase()
            });
            // if versioned
            //     .classes
            //     .clone()
            //     .iter()
            //     .filter(|x| x.superclass == member.name)
            //     .collect::<Vec<&Class>>()
            //     .len()
            //     == 0
            // {
            //     for prop in dedup_members.clone().iter() {
            //         if prop.name == "Instance" || prop.value_type.is_none() {
            //             continue;
            //         }
            //         let propname = sanitize_for_syntax(prop.name.clone());
            //         let typename = typedef_to_typestring(
            //             member.name.clone(),
            //             prop.value_type.clone().unwrap(),
            //         );
            //         let indexname = sanitize_for_indexing(propname.clone());
            //         class_impl += format!(
            //             "\tfn {}(&self) -> &{} {{&self.{}}}\n",
            //             indexname, typename, propname
            //         )
            //         .as_str();
            //         class_impl += format!(
            //             "\tfn set_{}(&mut self, val: {}) {{self.{} = val;}}\n",
            //             propname.to_lowercase(),
            //             typename,
            //             propname
            //         )
            //         .as_str();
            //     }
            // }
            // class_impl += "}\n";
            let mut prevclass: Option<&Class> = None;
            let mut superclass: Option<&Class> = Some(&member);
            let mut members: HashMap<String, &ClassMember> = HashMap::with_capacity(50);
            let mut ancestors: Vec<String> = Vec::new();
            // let mut class_impl_2: String = "".to_string();
            while let Some(cl) = superclass {
                println!("{} merging in props from {}", member.name, cl.name);
                for mem in cl.members.iter() {
                    if !members.contains_key(&mem.name)
                        && mem.member_type == ClassMemberType::Property
                    {
                        // if !dedup_members.iter().any(|x| sanitize_for_syntax(x.name.to_lowercase()) == sanitize_for_syntax(mem.name.to_lowercase())) {dedup_members.push(mem.clone())};
                        members.insert(mem.name.clone(), mem);
                    }
                }
                ancestors.push(cl.name.clone());
                // println!("cargo::warning=prevclass {:?}, {}",prevclass,cl.name);
                class_impl += format!("impl {}Trait for {} {{\n", cl.name, member.name).as_str();
                let mut newmembers = cl.members.clone();
                newmembers.sort_by(|a, b| {
                    let newa = sanitize_for_syntax(a.name.clone()).to_lowercase();
                    let newb = sanitize_for_syntax(b.name.clone()).to_lowercase();
                    (&newa).cmp(&newb)
                });
                newmembers.dedup_by(|a, b| {
                    sanitize_for_syntax(a.name.clone()).to_lowercase()
                        == sanitize_for_syntax(b.name.clone()).to_lowercase()
                });
                for prop in newmembers.iter() {
                    if prop.name.to_lowercase() == "parent"
                        || prop.name.to_lowercase() == "classname"
                        || prop.value_type.is_none()
                    {
                        continue;
                    }
                    let propname = sanitize_for_syntax(prop.name.clone());
                    let typename =
                        typedef_to_typestring(cl.name.clone(), prop.value_type.clone().unwrap());
                    let indexname = sanitize_for_indexing(propname.clone());
                    class_impl += format!(
                        "\tfn {}(&self) /*b4*/ -> &{} {{&self.{}}}\n",
                        indexname, typename, propname
                    )
                    .as_str();
                    class_impl += format!(
                        "\tfn set_{}(&mut self, val: {}) {{self.{} = val;}}\n",
                        propname.to_lowercase(),
                        typename,
                        propname
                    )
                    .as_str();
                }
                /*if cl.superclass == member.superclass {
                class_impl += format!(
                    "\tfn as_{}(&self) /*firstif*/
 -> Option<&dyn {}Trait> {{Some(self)}}\n",
                        cl.name.to_lowercase(),
                        cl.name
                    )
                    .as_str();
                    class_impl += format!(
                "\tfn as_mut_{}(&mut self) /*firstif*/
 -> Option<&mut dyn {}Trait> {{Some(self)}}\n",
                        cl.name.to_lowercase(),
                        cl.name
                    )
                    .as_str();
                } else */                if prevclass.is_some() {
                    let soup = &prevclass.unwrap().name;
                    class_impl += format!(
                        "\tfn as_{}(&self) /*secif*/ -> Option<&dyn {}Trait> {{Some(self)}}\n",
                        soup.to_lowercase(),
                        soup
                    )
                    .as_str();
                    class_impl += format!(
                        "\tfn as_mut_{}(&mut self) /*secif*/ -> Option<&mut dyn {}Trait> {{Some(self)}}\n",
                        soup.to_lowercase(),
                        soup
                    )
                    .as_str();
                }
                class_impl += "}\n";
                if let Some(next_superclass) = class2super.get(&cl.name) {
                    let lastclass = superclass;
                    prevclass = lastclass;
                    superclass = versioned
                        .classes
                        .iter()
                        .find(|x| x.name == *next_superclass);
                } else {
                    break;
                }
            }
            class_impl += format!("impl RootTrait for {} {{\n",member.name).as_str();
            class_impl += format!(
                "\tfn classname(&self) -> String {{ return {:?}.to_string() }}\n",
                member.name
            )
            .as_str();
            class_impl += format!(
                "\tfn is_a(&self,s: &str) -> bool {{ return s=={:?} {}}}\n",
                member.name,
                ancestors
                    .iter()
                    .map(|x| format!("|| s=={:?} ", x))
                    .collect::<Vec<String>>()
                    .concat()
            )
            .as_str();
            let from_root = ancestors.last().unwrap();
            class_impl += format!("\tfn as_{}(&self) -> Option<&dyn {}Trait> {{Some(self)}}\n",from_root.to_lowercase(),from_root).as_str();
            class_impl += format!("\tfn as_mut_{}(&mut self) -> Option<&mut dyn {}Trait> {{Some(self)}}\n",from_root.to_lowercase(),from_root).as_str();
            class_impl += "}\n";
            // class_impl += class_impl_2.as_str();
            for prop in members.iter().map(|x| x.1) {
                if prop.name.to_lowercase() == "parent"
                    || prop.name.to_lowercase() == "classname"
                    || prop.value_type.is_none()
                {
                    continue;
                }
                class += format!(
                    "\t{}: {},\n",
                    sanitize_for_syntax(prop.name.clone()),
                    typedef_to_typestring(member.name.clone(), prop.value_type.clone().unwrap())
                )
                .as_str();
            }
            class += "}\n";
            // class += format!("traitcast::traitcast!(struct {}: {}Trait{});\n",member.name,member.superclass,ancestors).as_str();
            class += format!(
                "pub trait {}Trait: {}Trait {{\n",
                member.name, if member.superclass=="<<<ROOT>>>" { "Root".to_string() } else {member.superclass}
            )
            .as_str();
            let mut len: usize = 0;
            for sub in versioned
                .classes
                .clone()
                .iter()
                .filter(|x| x.superclass == member.name)
            {
                len += 1;
                class += format!(
                    "\tfn as_{}(&self) -> Option<&dyn {}Trait> {{None}}\n",
                    sub.name.to_lowercase(),
                    sub.name
                )
                .as_str();
                class += format!(
                    "\tfn as_mut_{}(&mut self) -> Option<&mut dyn {}Trait> {{None}}\n",
                    sub.name.to_lowercase(),
                    sub.name
                )
                .as_str();
            }
            for prop in dedup_members.iter() {
                if prop.name.to_lowercase() == "parent"
                    || prop.name.to_lowercase() == "classname"
                    || prop.value_type.is_none()
                {
                    continue;
                }
                let propname = sanitize_for_syntax(prop.name.clone());
                let typename =
                    typedef_to_typestring(member.name.clone(), prop.value_type.clone().unwrap());
                let indexname = sanitize_for_indexing(propname.clone());
                class += format!("\tfn {}(&self) /*a4*/ -> &{};\n", indexname, typename).as_str();
                class += format!(
                    "\tfn set_{}(&mut self, val: {});\n",
                    propname.to_lowercase(),
                    typename
                )
                .as_str();
            }
            // class += "\n\tfn is_a(&self,s: &str) -> bool;";
            // class += "\n\tfn classname(&self) -> &'static str;\n";
            class += "}\n";
            class += class_impl.as_str();
            generated += class.as_str();
        // }
    }
    // let mut gen2: String = "".to_string();
    // for member in versioned.classes.clone().iter().filter(|x| x.superclass=="<<<ROOT>>>") {
    //     gen2 += format!("// impl {}Trait {{",member.name).as_str();
    //     for subclass in versioned
    //         .classes
    //         .clone()
    //         .iter()
    //         .filter(|x| x.superclass == member.name)
    //     {
    //         gen2 += format!(
    //             "\n\tfn as_{}(&self) -> Option<&dyn {}Trait> {{None}}",
    //             subclass.name.to_lowercase(),
    //             subclass.name
    //         )
    //         .as_str();
    //         gen2 += format!(
    //             "\n\tfn as_mut_{}(&mut self) -> Option<&mut dyn {}Trait> {{None}}",
    //             subclass.name.to_lowercase(),
    //             subclass.name
    //         )
    //         .as_str();
    //     }
    //     if member.name == "Object" || (member.name == "Instance" && object_not_introduced) {
    //         gen2 += format!("\n\tfn classname() -> &'static str {{\"{}\"}}",member.name).as_str();
    //         gen2 += format!("\n\tfn is_a(s: &str) -> bool {{return s==\"{}\"}}",member.name).as_str();
    //     }
    //     gen2 += "\n//}";
    // }
    // std::fs::write(&dest2, gen2.as_str()).unwrap();
    // generated.push_str(format!(
    //     "#[derive(Debug,Clone,VariantNames)]\npub enum ClassType {{{}}}",
    //     classnames
    //         .iter()
    //         .map(|x| x.to_owned() + "(" + x.as_str() + ")")
    //         .collect::<Vec<String>>()
    //         .join(",\n")
    // ));
    std::fs::write(&dest, generated.as_str()).unwrap();
}
