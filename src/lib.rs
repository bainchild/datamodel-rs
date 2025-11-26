// #![trait_upcasting]
// this is invalid, and using include!() makes source file attributes not work.
// #[path = concat!(env!("OUT_DIR"),"/versioned_instances.rs")]
// Guess we're hardcoding the path!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Kinda defeats the whole purpose of OUT_DIR but sure if we can't fix #![allow()] in include!() 'd files
// #[path = "../target/debug/build/datamodel_rs-f2ab66976eeb2e61/out/versioned_instances.rs"]
pub mod datatypes;
#[path = "versioned_instances.rs"]
pub mod generated;
use ego_tree::{NodeId, Tree};
use datatypes::Color3;
use generated::*;
use rbx_dom_weak::WeakDom;
use std::any::Any;
pub trait RootTrait: Any {
    fn classname(&self) -> String {"RootTrait".to_string()}
    fn is_a(&self, s: &str) -> bool {return s == "RootTrait";}
    fn as_unknownobject(&self) -> Option<&dyn UnknownObjectTrait> {None}
    fn as_mut_unknownobject(&mut self) -> Option<&mut dyn UnknownObjectTrait> {None}
    fn as_instance(&self) -> Option<&dyn InstanceTrait> {None}
    fn as_mut_instance(&mut self) -> Option<&mut dyn InstanceTrait> {None}
    // fn as_object(&self) -> Option<&dyn ObjectTrait> {None}
    // fn as_mut_object(&mut self) -> Option<&mut dyn ObjectTrait> {None}
}
// class & name, all that's needed for a good number of instances you
// don't want to implement but that still need to exist and contain children
macro_rules! minimum_support {
    ( $x:expr, $a:expr, $( $y:ident ),* ) => {
        match $x {
            $(
            stringify!($y) =>
                Some(
                        Box::new(
                            {
                                let mut temp = $y::default();
                                temp.set_name($a);
                                temp
                        }) as Box<dyn RootTrait>,
                    )
            ,)*
            _ => None
        }
    };
}
pub fn rbxdom_instance_to_dmrs(
    tree: &mut Tree<Box<dyn RootTrait>>,
    dom: &WeakDom,
    inst: &rbx_dom_weak::Instance,
    parent_node_id: NodeId,
) -> Result<NodeId, String> {
    // println!("{} {}", inst.name, inst.class.as_str());
    let mut res: Option<Box<dyn RootTrait>> = minimum_support!(
        inst.class.as_str(),
        inst.name.clone(),
        Workspace,
        Model,
        RunService,
        ReplicatedStorage,
        StarterPlayer
    );
    if res.is_none() {
        res = match inst.class.as_str() {
            "Part" => {
                let mut part = Part::default();
                part.set_name(inst.name.clone());
                // part specific properties
                Some(Box::new(part) as Box<dyn RootTrait>)
            }
            "SpawnLocation" => {
                let mut spawn = SpawnLocation::default();
                spawn.set_name(inst.name.clone());
                if let Some(rbx_types::Variant::Bool(b)) =
                    inst.properties.get(&rbx_dom_weak::ustr("Neutral"))
                {
                    spawn.set_neutral(*b);
                }
                if let Some(rbx_types::Variant::Bool(b)) =
                    inst.properties.get(&rbx_dom_weak::ustr("Enabled"))
                {
                    spawn.set_enabled(*b);
                }
                // spawnlocation specific properties
                Some(Box::new(spawn) as Box<dyn RootTrait>)
            }
            _ => None,
        }
    }
    if let Some(mut result) = res {
        if let Some(mut instance) = result.as_mut_instance() {
            // instance properties
            if let Some(mut pvinst) = instance.as_mut_pvinstance() {
                // pvinstance properties
                if let Some(mut basepart) = pvinst.as_mut_basepart() {
                    // basepart properties
                    // for (k, v) in inst.properties.clone() {
                    //     println!("kv {} {:?}", k, v);
                    // }
                    if let Some(rbx_types::Variant::Bool(b)) =
                        inst.properties.get(&rbx_dom_weak::ustr("Anchored"))
                    {
                        basepart.set_anchored(*b);
                    }
                    if let Some(rbx_types::Variant::Bool(b)) =
                        inst.properties.get(&rbx_dom_weak::ustr("CanCollide"))
                    {
                        basepart.set_cancollide(*b);
                    }
                    // velocity
                    // size
                    // cframe
                    // material
                    if let Some(rbx_types::Variant::Color3uint8(col)) =
                        inst.properties.get(&rbx_dom_weak::ustr("Color"))
                    {
                        basepart.set_color(Color3(col.r as f32,col.g as f32,col.b as f32));
                    }
                }
            }
        }
        let nodeid = tree.get_mut(parent_node_id).unwrap()
            .append(
                result
            )
            .id();
        for child in inst.children().iter().filter_map(|x| dom.get_by_ref(*x)) {
            match rbxdom_instance_to_dmrs(tree, dom, child, nodeid) {
                Ok(ins) => {
                    // nodeid
                }
                Err(e) => {
                    // println!("suberror {}", e)
                }
            }
        }
        Ok(nodeid)
    } else {
        Err("todo".to_string())
    }
}
// todo: this but without box(?)
pub fn rbx_dom_to_tree<'a>(dom: WeakDom) -> Tree<Box<dyn RootTrait>> {
    let mut datamodel = Tree::new(
            Box::new(DataModel::default()) as Box<dyn RootTrait>
        );
    assert_eq!(dom.root().class, "DataModel");
    // for (k,v) in dom.root().properties.iter() {
    //     println!("props {} {:?}",k.as_str(),v);
    // }
    let root_node_id = datamodel.root().id();
    for child in dom
        .root()
        .children()
        .iter()
        .filter_map(|x| dom.get_by_ref(*x))
    {
        match rbxdom_instance_to_dmrs(&mut datamodel, &dom, child, root_node_id) {
            Ok(ins) => {
                // nodeid
            }
            Err(e) => {
                // println!("error {}", e)
            }
        }
    }
    datamodel
}

// these run but they aren't good tests so they are commented out
// but provide a short example until docs/examples are provided.
// #[cfg(test)]
// mod test {
//     use std::any::Any;
//     use crate::{DataModel, rbx_dom_to_tree};
//     use crate::instances::{DataModelTrait, InstanceTrait};
//     use rbx_dom_weak::{InstanceBuilder, WeakDom};
//     #[test]
//     fn testing() {
//         let root: &dyn InstanceTrait = &DataModel::default();
//         println!("{:?}",root.classname());
//         let res: Option<&dyn DataModelTrait> = (||->Option<&dyn DataModelTrait>{root.as_serviceprovider()?.as_datamodel()})();
//         println!("{:?}",res.is_some());
//         println!("{:?}",res.unwrap().classname())
//     }
//     #[test]
//     fn dom2tree() {
//         let dom = WeakDom::new(InstanceBuilder::new("DataModel"));
//         let mut tree = rbx_dom_to_tree(dom);
//         if let Some(id) = tree.root_node_id() {
//             if let Ok(root) = tree.remove_node(id.clone(),id_tree::RemoveBehavior::LiftChildren) {
//                 if let Some(r2) = (root.data().as_serviceprovider().unwrap().as_datamodel().unwrap() as &dyn Any).downcast_ref::<DataModel>() {
//                     println!("Datamodle: {:?}", r2);
//                  } else {
//                     println!("failed downcast {:?}",root.data().classname());
//                  }
//             } else {
//                 println!("remove failed");
//             }
//         } else {
//             println!("root node id failed");
//         }
//     }
// }
