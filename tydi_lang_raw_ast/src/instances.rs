use std::collections::HashSet;
use std::sync::{Arc, RwLock};
use crate::error::ErrorCode;
use crate::streamlet::Streamlet;
use crate::generate_get;
use crate::inferable::{Inferable, InferState, NewInferable};
use crate::scope::{Scope, ScopeRelationType, ScopeType};
use crate::util::{generate_padding, PrettyPrint};

#[derive(Clone, Debug)]
pub struct Instance {
    name: String,

    streamlet_type: Inferable<Arc<RwLock<Streamlet>>>,
}

impl Instance {
    generate_get!(name, String, get_name);
    generate_get!(streamlet_type, Inferable<Arc<RwLock<Streamlet>>>, get_streamlet_type);

    pub fn new(name_: String, streamlet_type_exp_: Inferable<Arc<RwLock<Streamlet>>>) -> Self {
        match streamlet_type_exp_.get_infer_state() {
            InferState::Inferred => {
                Self {
                    name: name_,
                    streamlet_type: <Inferable<Arc<RwLock<Streamlet>>> as NewInferable<Arc<RwLock<Streamlet>>>>::_new_inferred(streamlet_type_exp_.get_raw_exp(), streamlet_type_exp_.get_raw_value()),
                }
            }
            InferState::NotInferred => {
                Self {
                    name: name_,
                    streamlet_type: <Inferable<Arc<RwLock<Streamlet>>> as NewInferable<Arc<RwLock<Streamlet>>>>::_new(streamlet_type_exp_.get_raw_exp()),
                }
            }
        }
    }
}

impl From<Instance> for String {
    fn from(inst: Instance) -> Self {
        return format!("{}:Instance({})", inst.get_name(), String::from(inst.streamlet_type));
    }
}

impl PrettyPrint for Instance {
    fn pretty_print(&self, depth: u32, _: bool) -> String {
        return format!("{}{}", generate_padding(depth), String::from(self.clone()));
    }
}

impl Scope {
    pub fn new_instance(&mut self, name_: String, streamlet_exp: Inferable<Arc<RwLock<Streamlet>>>) -> Result<(), ErrorCode> {
        if (self.scope_type != ScopeType::ImplementScope) && (self.scope_type != ScopeType::BasicScope) { return Err(ErrorCode::ScopeNotAllowed(String::from("not allowed to define instances outside of implement or base scope"))); }

        match self.instances.get(&name_) {
            None => {}
            Some(_) => { return Err(ErrorCode::IdRedefined(format!("instance {} already defined", name_))); }
        };
        self.instances.insert(name_.clone(), Arc::new(RwLock::new(Instance::new(name_.clone(), streamlet_exp))));
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