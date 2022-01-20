use std::sync::{Arc, RwLock};
use crate::logical_data_type::LogicalDataType;
use crate::util::{generate_padding, PrettyPrint};
use crate::{generate_access, generate_get, generate_set};
use crate::error::ErrorCode;
use crate::inferable::{Inferable, NewInferable};
use crate::scope::{Scope, ScopeType};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PortDirection {
    Input,
    Output,
}

impl From<PortDirection> for String {
    fn from(dir: PortDirection) -> Self {
        return match dir {
            PortDirection::Input => { String::from("in") }
            PortDirection::Output => { String::from("out") }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Port {
    name: String,
    port_type: Inferable<Arc<RwLock<LogicalDataType>>>,
    direction: PortDirection,
}

impl Port {
    generate_get!(name, String, get_name);
    generate_access!(port_type, Inferable<Arc<RwLock<LogicalDataType>>>, get_type, set_type);
    generate_get!(direction, PortDirection, get_direction);

    pub fn new(name_: String, type_exp: Inferable<Arc<RwLock<LogicalDataType>>>, direction_: PortDirection) -> Self {
        Self {
            name: name_.clone(),
            port_type: type_exp.clone(),
            direction: direction_,
        }
    }
}

impl From<Port> for String {
    fn from(port: Port) -> Self {
        return format!("{}:Port({},{})", port.get_name(), String::from(port.get_type().clone()), String::from(port.direction.clone()) );
    }
}

impl PrettyPrint for Port {
    fn pretty_print(&self, depth: u32, verbose: bool) -> String {
        return format!("{}{}", generate_padding(depth), String::from(self.clone()));
    }
}

impl Scope {
    pub fn new_port(&mut self, name_: String, type_: Inferable<Arc<RwLock<LogicalDataType>>>, dir: PortDirection) -> Result<(), ErrorCode> {
        if self.scope_type != ScopeType::StreamletScope { return Err(ErrorCode::ScopeNotAllowed(String::from("port is only allowed to define in streamlet"))); }

        match self.types.get(&name_) {
            None => {}
            Some(_) => { return Err(ErrorCode::IdRedefined(format!("type {} already defined", name_.clone()))); }
        };
        let port = Port::new(name_.clone(), type_.clone(), dir.clone());
        self.ports.insert(name_.clone(), Arc::new(RwLock::new(port)));

        return Ok(());
    }
}