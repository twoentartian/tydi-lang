use std::collections::HashSet;
use std::sync::{Arc, RwLock};
use crate::data_type::DataType;
use crate::error::ErrorCode;
use crate::generate_get;
use crate::inferable::Inferable;
use crate::logical_data_type::LogicalDataType;
use crate::port::PortDirection;
use crate::scope::{Scope, ScopeRelationType, ScopeType};
use crate::util::{generate_padding, PrettyPrint};
use crate::variable::Variable;

/// If Scope

#[derive(Clone, Debug)]
pub struct IfScope {
    name: String,
    if_exp: Arc<RwLock<Variable>>,
    scope: Arc<RwLock<Scope>>,
}

impl IfScope {
    generate_get!(name, String, get_name);
    generate_get!(if_exp, Arc<RwLock<Variable>>, get_if_exp);
    generate_get!(scope, Arc<RwLock<Scope>>, get_scope);

    pub fn new(name_: String, if_exp_: Arc<RwLock<Variable>>) -> Self {
        let scope_ = Arc::new(RwLock::new(Scope::new(format!("if_{}", name_.clone()), ScopeType::IfForScope)));
        {
            scope_.write().unwrap().set_self_ref(scope_.clone());
        }
        Self {
            name: name_,
            if_exp: if_exp_,
            scope: scope_,
        }
    }

}

impl From<IfScope> for String {
    fn from(if_scope: IfScope) -> Self {
        return format!("If({})", if_scope.get_name());
    }
}

impl PrettyPrint for IfScope {
    fn pretty_print(&self, depth: u32, verbose: bool) -> String {
        let mut output = String::new();

        //enter group
        output.push_str(&format!("{}If({}){{\n", generate_padding(depth), String::from((*self.if_exp.read().unwrap()).clone())));
        //enter scope
        output.push_str(&format!("{}", self.scope.read().unwrap().pretty_print(depth+1, verbose)));
        //leave group
        output.push_str(&format!("{}}}", generate_padding(depth)));

        return output;
    }
}

impl Scope {
    pub fn new_if_block(&mut self, name_: String, if_exp_: Arc<RwLock<Variable>>) -> Result<Arc<RwLock<Scope>>, ErrorCode> {
        if !(self.scope_type == ScopeType::ImplementScope || self.scope_type == ScopeType::IfForScope) { return Err(ErrorCode::ScopeNotAllowed(format!("not allowed to define if block outside of if/for/implement scope"))); }

        match self.streamlets.get(&name_) {
            None => {}
            Some(_) => { return Err(ErrorCode::IdRedefined(format!("if block {} already defined", name_.clone()))); }
        };

        let mut if_scope = IfScope::new(name_.clone(), if_exp_.clone());
        {
            let parent_scope = self.self_ref.clone().unwrap();
            if_scope.scope.write().unwrap().new_relationship_with_name(parent_scope.clone(), String::from("base"), ScopeRelationType::IfForScopeRela);
        }

        let scope_clone = if_scope.scope.clone();
        self.if_blocks.insert(name_.clone(), Arc::new(RwLock::new(if_scope)));
        return Ok(scope_clone);
    }
}

/// For Scope

#[derive(Clone, Debug)]
pub struct ForScope {
    name: String,
    for_var_exp: Arc<RwLock<Variable>>,
    for_array_exp: Arc<RwLock<Variable>>,
    scope: Arc<RwLock<Scope>>,
}

impl ForScope {
    generate_get!(name, String, get_name);
    generate_get!(for_var_exp, Arc<RwLock<Variable>>, get_var_exp);
    generate_get!(for_array_exp, Arc<RwLock<Variable>>, get_array_exp);
    generate_get!(scope, Arc<RwLock<Scope>>, get_scope);

    pub fn new(name_: String, for_var_exp_: Arc<RwLock<Variable>>, for_array_exp_: Arc<RwLock<Variable>>) -> Self {
        let scope_ = Arc::new(RwLock::new(Scope::new(format!("for_{}", name_.clone()), ScopeType::IfForScope)));
        {
            scope_.write().unwrap().set_self_ref(scope_.clone());
        }
        Self {
            name: name_,
            for_var_exp: for_var_exp_,
            for_array_exp: for_array_exp_,
            scope: scope_,
        }
    }

}

impl From<ForScope> for String {
    fn from(for_scope: ForScope) -> Self {
        return format!("For({})", for_scope.get_name());
    }
}

impl PrettyPrint for ForScope {
    fn pretty_print(&self, depth: u32, verbose: bool) -> String {
        let mut output = String::new();

        //enter group
        output.push_str(&format!("{}For({}){{\n", generate_padding(depth), format!("{} in {}", String::from((*self.for_var_exp.read().unwrap()).clone()), String::from((*self.for_array_exp.read().unwrap()).clone()))));
        //enter scope
        output.push_str(&format!("{}", self.scope.read().unwrap().pretty_print(depth+1, verbose)));
        //leave group
        output.push_str(&format!("{}}}", generate_padding(depth)));

        return output;
    }
}

impl Scope {
    pub fn new_for_block(&mut self, name_: String, for_var_exp_: Arc<RwLock<Variable>>, for_array_exp_: Arc<RwLock<Variable>>) -> Result<Arc<RwLock<Scope>>, ErrorCode> {
        if !(self.scope_type == ScopeType::ImplementScope || self.scope_type == ScopeType::IfForScope) { return Err(ErrorCode::ScopeNotAllowed(format!("not allowed to define for block outside of if/for/implement scope"))); }

        match self.streamlets.get(&name_) {
            None => {}
            Some(_) => { return Err(ErrorCode::IdRedefined(format!("for block {} already defined", name_.clone()))); }
        };

        let mut for_scope = ForScope::new(name_.clone(), for_var_exp_.clone(), for_array_exp_.clone());
        {
            let parent_scope = self.self_ref.clone().unwrap();
            for_scope.scope.write().unwrap().new_relationship_with_name(parent_scope.clone(), String::from("base"), ScopeRelationType::IfForScopeRela);
        }

        let scope_clone = for_scope.scope.clone();
        self.for_blocks.insert(name_.clone(), Arc::new(RwLock::new(for_scope)));
        return Ok(scope_clone);
    }
}