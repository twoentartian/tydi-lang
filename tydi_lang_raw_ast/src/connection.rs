use std::collections::HashSet;
use std::sync::{Arc, RwLock};
use deep_clone::DeepClone;
use port::PortArray;
use streamlet::Streamlet;
use crate::error::ErrorCode;
use crate::{generate_get, generate_access, generate_set};
use crate::inferable::{Inferable};
use crate::port::Port;
use crate::scope::{Scope, ScopeRelationType, ScopeType};
use crate::util::{generate_padding, PrettyPrint, EnableDocument};
use derived_macro::EnableDocument;
use scope::HashMap;
use tydi_il::ToTydiIL;
use util::rename_id_to_il;
use crate::variable::Variable;

#[derive(Clone, Debug)]
pub enum PortOwner {
    UnknownPortOwner,
    SelfOwner,
    ExternalOwner(String, Option<Arc<RwLock<Streamlet>>>, Option<Arc<RwLock<Variable>>>),
}

impl DeepClone for PortOwner {
    fn deep_clone(&self) -> Self {
        match self.clone() {
            PortOwner::ExternalOwner(name, streamlet, var) => {
                //IMPORTANT Notice: we use clone on streamlet because we believe they have common parent streamlet and it won't change during compile time
                return PortOwner::ExternalOwner(name.deep_clone(), streamlet.clone(), var.deep_clone());
            }
            _ => { return self.clone() }
        };
    }
}

impl From<PortOwner> for String {
    fn from(owner: PortOwner) -> Self {
        return match owner {
            PortOwner::UnknownPortOwner => { String::from("UnknownPortOwner") }
            PortOwner::SelfOwner => { String::from("Self") }
            PortOwner::ExternalOwner(name,_, index) => {
                match index {
                    None => { format!("ExternalOwner({})", name.clone()) }
                    Some(index) => { format!("ExternalOwner({})[{}]", name.clone(), String::from((*index.read().unwrap()).clone())) }
                }
            }
        }
    }
}

#[derive(Clone, Debug, EnableDocument)]
pub struct Connection {
    name: String,

    lhs_port: Inferable<Arc<RwLock<Port>>>,
    lhs_port_owner: PortOwner,
    lhs_port_array_type: PortArray,

    rhs_port: Inferable<Arc<RwLock<Port>>>,
    rhs_port_owner: PortOwner,
    rhs_port_array_type: PortArray,
    delay: Arc<RwLock<Variable>>,

    docu: Option<String>,

    check_restrict_type_same: bool,
}

impl DeepClone for Connection {
    fn deep_clone(&self) -> Self {
        return Self {
            name: self.name.deep_clone(),

            lhs_port: self.lhs_port.deep_clone(),
            lhs_port_owner: self.lhs_port_owner.deep_clone(),
            lhs_port_array_type: self.lhs_port_array_type.deep_clone(),

            rhs_port: self.rhs_port.deep_clone(),
            rhs_port_owner: self.rhs_port_owner.deep_clone(),
            rhs_port_array_type: self.rhs_port_array_type.deep_clone(),
            delay: self.delay.deep_clone(),

            docu: self.docu.deep_clone(),

            check_restrict_type_same: self.check_restrict_type_same.clone(),
        }
    }
}

impl ToTydiIL for Connection {
    fn to_tydi_il(&self, _: &mut HashMap<String, (String, Vec<String>)>, depth: u32) -> String {
        let lhs_owner = match &self.lhs_port_owner {
            PortOwner::UnknownPortOwner => { unreachable!() }
            PortOwner::SelfOwner => { String::from("") }
            PortOwner::ExternalOwner(name, _, array_index) => {
                let owner_name = match array_index {
                    Some(array_index) => {
                        format!("{}@{}",name, String::from((*array_index.read().unwrap()).clone()))
                    }
                    None => {
                        format!("{}",name)
                    }
                };
                format!("{}.", crate::util::rename_id_to_il(owner_name))
            }
        };
        let rhs_owner = match &self.rhs_port_owner {
            PortOwner::UnknownPortOwner => { unreachable!() }
            PortOwner::SelfOwner => { String::from("") }
            PortOwner::ExternalOwner(name, _, array_index) => {
                let owner_name = match array_index {
                    Some(array_index) => {
                        format!("{}@{}",name, String::from((*array_index.read().unwrap()).clone()))
                    }
                    None => {
                        format!("{}",name)
                    }
                };
                format!("{}.", crate::util::rename_id_to_il(owner_name))
            }
        };

        let lhs_port_name = self.lhs_port.get_raw_value().read().unwrap().get_name();
        let rhs_port_name = self.rhs_port.get_raw_value().read().unwrap().get_name();

        // let lhs_port_name = match self.lhs_port_array_type.clone() {
        //     PortArray::UnknownPortArray => { unreachable!() }
        //     PortArray::SinglePort => { lhs_port_name }
        //     PortArray::ArrayPort(i) => { format!("{}@{}", lhs_port_name, String::from((*i.read().unwrap()).clone())) }
        // };
        // let rhs_port_name = match self.rhs_port_array_type.clone() {
        //     PortArray::UnknownPortArray => { unreachable!() }
        //     PortArray::SinglePort => { rhs_port_name }
        //     PortArray::ArrayPort(i) => { format!("{}@{}", rhs_port_name, String::from((*i.read().unwrap()).clone())) }
        // };

        let lhs_port_name = rename_id_to_il(lhs_port_name);
        let rhs_port_name = rename_id_to_il(rhs_port_name);

        return format!("{}{}{} -- {}{}", generate_padding(depth), lhs_owner, lhs_port_name, rhs_owner, rhs_port_name);
    }
}

impl Connection {
    generate_access!(name, String, get_name, set_name);

    generate_access!(lhs_port, Inferable<Arc<RwLock<Port>>>, get_lhs_port, set_lhs_port);
    generate_access!(lhs_port_owner, PortOwner, get_lhs_port_owner, set_lhs_port_owner);
    generate_access!(lhs_port_array_type, PortArray, get_lhs_port_array_type, set_lhs_port_array_type);

    generate_access!(rhs_port, Inferable<Arc<RwLock<Port>>>, get_rhs_port, set_rhs_port);
    generate_access!(rhs_port_owner, PortOwner, get_rhs_port_owner, set_rhs_port_owner);
    generate_access!(rhs_port_array_type, PortArray, get_rhs_port_array_type, set_rhs_port_array_type);

    generate_access!(delay, Arc<RwLock<Variable>>, get_delay, set_delay);
    generate_access!(check_restrict_type_same, bool, get_check_restrict_type_same, set_check_restrict_type_same);

    pub fn new(name_: String, lhs_port_: Inferable<Arc<RwLock<Port>>>, rhs_port_: Inferable<Arc<RwLock<Port>>>, delay_: Variable) -> Self {
        Self {
            name: name_.clone(),
            lhs_port: lhs_port_.clone(),
            lhs_port_owner: PortOwner::UnknownPortOwner,
            lhs_port_array_type: PortArray::UnknownPortArray,
            rhs_port: rhs_port_.clone(),
            rhs_port_owner: PortOwner::UnknownPortOwner,
            rhs_port_array_type: PortArray::UnknownPortArray,
            delay: Arc::new(RwLock::new(delay_.clone())),
            docu: None,
            check_restrict_type_same: true,
        }
    }
}

impl From<Connection> for String {
    fn from(conn: Connection) -> Self {
        return format!("{}.{}{} ={}=> {}.{}{} ({}) {}", String::from(conn.lhs_port_owner.clone()), String::from(conn.lhs_port.clone()), String::from(conn.lhs_port_array_type.clone()),
                       String::from((*conn.delay.read().unwrap()).clone()),
                       String::from(conn.rhs_port_owner.clone()), String::from(conn.rhs_port.clone()), String::from(conn.rhs_port_array_type.clone()),
                       conn.name.clone(), if !conn.get_check_restrict_type_same() { String::from("@NoStrictType") } else { String::from("") });
    }
}

impl PrettyPrint for Connection {
    fn pretty_print(&self, depth: u32, _: bool) -> String {
        return format!("{}{}", generate_padding(depth), String::from(self.clone()));
    }
}

impl Scope {
    pub fn new_connection(&mut self, name_: String, lhs_port_: Inferable<Arc<RwLock<Port>>>, rhs_port_: Inferable<Arc<RwLock<Port>>>, delay_: Variable) -> Result<(), ErrorCode> {
        if !(self.scope_type == ScopeType::ImplementScope || self.scope_type == ScopeType::IfForScope) { return Err(ErrorCode::ScopeNotAllowed(String::from("not allowed to define connections outside of implement scope"))); }

        match self.connections.get(&name_) {
            None => {}
            Some(_) => { return Err(ErrorCode::IdRedefined(format!("connection {} already defined", name_))); }
        };
        self.connections.insert(name_.clone(), Arc::new(RwLock::new(Connection::new(name_.clone(), lhs_port_.clone(), rhs_port_.clone(), delay_.clone()))));
        return Ok(());
    }

    pub fn with_connection(&mut self, connection: Arc<RwLock<Connection>>) -> Result<(), ErrorCode> {
        if !(self.scope_type == ScopeType::ImplementScope || self.scope_type == ScopeType::IfForScope) { return Err(ErrorCode::ScopeNotAllowed(String::from("not allowed to define connections outside of implement scope"))); }

        let name_ = connection.read().unwrap().get_name();
        match self.connections.get(&name_) {
            None => {}
            Some(_) => { return Err(ErrorCode::IdRedefined(format!("connection {} already defined", name_))); }
        };
        self.connections.insert(name_.clone(), connection.clone());
        return Ok(());
    }

    pub fn resolve_connection_in_current_scope(& self, name_: String) -> Result<Arc<RwLock<Connection>>, ErrorCode> {
        return match self.connections.get(&name_) {
            None => { Err(ErrorCode::IdNotFound(format!("connection {} not found", name_))) }
            Some(var) => { Ok(var.clone()) }
        };
    }

    fn _resolve_connection_in_scope(target_scope: Arc<RwLock<Scope>>, name_: &String, allowed_relationships: &HashSet<ScopeRelationType>) -> Result<Arc<RwLock<Connection>>, ErrorCode> {
        let target_scope_r = target_scope.read().unwrap();

        //find self scope
        match target_scope_r.resolve_connection_in_current_scope(name_.clone()) {
            Ok(var) => { return Ok(var) }
            Err(_) => {}
        }

        //find in parent scope
        for (_, scope_real) in &(target_scope_r.scope_relationships) {
            if !allowed_relationships.contains(&scope_real.get_relationship()) { continue }
            let result = Scope::_resolve_connection_in_scope(scope_real.get_target_scope().clone(), &name_, &allowed_relationships);
            match result {
                Ok(inst) => {return Ok(inst)}
                Err(_) => {}
            }
        }

        return Err(ErrorCode::IdNotFound(format!("connection {} not found", name_.clone())));
    }

    pub fn resolve_connection_with_relation(& self, name_: String, allowed_relationships: Vec<ScopeRelationType>) -> Result<Arc<RwLock<Connection>>, ErrorCode> {
        match self.resolve_connection_in_current_scope(name_.clone()) {
            Ok(inst) => { return Ok(inst) }
            Err(_) => {}
        }

        let mut allowed_relationships_hash = HashSet::new();
        for allowed_relationship in allowed_relationships {
            allowed_relationships_hash.insert(allowed_relationship.clone());
        }

        //find in parent scope
        for (_, scope_real) in &(self.scope_relationships) {
            if !allowed_relationships_hash.contains(&scope_real.get_relationship()) { continue }
            let result = Scope::_resolve_connection_in_scope(scope_real.get_target_scope().clone(), &name_, & allowed_relationships_hash);
            match result {
                Ok(var) => {return Ok(var)}
                Err(_) => {}
            }
        }

        return Err(ErrorCode::IdNotFound(format!("connection {} not found", name_.clone())));
    }

    pub fn resolve_connection_from_scope(& self, name_: String) -> Result<Arc<RwLock<Connection>>, ErrorCode> {
        use crate::scope::ScopeRelationType::*;
        let allowed_relationships = vec![];
        return self.resolve_connection_with_relation(name_, allowed_relationships);
    }
}