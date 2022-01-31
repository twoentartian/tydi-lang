use std::collections::HashSet;
use std::sync::{Arc, RwLock};
use crate::variable::Variable;
use crate::data_type::DataType;
use crate::error::ErrorCode;
use crate::{generate_get, generate_access, generate_set};
use crate::inferable::Inferable;
use crate::logical_data_type::LogicalDataType;
use crate::port::PortDirection;
use crate::scope::{Scope, ScopeRelationType, ScopeType};
use crate::util::{generate_padding, PrettyPrint};

#[derive(Clone, Debug)]
pub enum StreamletType {
    UnknownType,
    NormalStreamlet,
    TemplateStreamlet(Vec<Arc<RwLock<Variable>>>),
    DummyStremlet,
}

impl From<StreamletType> for String {
    fn from(type_: StreamletType) -> Self {
        match type_ {
            StreamletType::UnknownType => { return String::from("UnknownType"); },
            StreamletType::NormalStreamlet => { return String::from("NormalStreamlet"); },
            StreamletType::TemplateStreamlet(vars) => {
                let mut output = String::from("");
                for v in vars {
                    let type_ = v.read().unwrap().get_type();
                    output.push_str(&format!("@{}", String::from((*(type_.read().unwrap())).clone()) ));
                }
                return output;
            },
            StreamletType::DummyStremlet => { return String::from("DummyStremlet"); },
        }
    }
}

impl PrettyPrint for StreamletType {
    fn pretty_print(&self, depth: u32, verbose: bool) -> String {
        return String::from(self.clone());
    }
}

#[derive(Clone, Debug)]
pub struct Streamlet {
    name: String,

    streamlet_type: StreamletType,
    scope: Arc<RwLock<Scope>>,
}

impl Streamlet {
    generate_get!(name, String, get_name);
    generate_access!(streamlet_type, StreamletType, get_type, set_type);
    generate_get!(scope, Arc<RwLock<Scope>>, get_scope);

    pub fn set_name(&mut self, name_: String) {
        self.name = name_.clone();
        self.scope.write().unwrap().set_name(format!("streamlet_{}", name_.clone()));
    }

    pub fn new(name_: String, type_: StreamletType) -> Self {
        let scope_ = Arc::new(RwLock::new(Scope::new(format!("streamlet_{}", name_.clone()), ScopeType::StreamletScope)));
        {
            scope_.write().unwrap().set_self_ref(scope_.clone());
        }
        Self {
            name: name_,
            streamlet_type: type_,
            scope: scope_,
        }
    }

    pub fn new_port(& self, name_: String, type_: Inferable<Arc<RwLock<LogicalDataType>>>, direction_: PortDirection) -> Result<(),ErrorCode> {
        return self.scope.write().unwrap().new_port(name_.clone(), type_.clone(), direction_.clone());
    }

    pub fn new_port_array(& self, name_: String, type_: Inferable<Arc<RwLock<LogicalDataType>>>, direction_: PortDirection, array_: Arc<RwLock<Variable>>) -> Result<(),ErrorCode> {
        return self.scope.write().unwrap().new_port_array(name_.clone(), type_.clone(), direction_.clone(), array_.clone());
    }

    pub fn new_variable(& self, name_: String, type_: DataType, exp_: String) -> Result<(), ErrorCode> {
        return self.scope.write().unwrap().new_variable(name_.clone(), type_.clone(), exp_.clone());
    }
}

impl From<Streamlet> for String {
    fn from(streamlet: Streamlet) -> Self {
        return format!("Streamlet({})", streamlet.get_name());
    }
}

impl PrettyPrint for Streamlet {
    fn pretty_print(&self, depth: u32, verbose: bool) -> String {
        let mut output = String::new();

        //enter group
        output.push_str(&format!("{}Streamlet({})<{}>{{\n", generate_padding(depth), self.name.clone(), String::from(self.streamlet_type.clone())));
        //enter scope
        output.push_str(&format!("{}", self.scope.read().unwrap().pretty_print(depth+1, verbose)));
        //leave group
        output.push_str(&format!("{}}}", generate_padding(depth)));

        return output;
    }
}

impl Scope {
    pub fn new_streamlet(&mut self, name_: String, type_: StreamletType) -> Result<Arc<RwLock<Scope>>, ErrorCode> {
        if self.scope_type != ScopeType::BasicScope { return Err(ErrorCode::ScopeNotAllowed(format!("not allowed to define streamlet outside of base scope"))); }

        match self.streamlets.get(&name_) {
            None => {}
            Some(_) => { return Err(ErrorCode::IdRedefined(format!("streamlet {} already defined", name_.clone()))); }
        };

        let streamlet = Streamlet::new(name_.clone(), type_.clone());
        {
            let parent_scope = self.self_ref.clone().unwrap();
            streamlet.scope.write().unwrap().new_relationship_with_name(parent_scope.clone(), String::from("base"), ScopeRelationType::StreamletScopeRela);
        }

        let scope_clone = streamlet.scope.clone();
        self.streamlets.insert(name_.clone(), Arc::new(RwLock::new(streamlet)));
        return Ok(scope_clone);
    }

    pub fn with_streamlet(&mut self, streamlet: Streamlet) -> Result<Arc<RwLock<Scope>>, ErrorCode> {
        if self.scope_type != ScopeType::BasicScope { return Err(ErrorCode::ScopeNotAllowed(format!("not allowed to define streamlet outside of base scope"))); }

        match self.streamlets.get(&streamlet.name) {
            None => {}
            Some(_) => { return Err(ErrorCode::IdRedefined(format!("streamlet {} already defined", streamlet.get_name()))); }
        };

        let scope_clone = streamlet.scope.clone();
        self.streamlets.insert(streamlet.get_name(), Arc::new(RwLock::new(streamlet)));
        return Ok(scope_clone);
    }

    pub fn resolve_streamlet_in_current_scope(& self, name_: String) -> Result<Arc<RwLock<Streamlet>>, ErrorCode> {
        return match self.streamlets.get(&name_) {
            None => { Err(ErrorCode::IdNotFound(format!("streamlet {} not found", name_))) }
            Some(streamlet) => { Ok(streamlet.clone()) }
        };
    }

    fn _resolve_streamlet_in_scope(target_scope: Arc<RwLock<Scope>>, name_: &String, allowed_relationships: &HashSet<ScopeRelationType>) -> Result<Arc<RwLock<Streamlet>>, ErrorCode> {
        let target_scope_r = target_scope.read().unwrap();

        //find self scope
        match target_scope_r.resolve_streamlet_in_current_scope(name_.clone()) {
            Ok(streamlet) => { return Ok(streamlet) }
            Err(_) => {}
        }

        //find in parent scope
        for (_, scope_real) in &(target_scope_r.scope_relationships) {
            let result = Scope::_resolve_streamlet_in_scope(scope_real.get_target_scope().clone(), &name_, &allowed_relationships);
            match result {
                Ok(var) => {return Ok(var)}
                Err(_) => {}
            }
        }

        return Err(ErrorCode::IdNotFound(format!("streamlet {} not found", name_.clone())));
    }

    pub fn resolve_streamlet_with_relation(& self, name_: String, allowed_relationships: Vec<ScopeRelationType>) -> Result<Arc<RwLock<Streamlet>>, ErrorCode> {
        match self.resolve_streamlet_in_current_scope(name_.clone()) {
            Ok(streamlet) => { return Ok(streamlet) }
            Err(_) => {}
        }

        let mut allowed_relationships_hash = HashSet::new();
        for allowed_relationship in allowed_relationships {
            allowed_relationships_hash.insert(allowed_relationship.clone());
        }

        //find in parent scope
        for (_, scope_real) in &(self.scope_relationships) {
            let result = Scope::_resolve_streamlet_in_scope(scope_real.get_target_scope().clone(), &name_, & allowed_relationships_hash);
            match result {
                Ok(var) => {return Ok(var)}
                Err(_) => {}
            }
        }

        return Err(ErrorCode::IdNotFound(format!("streamlet {} not found", name_.clone())));
    }

    pub fn resolve_streamlet_from_scope(& self, name_: String) -> Result<Arc<RwLock<Streamlet>>, ErrorCode> {
        use crate::scope::ScopeRelationType::*;
        let allowed_relationships = vec![GroupScopeRela, UnionScopeRela,
                                         StreamScopeRela, StreamletScopeRela, ImplementScopeRela];
        return self.resolve_streamlet_with_relation(name_, allowed_relationships);
    }

}