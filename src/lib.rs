// #![trait_upcasting]
// this is invalid, and using include!() makes source file attributes not work.
// #[path = concat!(env!("OUT_DIR"),"/versioned_instances.rs")]
// Guess we're hardcoding the path!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Kinda defeats the whole purpose of OUT_DIR but sure if we can't fix #![allow()] in include!() 'd files
// #[path = "../target/debug/build/datamodel_rs-f2ab66976eeb2e61/out/versioned_instances.rs"]
pub mod datatypes;
#[path = "versioned_instances.rs"]
pub mod instances;
use id_tree::{Node, NodeId, Tree, TreeBuilder};
use instances::*;
use rbx_dom_weak::WeakDom;
use std::any::Any;
pub trait ObjectTrait: Any {
    fn classname(&self) -> &'static str {
        "Object"
    }
    fn is_a(&self, s: &str) -> bool {
        return s == "Object";
    }
    // unsure why you would do this, but it exists if you need it.
    fn as_unknownobject(&self) -> Option<&dyn UnknownObjectTrait> {
        None
    }
    fn as_mut_unknownobject(&mut self) -> Option<&mut dyn UnknownObjectTrait> {
        None
    }
    // these need to be transplanted from a build script generated file,
    // cause the subclasses of the root class need to be present for trait traversal,
    // but objecttrait needs to be here for any object-global functionality
    fn as_capture(&self) -> Option<&dyn CaptureTrait> {
        None
    }
    fn as_mut_capture(&mut self) -> Option<&mut dyn CaptureTrait> {
        None
    }
    fn as_configsnapshot(&self) -> Option<&dyn ConfigSnapshotTrait> {
        None
    }
    fn as_mut_configsnapshot(&mut self) -> Option<&mut dyn ConfigSnapshotTrait> {
        None
    }
    fn as_editableimage(&self) -> Option<&dyn EditableImageTrait> {
        None
    }
    fn as_mut_editableimage(&mut self) -> Option<&mut dyn EditableImageTrait> {
        None
    }
    fn as_editablemesh(&self) -> Option<&dyn EditableMeshTrait> {
        None
    }
    fn as_mut_editablemesh(&mut self) -> Option<&mut dyn EditableMeshTrait> {
        None
    }
    fn as_executedremotecommand(&self) -> Option<&dyn ExecutedRemoteCommandTrait> {
        None
    }
    fn as_mut_executedremotecommand(&mut self) -> Option<&mut dyn ExecutedRemoteCommandTrait> {
        None
    }
    fn as_instance(&self) -> Option<&dyn InstanceTrait> {
        None
    }
    fn as_mut_instance(&mut self) -> Option<&mut dyn InstanceTrait> {
        None
    }
    fn as_mlsession(&self) -> Option<&dyn MLSessionTrait> {
        None
    }
    fn as_mut_mlsession(&mut self) -> Option<&mut dyn MLSessionTrait> {
        None
    }
    fn as_terrainiterateoperation(&self) -> Option<&dyn TerrainIterateOperationTrait> {
        None
    }
    fn as_mut_terrainiterateoperation(&mut self) -> Option<&mut dyn TerrainIterateOperationTrait> {
        None
    }
    fn as_terrainmodifyoperation(&self) -> Option<&dyn TerrainModifyOperationTrait> {
        None
    }
    fn as_mut_terrainmodifyoperation(&mut self) -> Option<&mut dyn TerrainModifyOperationTrait> {
        None
    }
    fn as_terrainreadoperation(&self) -> Option<&dyn TerrainReadOperationTrait> {
        None
    }
    fn as_mut_terrainreadoperation(&mut self) -> Option<&mut dyn TerrainReadOperationTrait> {
        None
    }
    fn as_terrainwriteoperation(&self) -> Option<&dyn TerrainWriteOperationTrait> {
        None
    }
    fn as_mut_terrainwriteoperation(&mut self) -> Option<&mut dyn TerrainWriteOperationTrait> {
        None
    }
    fn as_webstreamclient(&self) -> Option<&dyn WebStreamClientTrait> {
        None
    }
    fn as_mut_webstreamclient(&mut self) -> Option<&mut dyn WebStreamClientTrait> {
        None
    }
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
                        }) as Box<dyn InstanceTrait>,
                    )
            ,)*
            _ => None
        }
    };
}
pub fn rbxdom_instance_to_dmrs(
    tree: &mut Tree<Box<dyn InstanceTrait>>,
    dom: &WeakDom,
    inst: &rbx_dom_weak::Instance,
    parent_node_id: &NodeId,
) -> Result<NodeId, String> {
    println!("{} {}", inst.name, inst.class.as_str());
    let mut res: Option<Box<dyn InstanceTrait>> = minimum_support!(
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
                Some(Box::new(part) as Box<dyn InstanceTrait>)
            }
            "SpawnLocation" => {
                let mut spawn = SpawnLocation::default();
                spawn.set_name(inst.name.clone());
                if let Some(rbx_types::Variant::Bool(b)) =
                    inst.properties.get(&rbx_dom_weak::ustr("Neutral"))
                {
                    spawn.set_neutral(*b);
                }
                // spawnlocation specific properties
                Some(Box::new(spawn) as Box<dyn InstanceTrait>)
            }
            _ => None,
        }
    }
    if let Some(mut result) = res {
        if let Some(mut pvinst) = result.as_mut_pvinstance() {
            // pvinstance properties
            if let Some(mut basepart) = pvinst.as_mut_basepart() {
                // basepart properties
                for (k, v) in inst.properties.clone() {
                    println!("kv {} {:?}", k, v);
                }
            }
        }
        let nodeid = tree
            .insert(
                Node::new(result),
                id_tree::InsertBehavior::UnderNode(&parent_node_id),
            )
            .unwrap();
        for child in inst.children().iter().filter_map(|x| dom.get_by_ref(*x)) {
            match rbxdom_instance_to_dmrs(tree, dom, child, &nodeid) {
                Ok(ins) => {
                    // nodeid
                }
                Err(e) => {
                    println!("suberror {}", e)
                }
            }
        }
        Ok(nodeid)
    } else {
        Err("todo".to_string())
    }
}
// todo: this but without box(?)
pub fn rbx_dom_to_tree<'a>(dom: WeakDom) -> Tree<Box<dyn InstanceTrait>> {
    let mut datamodel = TreeBuilder::new()
        .with_node_capacity(30)
        .with_root(Node::new(
            Box::new(DataModel::default()) as Box<dyn InstanceTrait>
        ))
        .build();
    assert_eq!(dom.root().class, "DataModel");
    // for (k,v) in dom.root().properties.iter() {
    //     println!("props {} {:?}",k.as_str(),v);
    // }
    let root_node_id = datamodel.root_node_id().unwrap().clone();
    for child in dom
        .root()
        .children()
        .iter()
        .filter_map(|x| dom.get_by_ref(*x))
    {
        match rbxdom_instance_to_dmrs(&mut datamodel, &dom, child, &root_node_id) {
            Ok(ins) => {
                // nodeid
            }
            Err(e) => {
                println!("error {}", e)
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
