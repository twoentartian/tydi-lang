use std::collections::HashSet;
use std::sync::{Arc, RwLock};
use crate::error::ErrorCode;
use crate::streamlet::Streamlet;
use crate::generate_get;
use crate::inferable::{Inferable, InferState, NewInferable};
use crate::port::Port;
use crate::scope::{Scope, ScopeRelationType, ScopeType};
use crate::util::{generate_padding, PrettyPrint};
use crate::variable::Variable;

#[derive(Clone, Debug)]
pub struct Connection {
    name: String,

    lhs_port: Inferable<Arc<RwLock<Port>>>,
    rhs_port: Inferable<Arc<RwLock<Port>>>,
    delay: Arc<RwLock<Variable>>,
}

impl Connection {
    generate_get!(name, String, get_name);
    generate_get!(lhs_port, Inferable<Arc<RwLock<Port>>>, get_lhs_port);
    generate_get!(rhs_port, Inferable<Arc<RwLock<Port>>>, get_rhs_port);
    generate_get!(delay, Arc<RwLock<Variable>>, get_delay);

    pub fn new(name_: String, lhs_port_: Inferable<Arc<RwLock<Port>>>, rhs_port_: Inferable<Arc<RwLock<Port>>>, delay_: Variable) -> Self {
        Self {
            name: name_.clone(),
            lhs_port: lhs_port_.clone(),
            rhs_port: rhs_port_.clone(),
            delay: Arc::new(RwLock::new(delay_.clone())),
        }
    }
}

impl From<Connection> for String {
    fn from(conn: Connection) -> Self {
        return format!("{}={}=>{} ({})", String::from(conn.lhs_port.clone()), String::from((*conn.delay.read().unwrap()).clone()), String::from(conn.rhs_port.clone()), conn.name.clone());
    }
}

impl PrettyPrint for Connection {
    fn pretty_print(&self, depth: u32, verbose: bool) -> String {
        return format!("{}{}", generate_padding(depth), String::from(self.clone()));
    }
}

impl Scope {
    pub fn new_connection(&mut self, name_: String, lhs_port_: Inferable<Arc<RwLock<Port>>>, rhs_port_: Inferable<Arc<RwLock<Port>>>, delay_: Variable) -> Result<(), ErrorCode> {
        if self.scope_type != ScopeType::ImplementScope { return Err(ErrorCode::ScopeNotAllowed(String::from("not allowed to define connections outside of implement scope"))); }

        match self.connections.get(&name_) {
            None => {}
            Some(_) => { return Err(ErrorCode::IdRedefined(format!("connection {} already defined", name_))); }
        };
        self.connections.insert(name_.clone(), Arc::new(RwLock::new(Connection::new(name_.clone(), lhs_port_.clone(), rhs_port_.clone(), delay_.clone()))));
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
        let allowed_relationships = vec![GroupScopeRela, UnionScopeRela,
                                         StreamScopeRela, StreamletScopeRela, ImplementScopeRela];
        return self.resolve_connection_with_relation(name_, allowed_relationships);
    }
}