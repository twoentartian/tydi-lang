use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};
use deep_clone::DeepClone;
use variable::Variable;
use crate::logical_data_type::LogicalDataType;
use crate::util::{generate_padding, PrettyPrint, EnableDocument};
use crate::{generate_access, generate_get, generate_set};
use crate::error::ErrorCode;
use crate::inferable::{Inferable};
use crate::scope::{Scope, ScopeRelationType, ScopeType};
use derived_macro::EnableDocument;
use crate::tydi_il;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PortDirection {
    Input,
    Output,
    Unknown,
}

impl DeepClone for PortDirection {
    fn deep_clone(&self) -> Self {
        return self.clone();
    }
}

impl From<PortDirection> for String {
    fn from(dir: PortDirection) -> Self {
        return match dir {
            PortDirection::Input => { String::from("in") }
            PortDirection::Output => { String::from("out") }
            PortDirection::Unknown => { String::from("unknown") }
        }
    }
}

#[derive(Clone, Debug)]
pub enum PortArray {
    UnknownPortArray,
    SinglePort,
    ArrayPort(Arc<RwLock<Variable>>),
}

impl DeepClone for PortArray {
    fn deep_clone(&self) -> Self {
        return match self {
            PortArray::ArrayPort(var) => PortArray::ArrayPort(var.deep_clone()),
            _ => self.clone(),
        }
    }
}

impl From<PortArray> for String {
    fn from(arr: PortArray) -> Self {
        return match arr {
            PortArray::UnknownPortArray => { String::from("UnknownPortArray") }
            PortArray::SinglePort => { String::from("") }
            PortArray::ArrayPort(p) => { format!("[{}]", String::from((*p.read().unwrap()).clone())) }
        }
    }
}

#[derive(Clone, Debug, EnableDocument)]
pub struct Port {
    name: String,
    port_type: Inferable<Arc<RwLock<LogicalDataType>>>,
    direction: PortDirection,
    array_type: PortArray,
    docu: Option<String>,
}

impl DeepClone for Port {
    fn deep_clone(&self) -> Self {
        return Self {
            name: self.name.deep_clone(),
            port_type: self.port_type.deep_clone(),
            direction: self.direction.deep_clone(),
            array_type: self.array_type.deep_clone(),
            docu: self.docu.deep_clone(),
        }
    }
}

impl tydi_il::ToTydiIL for Port {
    fn to_tydi_il(&self, type_alias_map: &mut HashMap<String, String>) -> String {
        let mut output = String::from("");

        //document
        match &self.docu {
            None => {}
            Some(docu) => {
                output.push_str(&format!("{}\n", docu));
            }
        }

        output.push_str(&format!("{}: {} {}",
                                 self.name.clone(),
                                 match self.direction {
                                     PortDirection::Input => "in",
                                     PortDirection::Output => "out",
                                     PortDirection::Unknown => unreachable!(),
                                 },
                                 self.port_type.get_raw_value().read().unwrap().to_tydi_il(type_alias_map)));
        return output;
    }
}

impl Port {
    generate_access!(name, String, get_name, set_name);
    generate_access!(port_type, Inferable<Arc<RwLock<LogicalDataType>>>, get_type, set_type);
    generate_get!(direction, PortDirection, get_direction);
    generate_access!(array_type, PortArray, get_array_type, set_array_type);

    pub fn new(name_: String, type_exp: Inferable<Arc<RwLock<LogicalDataType>>>, direction_: PortDirection) -> Self {
        Self {
            name: name_.clone(),
            port_type: type_exp.clone(),
            direction: direction_,
            array_type: PortArray::SinglePort,
            docu: None,
        }
    }

    pub fn new_array(name_: String, type_exp: Inferable<Arc<RwLock<LogicalDataType>>>, direction_: PortDirection, array_: Arc<RwLock<Variable>>) -> Self {
        Self {
            name: name_.clone(),
            port_type: type_exp.clone(),
            direction: direction_,
            array_type: PortArray::ArrayPort(array_.clone()),
            docu: None,
        }
    }
}

impl From<Port> for String {
    fn from(port: Port) -> Self {
        match port.clone().array_type {
            PortArray::UnknownPortArray => {
                return format!("{}:UnknownPort({},{})", port.get_name(), String::from(port.get_type().clone()), String::from(port.direction.clone()) );
            },
            PortArray::SinglePort => {
                return format!("{}:Port({},{})", port.get_name(), String::from(port.get_type().clone()), String::from(port.direction.clone()) );
            },
            PortArray::ArrayPort(array) => {
                return format!("{}:PortArray[{}]({},{})", port.get_name(), String::from((*array.read().unwrap()).clone()), String::from(port.get_type().clone()), String::from(port.direction.clone()) );
            },
        }
    }
}

impl PrettyPrint for Port {
    fn pretty_print(&self, depth: u32, _: bool) -> String {
        return format!("{}{}", generate_padding(depth), String::from(self.clone()));
    }
}

impl Scope {
    pub fn new_port(&mut self, name_: String, type_: Inferable<Arc<RwLock<LogicalDataType>>>, dir: PortDirection) -> Result<(), ErrorCode> {
        if self.scope_type != ScopeType::StreamletScope { return Err(ErrorCode::ScopeNotAllowed(String::from("port is only allowed to define in streamlet"))); }

        match self.ports.get(&name_) {
            None => {}
            Some(_) => { return Err(ErrorCode::IdRedefined(format!("port {} already defined", name_.clone()))); }
        };
        let port = Port::new(name_.clone(), type_.clone(), dir.clone());
        self.ports.insert(name_.clone(), Arc::new(RwLock::new(port)));

        return Ok(());
    }

    pub fn with_port(&mut self, port: Arc<RwLock<Port>>) -> Result<(), ErrorCode> {
        if self.scope_type != ScopeType::StreamletScope { return Err(ErrorCode::ScopeNotAllowed(String::from("port is only allowed to define in streamlet"))); }

        let name_ = port.read().unwrap().get_name();
        match self.ports.get(&name_) {
            None => {}
            Some(_) => { return Err(ErrorCode::IdRedefined(format!("port {} already defined", name_.clone()))); }
        };

        self.ports.insert(name_.clone(), port.clone());

        return Ok(());
    }

    pub fn new_port_array(&mut self, name_: String, type_: Inferable<Arc<RwLock<LogicalDataType>>>, dir: PortDirection, array_: Arc<RwLock<Variable>>) -> Result<(), ErrorCode> {
        if self.scope_type != ScopeType::StreamletScope { return Err(ErrorCode::ScopeNotAllowed(String::from("port is only allowed to define in streamlet"))); }

        match self.ports.get(&name_) {
            None => {}
            Some(_) => { return Err(ErrorCode::IdRedefined(format!("port {} already defined", name_.clone()))); }
        };
        let port = Port::new_array(name_.clone(), type_.clone(), dir.clone(), array_.clone());
        self.ports.insert(name_.clone(), Arc::new(RwLock::new(port)));

        return Ok(());
    }

    pub fn resolve_port_in_current_scope(& self, name_: String) -> Result<Arc<RwLock<Port>>, ErrorCode> {
        return match self.ports.get(&name_) {
            None => { Err(ErrorCode::IdNotFound(format!("port {} not found", name_))) }
            Some(port) => { Ok(port.clone()) }
        };
    }

    fn _resolve_port_in_scope(target_scope: Arc<RwLock<Scope>>, name_: &String, allowed_relationships: &HashSet<ScopeRelationType>) -> Result<Arc<RwLock<Port>>, ErrorCode> {
        let target_scope_r = target_scope.read().unwrap();

        //find self scope
        match target_scope_r.resolve_port_in_current_scope(name_.clone()) {
            Ok(var) => { return Ok(var) }
            Err(_) => {}
        }

        //find in parent scope
        for (_, scope_real) in &(target_scope_r.scope_relationships) {
            let result = Scope::_resolve_port_in_scope(scope_real.get_target_scope().clone(), &name_, &allowed_relationships);
            match result {
                Ok(inst) => {return Ok(inst)}
                Err(_) => {}
            }
        }

        return Err(ErrorCode::IdNotFound(format!("port {} not found", name_.clone())));
    }

    pub fn resolve_port_with_relation(& self, name_: String, allowed_relationships: Vec<ScopeRelationType>) -> Result<Arc<RwLock<Port>>, ErrorCode> {
        match self.resolve_port_in_current_scope(name_.clone()) {
            Ok(inst) => { return Ok(inst) }
            Err(_) => {}
        }

        let mut allowed_relationships_hash = HashSet::new();
        for allowed_relationship in allowed_relationships {
            allowed_relationships_hash.insert(allowed_relationship.clone());
        }

        //find in parent scope
        for (_, scope_real) in &(self.scope_relationships) {
            let result = Scope::_resolve_port_in_scope(scope_real.get_target_scope().clone(), &name_, & allowed_relationships_hash);
            match result {
                Ok(var) => {return Ok(var)}
                Err(_) => {}
            }
        }

        return Err(ErrorCode::IdNotFound(format!("port {} not found", name_.clone())));
    }

    pub fn resolve_port_from_scope(& self, name_: String) -> Result<Arc<RwLock<Port>>, ErrorCode> {
        use crate::scope::ScopeRelationType::*;
        let allowed_relationships = vec![GroupScopeRela, UnionScopeRela,
                                         StreamScopeRela, StreamletScopeRela, ImplementScopeRela];
        return self.resolve_port_with_relation(name_, allowed_relationships);
    }

}