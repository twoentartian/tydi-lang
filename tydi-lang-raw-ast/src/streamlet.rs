use std::sync::{Arc, RwLock};
use crate::data_type::DataType;
use crate::scope::{Scope, ScopeType};

#[derive(Clone, Debug)]
pub enum StreamletType {
    UnknownType,
    NormalStreamlet,
    TemplateStreamlet(Vec<Arc<RwLock<DataType>>>),
}

#[derive(Clone, Debug)]
pub struct Streamlet {
    name: String,

    streamlet_type: StreamletType,
    scope: Arc<RwLock<Scope>>,
}

impl Streamlet {
    pub fn new(name_: String, type_: StreamletType) -> Self {
        let scope_ = Arc::new(RwLock::new(Scope::new(format!("streamlet_{}", name_.clone()), ScopeType::StreamletScope)));
        {
            scope_.write().unwrap().set_self_ref(scope_.clone());
        }
        Self {
            name: name_,
            streamlet_type: type_,
            scope: scope_,
        }
    }
}
