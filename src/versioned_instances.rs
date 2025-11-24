use crate::{ObjectTrait, datatypes::*};
use id_tree::NodeId;
use mlua::{BString, Vector as Vector3};
use std::any::Any;
use strum::VariantNames;
use strum_macros::EnumString;
include!(concat!(env!("OUT_DIR"), "/versioned_instances.rs"));
pub trait UnknownObjectTrait: ObjectTrait {}
#[derive(Debug, Clone)]
pub struct UnknownObject {}
impl UnknownObjectTrait for UnknownObject {}
impl ObjectTrait for UnknownObject {
    fn as_unknownobject(&self) -> Option<&dyn UnknownObjectTrait> {
        Some(self)
    }
    fn as_mut_unknownobject(&mut self) -> Option<&mut dyn UnknownObjectTrait> {
        Some(self)
    }
}
