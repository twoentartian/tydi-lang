use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

use crate::circuit_connection::Connection;
use crate::circuit_frequency::SimulationClockDomain;
use crate::circuit_implement;
use crate::{generate_set, generate_get, generate_access};

#[derive(Clone, Debug, PartialEq)]
pub enum PortBindDirection {
    Unknown,
    Internal,
    External,
}

#[derive(Clone)]
pub struct Port {
    name: String,

    parent_implement: Option<Arc<RwLock<circuit_implement::Implement>>>,
    bind_external_connection: Option<Arc<RwLock<Connection>>>,
    bind_internal_connection: Option<Arc<RwLock<Connection>>>,
    clock_domain: SimulationClockDomain,
}

impl Port {
    generate_access!(name, String, get_name, set_name);
    generate_access!(parent_implement, Option<Arc<RwLock<circuit_implement::Implement>>>, get_parent_implement, set_parent_implement);
    generate_access!(bind_external_connection, Option<Arc<RwLock<Connection>>>, get_bind_external_connection, set_bind_external_connection);
    generate_access!(bind_internal_connection, Option<Arc<RwLock<Connection>>>, get_bind_internal_connection, set_bind_internal_connection);
    generate_access!(clock_domain, SimulationClockDomain, get_clock_domain, set_clock_domain);

    pub fn new() -> Self {
        return Self {
            name: String::from(""),

            parent_implement: None,
            bind_external_connection: None,
            bind_internal_connection: None,
            clock_domain: SimulationClockDomain::new(),
        }
    }
}

