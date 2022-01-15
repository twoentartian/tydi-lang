use std::sync::{Arc, RwLock};
use crate::scope::{DataType, ScopeRelationship, Scope};
use crate::util::{generate_padding, PrettyPrint};

pub use crate::error::ErrorCode;
pub use crate::error::ErrorCode::*;

#[derive(Clone, Debug)]
pub struct Variable {
    pub name: String,
    pub var_type: Arc<RwLock<DataType>>,
}

impl Variable {
    pub fn new(name_: String, type_: DataType) -> Self {
        Self {
            name: name_,
            var_type: Arc::new(RwLock::new(type_)),
        }
    }
}

impl PrettyPrint for Variable {
    fn pretty_print(&self, depth: u32) -> String {
        return format!("{}{}:{}", generate_padding(depth), self.name.clone(), self.var_type.read().unwrap().pretty_print(depth) );
    }
}

impl Scope {
    pub fn new_variable(&mut self, name_: String, type_: DataType) -> Result<(), ErrorCode> {
        match self.vars.get(&name_) {
            None => {}
            Some(_) => { return Err(IdRedefined(format!("variable {} already defined", name_))); }
        };
        self.vars.insert(name_.clone(), Arc::new(RwLock::new(Variable::new(name_.clone(), type_.clone()))));
        return Ok(());
    }

    pub fn resolve_variable(& self, name_: String) -> Result<Arc<RwLock<Variable>>, ErrorCode> {
        return match self.vars.get(&name_) {
            None => { Err(IdNotFound(format!("variable {} not found", name_))) }
            Some(var) => { Ok(var.clone()) }
        };
    }

    pub fn resolve_variable_in_scopes(& self, name_: String, allowed_relationships: Vec<ScopeRelationship>) -> Result<Arc<RwLock<Variable>>, ErrorCode> {
        match self.resolve_variable(name_.clone()) {
            Ok(var) => { return Ok(var) }
            Err(_) => {}
        }

        todo!();

        return Err(IdNotFound(format!("variable {} not found", name_.clone())));
    }
}