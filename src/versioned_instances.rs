use crate::{RootTrait, datatypes::*};
use ego_tree::NodeId;
use std::any::Any;
use strum::VariantNames;
use strum_macros::EnumString;
include!(concat!(env!("OUT_DIR"), "/versioned_instances.rs"));
pub trait UnknownObjectTrait: RootTrait {}
#[derive(Debug, Clone)]
pub struct UnknownObject {
    classname: String,
    ancestors: Vec<String>
}
impl UnknownObjectTrait for UnknownObject {}
// impl ObjectTrait for UnknownObject {}
impl RootTrait for UnknownObject {
    fn classname(&self) -> String {
        self.classname.clone()
    }
    fn is_a(&self, s: &str) -> bool {
        self.classname == s || self.ancestors.contains(&s.to_string())
    }
    fn as_unknownobject(&self) -> Option<&dyn UnknownObjectTrait> {
        Some(self)
    }
    fn as_mut_unknownobject(&mut self) -> Option<&mut dyn UnknownObjectTrait> {
        Some(self)
    }
}
