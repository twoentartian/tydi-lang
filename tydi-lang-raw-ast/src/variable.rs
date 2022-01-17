use std::sync::{Arc, RwLock};
use std::collections::HashSet;
use crate::scope::{DataType, ScopeRelationType, Scope, ScopeType};
use crate::util::{generate_padding, PrettyPrint};

pub use crate::error::ErrorCode;

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
    fn pretty_print(&self, depth: u32, verbose: bool) -> String {
        return format!("{}{}:{}", generate_padding(depth), self.name.clone(), self.var_type.read().unwrap().pretty_print(depth, verbose) );
    }
}

impl Scope {
    pub fn new_variable(&mut self, name_: String, type_: DataType) -> Result<(), ErrorCode> {
        match self.vars.get(&name_) {
            None => {}
            Some(_) => { return Err(ErrorCode::IdRedefined(format!("variable {} already defined", name_))); }
        };
        self.vars.insert(name_.clone(), Arc::new(RwLock::new(Variable::new(name_.clone(), type_.clone()))));
        return Ok(());
    }

    pub fn resolve_variable_in_current_scope(& self, name_: String) -> Result<Arc<RwLock<Variable>>, ErrorCode> {
        return match self.vars.get(&name_) {
            None => { Err(ErrorCode::IdNotFound(format!("variable {} not found", name_))) }
            Some(var) => { Ok(var.clone()) }
        };
    }

    fn _resolve_var_in_scope(target_scope: Arc<RwLock<Scope>>, name_: &String, allowed_relationships: &HashSet<ScopeRelationType>) -> Result<Arc<RwLock<Variable>>, ErrorCode> {
        let target_scope_r = target_scope.read().unwrap();

        //find self scope
        match target_scope_r.resolve_variable_in_current_scope(name_.clone()) {
            Ok(var) => { return Ok(var) }
            Err(_) => {}
        }

        //find in parent scope
        for (_, scope_real) in &(target_scope_r.scope_relationships) {
            let result = Scope::_resolve_var_in_scope(scope_real.get_target_scope().clone(), &name_, &allowed_relationships);
            match result {
                Ok(var) => {return Ok(var)}
                Err(_) => {}
            }
        }

        return Err(ErrorCode::IdNotFound(format!("variable {} not found", name_.clone())));
    }

    pub fn resolve_variable_with_relation(& self, name_: String, allowed_relationships: Vec<ScopeRelationType>) -> Result<Arc<RwLock<Variable>>, ErrorCode> {
        match self.resolve_variable_in_current_scope(name_.clone()) {
            Ok(var) => { return Ok(var) }
            Err(_) => {}
        }

        let mut allowed_relationships_hash = HashSet::new();
        for allowed_relationship in allowed_relationships {
            allowed_relationships_hash.insert(allowed_relationship.clone());
        }

        //find in parent scope
        for (_, scope_real) in &(self.scope_relationships) {
            let result = Scope::_resolve_var_in_scope(scope_real.get_target_scope().clone(), &name_, & allowed_relationships_hash);
            match result {
                Ok(var) => {return Ok(var)}
                Err(_) => {}
            }
        }

        return Err(ErrorCode::IdNotFound(format!("variable {} not found", name_.clone())));
    }

    pub fn resolve_variable_from_scope(& self, name_: String) -> Result<Arc<RwLock<Variable>>, ErrorCode> {
        use crate::scope::ScopeRelationType::*;
        let allowed_relationships = vec![GroupScopeRela, UnionScopeRela,
                                         StreamScopeRela, StreamletScopeRela, ImplementScopeRela];
        return self.resolve_variable_with_relation(name_, allowed_relationships);
    }
}