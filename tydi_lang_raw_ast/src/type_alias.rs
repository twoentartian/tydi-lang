use std::sync::{Arc, RwLock};
use deep_clone::DeepClone;
use crate::inferable::{Inferable};
use crate::logical_data_type::LogicalDataType;
use crate::util::{PrettyPrint, EnableDocument};
use crate::{generate_get, generate_set, generate_access};

use derived_macro::EnableDocument;

#[derive(Clone, Debug, EnableDocument)]
pub struct TypeAlias {
    name: String,
    docu: Option<String>,
    type_infer: Inferable<Arc<RwLock<LogicalDataType>>>,
}

impl DeepClone for TypeAlias {
    fn deep_clone(&self) -> Self {
        return Self {
            name: self.name.clone(),
            type_infer: self.type_infer.deep_clone(),
            docu: self.docu.deep_clone(),
        }
    }
}

impl TypeAlias {
    generate_get!(name, String, get_name);
    generate_access!(type_infer, Inferable<Arc<RwLock<LogicalDataType>>>, get_type_infer, set_type_infer);

    pub fn new(name_: String, type_: Inferable<Arc<RwLock<LogicalDataType>>>) -> Self {
        Self {
            name: name_.clone(),
            type_infer: type_.clone(),
            docu: None,
        }
    }
}

impl From<TypeAlias> for String {
    fn from(t: TypeAlias) -> Self {
        return String::from(t.type_infer);
    }
}

impl PrettyPrint for TypeAlias {
    fn pretty_print(&self, depth: u32, verbose: bool) -> String {
        return self.type_infer.pretty_print(depth, verbose);
    }
}
