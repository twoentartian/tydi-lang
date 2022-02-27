use std::sync::{Arc, RwLock};
use data_type::DataType;
use deep_clone::DeepClone;

use crate::error::ErrorCode;
use crate::{generate_get, generate_access, generate_set};
use crate::scope::{Scope, ScopeRelationType, ScopeType};
use crate::util::{generate_padding, PrettyPrint};
use crate::variable::Variable;

/// If Scope

#[derive(Clone, Debug)]
pub struct ElifScope {
    name: String,
    elif_exp: Arc<RwLock<Variable>>,
    scope: Arc<RwLock<Scope>>,
}

impl DeepClone for ElifScope {
    fn deep_clone(&self) -> Self {
        let output = Self {
            name: self.name.deep_clone(),
            elif_exp: self.elif_exp.deep_clone(),
            scope: self.scope.deep_clone(),
        };
        {
            output.scope.write().unwrap().set_self_ref(output.scope.clone());
        }
        return output;
    }
}

impl ElifScope {
    generate_access!(name, String, get_name, set_name);
    generate_access!(elif_exp, Arc<RwLock<Variable>>, get_elif_exp, set_elif_exp);
    generate_access!(scope, Arc<RwLock<Scope>>, get_scope, set_scope);

    pub fn new(name_: String) -> Self {
        let scope_ = Arc::new(RwLock::new(Scope::new(format!("elif_{}", name_.clone()), ScopeType::IfForScope)));
        {
            scope_.write().unwrap().set_self_ref(scope_.clone());
        }
        return Self{
            name: name_.clone(),
            elif_exp: Arc::new(RwLock::new(Variable::new(String::from(""), DataType::UnknownType, String::from("")))),
            scope: scope_,
        }
    }
}

impl PrettyPrint for ElifScope {
    fn pretty_print(&self, depth: u32, verbose: bool) -> String {
        let mut output = String::new();

        // elif
        output.push_str(&format!("{}Elif({}){{\n", generate_padding(depth), String::from((*self.elif_exp.read().unwrap()).clone())));
        output.push_str(&format!("{}", self.scope.read().unwrap().pretty_print(depth+1, verbose)));
        output.push_str(&format!("{}}}", generate_padding(depth)));

        return output;
    }
}

#[derive(Clone, Debug)]
pub struct ElseScope {
    name: String,
    scope: Arc<RwLock<Scope>>,
}

impl DeepClone for ElseScope {
    fn deep_clone(&self) -> Self {
        let output = Self {
            name: self.name.deep_clone(),
            scope: self.scope.deep_clone(),
        };
        {
            output.scope.write().unwrap().set_self_ref(output.scope.clone());
        }
        return output;
    }
}

impl PrettyPrint for ElseScope {
    fn pretty_print(&self, depth: u32, verbose: bool) -> String {
        let mut output = String::new();

        // elif
        output.push_str(&format!("{}Else{{\n", generate_padding(depth)));
        output.push_str(&format!("{}", self.scope.read().unwrap().pretty_print(depth+1, verbose)));
        output.push_str(&format!("{}}}", generate_padding(depth)));

        return output;
    }
}

impl ElseScope {
    generate_access!(name, String, get_name, set_name);
    generate_access!(scope, Arc<RwLock<Scope>>, get_scope, set_scope);

    pub fn new(name_: String) -> Self {
        let scope_ = Arc::new(RwLock::new(Scope::new(format!("else_{}", name_.clone()), ScopeType::IfForScope)));
        {
            scope_.write().unwrap().set_self_ref(scope_.clone());
        }
        return Self{
            name: name_.clone(),
            scope: scope_,
        }
    }
}

#[derive(Clone, Debug)]
pub struct IfScope {
    name: String,
    if_exp: Arc<RwLock<Variable>>,
    scope: Arc<RwLock<Scope>>,
    elif_elements: Vec<ElifScope>,
    else_element: Option<ElseScope>,
}

impl DeepClone for IfScope {
    fn deep_clone(&self) -> Self {
        let output = Self {
            name: self.name.deep_clone(),
            if_exp: self.if_exp.deep_clone(),
            scope: self.scope.deep_clone(),
            elif_elements: self.elif_elements.deep_clone(),
            else_element: self.else_element.deep_clone(),
        };
        {
            let mut output_write = output.scope.write().unwrap();
            output_write.set_self_ref(output.scope.clone());
        }
        return output;
    }
}

impl IfScope {
    generate_get!(name, String, get_name);
    generate_access!(if_exp, Arc<RwLock<Variable>>, get_if_exp, set_if_exp);
    generate_access!(scope, Arc<RwLock<Scope>>, get_scope, set_scope);
    generate_access!(elif_elements, Vec<ElifScope>, get_elif, set_elif);
    generate_access!(else_element, Option<ElseScope>, get_else, set_else);

    pub fn new(name_: String, if_exp_: Arc<RwLock<Variable>>) -> Self {
        let scope_ = Arc::new(RwLock::new(Scope::new(format!("if_{}", name_.clone()), ScopeType::IfForScope)));
        {
            scope_.write().unwrap().set_self_ref(scope_.clone());
        }
        Self {
            name: name_,
            if_exp: if_exp_,
            scope: scope_,
            elif_elements: vec![],
            else_element: None,
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

        // if
        output.push_str(&format!("{}If({}){{\n", generate_padding(depth), String::from((*self.if_exp.read().unwrap()).clone())));
        output.push_str(&format!("{}", self.scope.read().unwrap().pretty_print(depth+1, verbose)));

        // elif
        for elif_element in self.elif_elements.clone() {
            output.push_str(&format!("{}\n", elif_element.pretty_print(depth+1, verbose)));
        }

        // else
        match self.else_element.clone() {
            Some(else_element) => {
                output.push_str(&format!("{}\n", else_element.pretty_print(depth+1, verbose)));
            }
            None => {}
        }

        output.push_str(&format!("{}}}", generate_padding(depth)));

        return output;
    }
}

impl Scope {
    pub fn new_if_block(&mut self, name_: String, if_exp_: Arc<RwLock<Variable>>, parent_scope_name: String) -> Result<Arc<RwLock<Scope>>, ErrorCode> {
        if !(self.scope_type == ScopeType::ImplementScope || self.scope_type == ScopeType::IfForScope) { return Err(ErrorCode::ScopeNotAllowed(format!("not allowed to define if block outside of if/for/implement scope"))); }

        match self.if_blocks.get(&name_) {
            None => {}
            Some(_) => { return Err(ErrorCode::IdRedefined(format!("if block {} already defined", name_.clone()))); }
        };

        let if_scope = IfScope::new(name_.clone(), if_exp_.clone());
        {
            let parent_scope = self.self_ref.clone().unwrap();
            if_scope.scope.write().unwrap().new_relationship_with_name(parent_scope.clone(), parent_scope_name.clone(), ScopeRelationType::IfForScopeRela);
        }

        let scope_clone = if_scope.scope.clone();
        self.if_blocks.insert(name_.clone(), Arc::new(RwLock::new(if_scope)));
        return Ok(scope_clone);
    }

    pub fn with_if_block(&mut self, target: Arc<RwLock<IfScope>>, parent_scope_name: String) -> Result<(), ErrorCode> {
        if !(self.scope_type == ScopeType::ImplementScope || self.scope_type == ScopeType::IfForScope) { return Err(ErrorCode::ScopeNotAllowed(format!("not allowed to define if block outside of if/for/implement scope"))); }
        let name_ = target.read().unwrap().get_name();
        match self.if_blocks.get(&name_) {
            None => {}
            Some(_) => { return Err(ErrorCode::IdRedefined(format!("if block {} already defined", name_.clone()))); }
        };

        // update scope
        let parent_scope = self.self_ref.clone().unwrap();
        {
            let if_scope = target.read().unwrap().get_scope();
            if_scope.write().unwrap().new_relationship_with_name(parent_scope.clone(), parent_scope_name.clone(), ScopeRelationType::IfForScopeRela);
        }
        {
            let elifs = target.read().unwrap().get_elif();
            for elif in elifs {
                elif.get_scope().write().unwrap().new_relationship_with_name(parent_scope.clone(), parent_scope_name.clone(), ScopeRelationType::IfForScopeRela);
            }
        }
        {
            let else_scope = target.read().unwrap().get_else();
            match else_scope {
                None => {},
                Some(else_scope) => {
                    else_scope.get_scope().write().unwrap().new_relationship_with_name(parent_scope.clone(), parent_scope_name.clone(), ScopeRelationType::IfForScopeRela);
                }
            }
        }

        self.if_blocks.insert(name_.clone(), target);
        return Ok(());
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

impl DeepClone for ForScope {
    fn deep_clone(&self) -> Self {
        let output = Self {
            name: self.name.deep_clone(),
            for_var_exp: self.for_var_exp.deep_clone(),
            for_array_exp: self.for_array_exp.deep_clone(),
            scope: self.scope.deep_clone(),
        };
        {
            output.scope.write().unwrap().set_self_ref(output.scope.clone());
        }
        return output;
    }
}

impl ForScope {
    generate_get!(name, String, get_name);
    generate_access!(for_var_exp, Arc<RwLock<Variable>>, get_var_exp, set_var_exp);
    generate_access!(for_array_exp, Arc<RwLock<Variable>>, get_array_exp, set_array_exp);
    generate_get!(scope, Arc<RwLock<Scope>>, get_scope);

    pub fn new(name_: String, for_var_exp_: Arc<RwLock<Variable>>, for_array_exp_: Arc<RwLock<Variable>>) -> Self {
        let scope_ = Arc::new(RwLock::new(Scope::new(name_.clone(), ScopeType::IfForScope)));
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

        match self.for_blocks.get(&name_) {
            None => {}
            Some(_) => { return Err(ErrorCode::IdRedefined(format!("for block {} already defined", name_.clone()))); }
        };

        let for_scope = ForScope::new(name_.clone(), for_var_exp_.clone(), for_array_exp_.clone());
        {
            let parent_scope = self.self_ref.clone().unwrap();
            for_scope.scope.write().unwrap().new_relationship_with_name(parent_scope.clone(), String::from("base"), ScopeRelationType::IfForScopeRela);
        }

        let scope_clone = for_scope.scope.clone();
        self.for_blocks.insert(name_.clone(), Arc::new(RwLock::new(for_scope)));
        return Ok(scope_clone);
    }

    pub fn with_for_block(&mut self, target: Arc<RwLock<ForScope>>, parent_scope_name: String) -> Result<(), ErrorCode> {
        if !(self.scope_type == ScopeType::ImplementScope || self.scope_type == ScopeType::IfForScope) { return Err(ErrorCode::ScopeNotAllowed(format!("not allowed to define for block outside of if/for/implement scope"))); }
        let name_ = target.read().unwrap().get_name();

        match self.for_blocks.get(&name_) {
            None => {}
            Some(_) => { return Err(ErrorCode::IdRedefined(format!("for block {} already defined", name_.clone()))); }
        };

        // update scope
        let parent_scope = self.self_ref.clone().unwrap();
        {
            let for_scope = target.read().unwrap().get_scope();
            for_scope.write().unwrap().new_relationship_with_name(parent_scope.clone(), parent_scope_name.clone(), ScopeRelationType::IfForScopeRela);
        }

        self.for_blocks.insert(name_.clone(), target);
        return Ok(());
    }
}
