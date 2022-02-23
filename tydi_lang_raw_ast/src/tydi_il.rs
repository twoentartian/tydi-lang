use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use crate::logical_data_type::LogicalDataType;

pub trait ToTydiIL {
    fn to_tydi_il(&self, type_alias_map: &mut HashMap<String, String>) -> String;
}