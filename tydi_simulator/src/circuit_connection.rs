use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

use crate::circuit_port::Port;
use crate::{generate_set, generate_get, generate_access};

#[derive(Clone)]
pub struct Connection {
    name: String,

    src_port: Option<Arc<RwLock<Port>>>,
    sink_port: Option<Arc<RwLock<Port>>>,


}

impl Connection {
    generate_access!(name, String, get_name, set_name);
    generate_access!(src_port, Option<Arc<RwLock<Port>>>, get_src_port, set_src_port);
    generate_access!(sink_port, Option<Arc<RwLock<Port>>>, get_sink_port, set_sink_port);

    pub fn new() -> Self {
        return Self {
            name: String::from(""),
            src_port: None,
            sink_port: None,
        }
    }

}
