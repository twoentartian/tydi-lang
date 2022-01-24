use std::sync::{Arc, RwLock};
use std::collections::HashSet;
use crate::scope::{DataType, ScopeRelationType, Scope};
use crate::util::{generate_padding, PrettyPrint};
use crate::{generate_get, generate_set, generate_access};
pub use crate::error::ErrorCode;
use crate::inferable::{Inferable, NewInferable};

#[derive(Clone, Debug, PartialEq)]
pub enum VariableValue {
    Unknown,
    Int(u32),
    Bool(bool),
    Float(f32),
    Str(String),
}

impl From<VariableValue> for String {
    fn from(v: VariableValue) -> Self {
        return match v {
            VariableValue::Unknown => { format!("Unknown") }
            VariableValue::Int(v) => { format!("{}", v) }
            VariableValue::Bool(v) => { format!("{}", v) }
            VariableValue::Float(v) => { format!("{}", v) }
            VariableValue::Str(v) => { format!("{}", v) }
        }
    }
}

impl PrettyPrint for VariableValue {
    fn pretty_print(&self, _: u32, _: bool) -> String {
        return String::from(self.clone());
    }
}

#[derive(Clone, Debug)]
pub struct Variable {
    name: String,
    var_type: Arc<RwLock<DataType>>,

    var_value: Inferable<VariableValue>,
}

impl Variable {
    generate_get!(name, String, get_name);
    generate_get!(var_type, Arc<RwLock<DataType>>, get_type);
    generate_access!(var_value, Inferable<VariableValue>, get_var_value, set_var_value);

    pub fn new(name_: String, type_: DataType, exp_: String) -> Self {
        Self {
            name: name_.clone(),
            var_type: Arc::new(RwLock::new(type_)),
            var_value: <Inferable<VariableValue> as NewInferable<VariableValue>>::_new(exp_.clone()),
        }
    }

    pub fn new_int(name_: String, v: u32) -> Self {
        Self {
            name: name_.clone(),
            var_type: Arc::new(RwLock::new(DataType::IntType)),
            var_value: <Inferable<VariableValue> as NewInferable<VariableValue>>::_new_inferred(name_.clone(), VariableValue::Int(v)),
        }
    }

    pub fn new_float(name_: String, v: f32) -> Self {
        Self {
            name: name_.clone(),
            var_type: Arc::new(RwLock::new(DataType::FloatType)),
            var_value: <Inferable<VariableValue> as NewInferable<VariableValue>>::_new_inferred(name_.clone(), VariableValue::Float(v)),
        }
    }

    pub fn new_bool(name_: String, v: bool) -> Self {
        Self {
            name: name_.clone(),
            var_type: Arc::new(RwLock::new(DataType::BoolType)),
            var_value: <Inferable<VariableValue> as NewInferable<VariableValue>>::_new_inferred(name_.clone(), VariableValue::Bool(v)),
        }
    }

    pub fn new_str(name_: String, v: String) -> Self {
        Self {
            name: name_.clone(),
            var_type: Arc::new(RwLock::new(DataType::StringType)),
            var_value: <Inferable<VariableValue> as NewInferable<VariableValue>>::_new_inferred(name_.clone(), VariableValue::Str(v)),
        }
    }

}

impl From<Variable> for String {
    fn from(v: Variable) -> Self {
        return String::from(v.var_value);
    }
}

impl PrettyPrint for Variable {
    fn pretty_print(&self, depth: u32, verbose: bool) -> String {
        return format!("{}{}:{}({})", generate_padding(depth), self.name.clone(), self.var_type.read().unwrap().pretty_print(depth, verbose), String::from(self.var_value.clone()) );
    }
}

impl Scope {
    pub fn new_variable(&mut self, name_: String, type_: DataType, exp_: String) -> Result<(), ErrorCode> {
        match self.vars.get(&name_) {
            None => {}
            Some(_) => { return Err(ErrorCode::IdRedefined(format!("variable {} already defined", name_))); }
        };
        self.vars.insert(name_.clone(), Arc::new(RwLock::new(Variable::new(name_.clone(), type_.clone(), exp_.clone()))));
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