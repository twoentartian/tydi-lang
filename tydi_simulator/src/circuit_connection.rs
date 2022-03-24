use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

use crate::circuit_port::Port;

#[derive(Debug, Clone)]
pub struct Connection {
    name: String,

    src_port: Option<Arc<RwLock<Port>>>,
    sink_port: Option<Arc<RwLock<Port>>>,

}