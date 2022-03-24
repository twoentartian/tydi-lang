use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

use crate::circuit_connection::Connection;
use crate::circuit_frequency::Frequency;

#[derive(Debug, Clone)]
pub struct Port {
    name: String,

    bind_connection: Option<Arc<RwLock<Connection>>>,
    clock_domain: Frequency,
}

impl Port {
    pub fn new() -> Self {
        return Self {
            name: String::from(""),

            bind_connection: None,
            clock_domain: Frequency::Unknown,
        }
    }
}