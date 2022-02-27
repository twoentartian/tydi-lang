use std::collections::HashMap;

pub trait ToTydiIL {
    fn to_tydi_il(&self, type_alias_map: &mut HashMap<String, (String, Vec<String>)>, depth: u32) -> String;
}