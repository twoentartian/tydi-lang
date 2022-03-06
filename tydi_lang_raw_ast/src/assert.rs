use std::sync::{Arc, RwLock};
use deep_clone::DeepClone;
use error::ErrorCode;
use scope::Scope;
use util::{generate_padding, PrettyPrint};

use crate::variable::Variable;
use crate::{generate_get, generate_access, generate_set};

#[derive(Clone, Debug)]
pub struct Assert {
    name: String,

    var: Arc<RwLock<Variable>>,
    msg: Option<Arc<RwLock<Variable>>>,
}

impl Assert {
    generate_get!(name, String, get_name);
    generate_access!(var, Arc<RwLock<Variable>>, get_var, set_var);
    generate_access!(msg, Option<Arc<RwLock<Variable>>>, get_msg, set_msg);

    pub fn new(name: String) -> Self {
        return Self {
            name: name.clone(),
            var: Arc::new(RwLock::new(Variable::new_empty())),
            msg: None,
        }
    }
}

impl DeepClone for Assert {
    fn deep_clone(&self) -> Self {
        return Self {
            name: self.name.deep_clone(),
            var: self.var.deep_clone(),
            msg: self.msg.deep_clone(),
        };
    }
}

impl From<Assert> for String {
    fn from(assert: Assert) -> Self {
        return format!("assert({})", String::from((*assert.var.read().unwrap()).clone()));
    }
}

impl PrettyPrint for Assert {
    fn pretty_print(&self, depth: u32, _: bool) -> String {
        return format!("{}{}", generate_padding(depth), String::from((*self).clone()));
    }
}

impl Scope {
    pub fn with_assert(&mut self, assert: Arc<RwLock<Assert>>) -> Result<(), ErrorCode> {
        let name_ = assert.read().unwrap().get_name();
        match self.asserts.get(&name_) {
            None => {}
            Some(_) => { return Err(ErrorCode::IdRedefined(format!("assert {} already defined", name_))); }
        };
        self.asserts.insert(name_.clone(), assert.clone());
        return Ok(());
    }
}