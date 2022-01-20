use std::collections::HashSet;
use std::fs::copy;
use std::sync::{Arc, RwLock};
use crate::data_type::DataType;
use crate::error::ErrorCode;
use crate::generate_get;
use crate::inferable::Inferable;
use crate::logical_data_type::LogicalDataType;
use crate::port::{Port, PortDirection};
use crate::scope::{Scope, ScopeRelationType, ScopeType};
use crate::streamlet::Streamlet;
use crate::util::{generate_padding, PrettyPrint};
use crate::variable::Variable;

#[derive(Clone, Debug)]
pub enum ImplementType {
    UnknownType,
    NormalImplement,
    AnyImplementOfStreamlet(Arc<RwLock<Streamlet>>),
    TemplateImplement(Vec<Arc<RwLock<DataType>>>),
}

#[derive(Clone, Debug)]
pub struct Implement {
    name: String,

    implement_type: ImplementType,
    scope: Arc<RwLock<Scope>>,
}

impl Implement {
    generate_get!(name, String, get_name);
    generate_get!(implement_type, ImplementType, get_type);
    generate_get!(scope, Arc<RwLock<Scope>>, get_scope);

    pub fn new(name_: String, type_: ImplementType) -> Self {
        let scope_ = Arc::new(RwLock::new(Scope::new(format!("implement_{}", name_.clone()), ScopeType::ImplementScope)));
        {
            scope_.write().unwrap().set_self_ref(scope_.clone());
        }
        Self {
            name: name_,
            implement_type: type_,
            scope: scope_,
        }
    }

    pub fn new_instance(& self, name_: String, streamlet_: Inferable<Arc<RwLock<Streamlet>>>) -> Result<(), ErrorCode> {
        let mut scope = self.scope.write().unwrap();
        return scope.new_instance(name_.clone(), streamlet_.clone());
    }

    pub fn new_connection(& self, name_: String, lhs_port_: Inferable<Arc<RwLock<Port>>>, rhs_port_: Inferable<Arc<RwLock<Port>>>, delay_: Variable) -> Result<(), ErrorCode> {
        let mut scope = self.scope.write().unwrap();
        return scope.new_connection(name_.clone(), lhs_port_.clone(), rhs_port_.clone(), delay_.clone());
    }

    pub fn new_variable(& self, name_: String, type_: DataType, exp_: String) -> Result<(), ErrorCode> {
        {
            self.scope.write().unwrap().new_variable(name_.clone(), type_.clone(), exp_.clone());
        }
        return Ok(());
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

        //enter group
        output.push_str(&format!("{}Implement({}){{\n", generate_padding(depth), self.name.clone()));
        //enter scope
        output.push_str(&format!("{}", self.scope.read().unwrap().pretty_print(depth+1, verbose)));
        //leave group
        output.push_str(&format!("{}}}", generate_padding(depth)));

        return output;
    }
}

impl Scope {
    pub fn new_implement(&mut self, name_: String, type_: ImplementType) -> Result<Arc<RwLock<Scope>>, ErrorCode> {
        if self.scope_type != ScopeType::BasicScope { panic!("not allowed to define implement outside of base scope") }

        match self.implements.get(&name_) {
            None => {}
            Some(_) => { return Err(ErrorCode::IdRedefined(format!("implement {} already defined", name_.clone()))); }
        };

        let mut implement = Implement::new(name_.clone(), type_.clone());
        {
            let parent_scope = self.self_ref.clone().unwrap();
            implement.scope.write().unwrap().new_relationship_with_name(parent_scope.clone(), String::from("base"), ScopeRelationType::ImplementScopeRela);
        }

        let scope_copy = implement.scope.clone();
        let implement_box = Arc::new(RwLock::new(implement));
        self.implements.insert(name_.clone(), implement_box.clone());
        return Ok(scope_copy);
    }

}