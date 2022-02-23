use std::collections::HashSet;
use std::sync::{Arc, RwLock};
use deep_clone::DeepClone;
use implement::{Implement, ImplementType};
use crate::error::ErrorCode;
use crate::{generate_get, generate_set, generate_access};
use crate::inferable::{Inferable, InferState, NewInferable};
use crate::variable::Variable;
use crate::scope::{Scope, ScopeRelationType, ScopeType};
use crate::util::{generate_padding, PrettyPrint, EnableDocument};
use derived_macro::EnableDocument;

#[derive(Clone, Debug)]
pub enum InstanceArray {
    UnknownInstanceArray,
    SingleInstance,
    ArrayInstance(Arc<RwLock<Variable>>),
}

impl DeepClone for InstanceArray {
    fn deep_clone(&self) -> Self {
        return match self {
            InstanceArray::ArrayInstance(var) => InstanceArray::ArrayInstance(var.deep_clone()),
            _ => self.clone(),
        }
    }
}

impl From<InstanceArray> for String {
    fn from(arr: InstanceArray) -> Self {
        return match arr {
            InstanceArray::UnknownInstanceArray => { String::from("UnknownInstanceArray") }
            InstanceArray::SingleInstance => { String::from("SingleInstance") }
            InstanceArray::ArrayInstance(p) => { format!("ArrayInstance({})", String::from((*p.read().unwrap()).clone())) }
        }
    }
}

#[derive(Clone, Debug, EnableDocument)]
pub struct Instance {
    name: String,
    package: Option<String>,

    implement_type: Inferable<Arc<RwLock<Implement>>>,
    implement_template_argexp: Vec<Arc<RwLock<Variable>>>,
    array_type: InstanceArray,

    docu: Option<String>,
}

impl DeepClone for Instance {
    fn deep_clone(&self) -> Self {
        return Self {
            name: self.name.deep_clone(),
            package: self.package.deep_clone(),

            implement_type: self.implement_type.deep_clone(),
            implement_template_argexp: self.implement_template_argexp.deep_clone(),
            array_type: self.array_type.deep_clone(),

            docu: self.docu.deep_clone(),
        }
    }
}

impl Instance {
    generate_access!(name, String, get_name, set_name);
    generate_access!(package, Option<String>, get_package, set_package);
    generate_access!(implement_type, Inferable<Arc<RwLock<Implement>>>, get_implement_type, set_implement_type);
    generate_access!(implement_template_argexp, Vec<Arc<RwLock<Variable>>>, get_implement_argexp, set_implement_argexp);
    generate_access!(array_type, InstanceArray, get_array_type, set_array_type);

    pub fn new(name_: String, package_: Option<String>, streamlet_type_exp_: Inferable<Arc<RwLock<Implement>>>, template_argexp: Vec<Arc<RwLock<Variable>>>) -> Self {
        match streamlet_type_exp_.get_infer_state() {
            InferState::Inferred => {
                Self {
                    name: name_,
                    package: package_,
                    implement_type: <Inferable<Arc<RwLock<Implement>>> as NewInferable<Arc<RwLock<Implement>>>>::_new_inferred(streamlet_type_exp_.get_raw_exp(), streamlet_type_exp_.get_raw_value()),
                    implement_template_argexp: template_argexp,
                    array_type: InstanceArray::SingleInstance,
                    docu: None,
                }
            }
            InferState::NotInferred => {
                Self {
                    name: name_,
                    package: package_,
                    implement_type: <Inferable<Arc<RwLock<Implement>>> as NewInferable<Arc<RwLock<Implement>>>>::_new(streamlet_type_exp_.get_raw_exp()),
                    implement_template_argexp: template_argexp,
                    array_type: InstanceArray::SingleInstance,
                    docu: None,
                }
            }
        }
    }

    pub fn new_array(name_: String, package_: Option<String>, streamlet_type_exp_: Inferable<Arc<RwLock<Implement>>>, template_argexp: Vec<Arc<RwLock<Variable>>>, array_: Arc<RwLock<Variable>>) -> Self {
        match streamlet_type_exp_.get_infer_state() {
            InferState::Inferred => {
                Self {
                    name: name_,
                    package: package_,
                    implement_type: <Inferable<Arc<RwLock<Implement>>> as NewInferable<Arc<RwLock<Implement>>>>::_new_inferred(streamlet_type_exp_.get_raw_exp(), streamlet_type_exp_.get_raw_value()),
                    implement_template_argexp: template_argexp,
                    array_type: InstanceArray::ArrayInstance(array_),
                    docu: None,
                }
            }
            InferState::NotInferred => {
                Self {
                    name: name_,
                    package: package_,
                    implement_type: <Inferable<Arc<RwLock<Implement>>> as NewInferable<Arc<RwLock<Implement>>>>::_new(streamlet_type_exp_.get_raw_exp()),
                    implement_template_argexp: template_argexp,
                    array_type: InstanceArray::ArrayInstance(array_),
                    docu: None,
                }
            }
        }
    }
}

impl From<Instance> for String {
    fn from(inst: Instance) -> Self {
        let array_exp = match inst.clone().array_type {
            InstanceArray::UnknownInstanceArray => { String::from("[Unknown]") }
            InstanceArray::SingleInstance => { String::from("") }
            InstanceArray::ArrayInstance(var) => { format!("[{}]", String::from((*var.read().unwrap()).clone())) }
        };
        let package_exp = match inst.clone().package {
            None => { String::from("") }
            Some(package) => { format!("{}.", package) }
        };

        let mut template_flag = false;
        if inst.implement_type.get_infer_state() == InferState::Inferred {
            match inst.implement_type.get_raw_value().read().unwrap().get_type() {
                ImplementType::NormalImplement => {
                    template_flag = false;
                }
                ImplementType::TemplateImplement(_) => {
                    template_flag = true;
                }
                _ => unreachable!()
            }
        }

        return if template_flag {
            let mut argexp_output = String::from("");
            for single_argexp in inst.get_implement_argexp() {
                argexp_output.push_str(&format!("@{}", &String::from((*single_argexp.read().unwrap()).clone())));
            }
            format!("{}:{}({}<{}>){}", inst.get_name(), package_exp, String::from(inst.implement_type), argexp_output, array_exp)
        } else {
            format!("{}:{}({}){}", inst.get_name(), package_exp, String::from(inst.implement_type), array_exp)
        }
    }
}

impl PrettyPrint for Instance {
    fn pretty_print(&self, depth: u32, _: bool) -> String {
        return format!("{}{}", generate_padding(depth), String::from(self.clone()));
    }
}

impl Scope {
    pub fn new_instance(&mut self, name_: String, package_: Option<String>, implement_exp: Inferable<Arc<RwLock<Implement>>>, template_argexp: Vec<Arc<RwLock<Variable>>>) -> Result<(), ErrorCode> {
        if (self.scope_type != ScopeType::ImplementScope) && (self.scope_type != ScopeType::BasicScope && (self.scope_type != ScopeType::IfForScope)) { return Err(ErrorCode::ScopeNotAllowed(String::from("not allowed to define instances outside of implement or base scope"))); }

        match self.instances.get(&name_) {
            None => {}
            Some(_) => { return Err(ErrorCode::IdRedefined(format!("instance {} already defined", name_))); }
        };
        self.instances.insert(name_.clone(), Arc::new(RwLock::new(Instance::new(name_.clone(), package_, implement_exp, template_argexp))));
        return Ok(());
    }

    pub fn with_instance(&mut self, inst: Arc<RwLock<Instance>>) -> Result<(), ErrorCode> {
        if (self.scope_type != ScopeType::ImplementScope) && (self.scope_type != ScopeType::BasicScope && (self.scope_type != ScopeType::IfForScope)) { return Err(ErrorCode::ScopeNotAllowed(String::from("not allowed to define instances outside of implement or base scope"))); }

        let name_ = inst.read().unwrap().get_name();
        match self.instances.get(&name_) {
            None => {}
            Some(_) => { return Err(ErrorCode::IdRedefined(format!("instance {} already defined", name_))); }
        };
        self.instances.insert(name_.clone(), inst.clone());
        return Ok(());
    }

    pub fn new_instance_array(&mut self, name_: String, package_: Option<String>, implement_exp: Inferable<Arc<RwLock<Implement>>>, template_argexp: Vec<Arc<RwLock<Variable>>>, array_: Arc<RwLock<Variable>>) -> Result<(), ErrorCode> {
        if (self.scope_type != ScopeType::ImplementScope) && (self.scope_type != ScopeType::BasicScope && (self.scope_type != ScopeType::IfForScope)) { return Err(ErrorCode::ScopeNotAllowed(String::from("not allowed to define instances outside of implement or base scope"))); }

        match self.instances.get(&name_) {
            None => {}
            Some(_) => { return Err(ErrorCode::IdRedefined(format!("instance {} already defined", name_))); }
        };
        self.instances.insert(name_.clone(), Arc::new(RwLock::new(Instance::new_array(name_.clone(), package_, implement_exp, template_argexp, array_))));
        return Ok(());
    }

    pub fn resolve_instance_in_current_scope(& self, name_: String) -> Result<Arc<RwLock<Instance>>, ErrorCode> {
        return match self.instances.get(&name_) {
            None => { Err(ErrorCode::IdNotFound(format!("instance {} not found", name_))) }
            Some(var) => { Ok(var.clone()) }
        };
    }

    fn _resolve_instance_in_scope(target_scope: Arc<RwLock<Scope>>, name_: &String, allowed_relationships: &HashSet<ScopeRelationType>) -> Result<Arc<RwLock<Instance>>, ErrorCode> {
        let target_scope_r = target_scope.read().unwrap();

        //find self scope
        match target_scope_r.resolve_instance_in_current_scope(name_.clone()) {
            Ok(var) => { return Ok(var) }
            Err(_) => {}
        }

        //find in parent scope
        for (_, scope_real) in &(target_scope_r.scope_relationships) {
            let result = Scope::_resolve_instance_in_scope(scope_real.get_target_scope().clone(), &name_, &allowed_relationships);
            match result {
                Ok(inst) => {return Ok(inst)}
                Err(_) => {}
            }
        }

        return Err(ErrorCode::IdNotFound(format!("instance {} not found", name_.clone())));
    }

    pub fn resolve_instance_with_relation(& self, name_: String, allowed_relationships: Vec<ScopeRelationType>) -> Result<Arc<RwLock<Instance>>, ErrorCode> {
        match self.resolve_instance_in_current_scope(name_.clone()) {
            Ok(inst) => { return Ok(inst) }
            Err(_) => {}
        }

        let mut allowed_relationships_hash = HashSet::new();
        for allowed_relationship in allowed_relationships {
            allowed_relationships_hash.insert(allowed_relationship.clone());
        }

        //find in parent scope
        for (_, scope_real) in &(self.scope_relationships) {
            let result = Scope::_resolve_instance_in_scope(scope_real.get_target_scope().clone(), &name_, & allowed_relationships_hash);
            match result {
                Ok(var) => {return Ok(var)}
                Err(_) => {}
            }
        }

        return Err(ErrorCode::IdNotFound(format!("instance {} not found", name_.clone())));
    }

    pub fn resolve_instance_from_scope(& self, name_: String) -> Result<Arc<RwLock<Instance>>, ErrorCode> {
        use crate::scope::ScopeRelationType::*;
        let allowed_relationships = vec![GroupScopeRela, UnionScopeRela,
                                         StreamScopeRela, StreamletScopeRela, ImplementScopeRela];
        return self.resolve_instance_with_relation(name_, allowed_relationships);
    }
}