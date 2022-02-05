use std::sync::{Arc, RwLock};
use crate::data_type::DataType;
use crate::error::ErrorCode;
use crate::{generate_get, generate_access, generate_set};
use crate::inferable::Inferable;
use crate::port::{Port};
use crate::scope::{Scope, ScopeRelationType, ScopeType};
use crate::streamlet::Streamlet;
use crate::util::{generate_padding, PrettyPrint};
use crate::variable::Variable;

#[derive(Clone, Debug)]
pub enum ImplementType {
    UnknownType,
    NormalImplement,
    AnyImplementOfStreamlet(String, Option<Arc<RwLock<Streamlet>>>),
    TemplateImplement(Vec<Arc<RwLock<Variable>>>),
    DummyImplement,
}

impl From<ImplementType> for String {
    fn from(type_: ImplementType) -> Self {
        match type_ {
            ImplementType::UnknownType => { return String::from("UnknownType"); },
            ImplementType::NormalImplement => { return String::from("NormalImplement"); },
            ImplementType::AnyImplementOfStreamlet(s, _) => { return format!("AnyImplementOfStreamlet({})", s.clone()); },
            ImplementType::TemplateImplement(vars) => {
                let mut output = String::from("");
                for v in vars {
                    let type_ = v.read().unwrap().get_type();
                    output.push_str(&format!("@{}", String::from((*(type_.read().unwrap())).clone()) ));
                }
                return output;
            },
            ImplementType::DummyImplement => { return String::from("DummyImplement"); },
        }
    }
}

impl PrettyPrint for ImplementType {
    fn pretty_print(&self, _: u32, _: bool) -> String {
        return String::from(self.clone());
    }
}

#[derive(Clone, Debug)]
pub struct Implement {
    name: String,

    implement_type: ImplementType,
    scope: Arc<RwLock<Scope>>,

    derived_streamlet: Arc<RwLock<Variable>>,
}

impl Implement {
    generate_get!(name, String, get_name);
    generate_access!(implement_type, ImplementType, get_type, set_type);
    generate_get!(scope, Arc<RwLock<Scope>>, get_scope);
    generate_access!(derived_streamlet, Arc<RwLock<Variable>>, get_derived_streamlet, set_derived_streamlet);

    pub fn set_name(&mut self, name_: String) {
        self.name = name_.clone();
        self.scope.write().unwrap().set_name(format!("implement_{}", name_.clone()));
    }

    pub fn new(name_: String, type_: ImplementType) -> Self {
        let scope_ = Arc::new(RwLock::new(Scope::new(format!("implement_{}", name_.clone()), ScopeType::ImplementScope)));
        {
            scope_.write().unwrap().set_self_ref(scope_.clone());
        }
        Self {
            name: name_,
            implement_type: type_,
            scope: scope_,

            derived_streamlet: Arc::new(RwLock::new(Variable::new(String::from(""), DataType::UnknownType, String::from("")))),
        }
    }

    pub fn new_instance(& self, name_: String, package_: Option<String>, streamlet_: Inferable<Arc<RwLock<Streamlet>>>, template_argexp: Vec<Arc<RwLock<Variable>>>) -> Result<(), ErrorCode> {
        let mut scope = self.scope.write().unwrap();
        return scope.new_instance(name_.clone(), package_, streamlet_.clone(), template_argexp);
    }

    pub fn new_connection(& self, name_: String, lhs_port_: Inferable<Arc<RwLock<Port>>>, rhs_port_: Inferable<Arc<RwLock<Port>>>, delay_: Variable) -> Result<(), ErrorCode> {
        let mut scope = self.scope.write().unwrap();
        return scope.new_connection(name_.clone(), lhs_port_.clone(), rhs_port_.clone(), delay_.clone());
    }

    pub fn new_variable(&self, name_: String, type_: DataType, exp_: String) -> Result<(), ErrorCode> {
        let mut scope = self.scope.write().unwrap();
        return scope.new_variable(name_.clone(), type_.clone(), exp_.clone());
    }

}

impl From<Implement> for String {
    fn from(implement: Implement) -> Self {
        return format!("Implement({})", implement.get_name());
    }
}

impl PrettyPrint for Implement {
    fn pretty_print(&self, depth: u32, verbose: bool) -> String {
        let mut output = String::new();

        //enter Implement
        let derived_streamlet = self.derived_streamlet.read().unwrap().get_type();
        output.push_str(&format!("{}Implement({})<{}> -> {}{{\n", generate_padding(depth), self.name.clone(), String::from(self.implement_type.clone()), String::from((*derived_streamlet.read().unwrap()).clone() )));
        //enter scope
        output.push_str(&format!("{}", self.scope.read().unwrap().pretty_print(depth+1, verbose)));
        //leave Implement
        output.push_str(&format!("{}}}", generate_padding(depth)));

        return output;
    }
}

impl Scope {
    pub fn new_implement(&mut self, name_: String, type_: ImplementType) -> Result<Arc<RwLock<Scope>>, ErrorCode> {
        if self.scope_type != ScopeType::BasicScope { return Err(ErrorCode::ScopeNotAllowed(format!("not allowed to define implement outside of base scope"))); }

        match self.implements.get(&name_) {
            None => {}
            Some(_) => { return Err(ErrorCode::IdRedefined(format!("implement {} already defined", name_.clone()))); }
        };

        let implement = Implement::new(name_.clone(), type_.clone());
        {
            let parent_scope = self.self_ref.clone().unwrap();
            implement.scope.write().unwrap().new_relationship_with_name(parent_scope.clone(), String::from("base"), ScopeRelationType::ImplementScopeRela);
        }

        let scope_copy = implement.scope.clone();
        let implement_box = Arc::new(RwLock::new(implement));
        self.implements.insert(name_.clone(), implement_box.clone());
        return Ok(scope_copy);
    }

    pub fn with_implement(&mut self, implement: Implement) -> Result<Arc<RwLock<Scope>>, ErrorCode> {
        if self.scope_type != ScopeType::BasicScope { return Err(ErrorCode::ScopeNotAllowed(format!("not allowed to define streamlet outside of base scope"))); }

        match self.streamlets.get(&implement.name) {
            None => {}
            Some(_) => { return Err(ErrorCode::IdRedefined(format!("implement {} already defined", implement.get_name()))); }
        };

        let scope_clone = implement.scope.clone();
        self.implements.insert(implement.get_name(), Arc::new(RwLock::new(implement)));
        return Ok(scope_clone);
    }
}