pub use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use deep_clone::DeepClone;

pub use crate::logical_data_type::*;
pub use crate::variable::*;
pub use crate::data_type::*;
pub use crate::group_union_type::*;
pub use crate::steam_type::*;
pub use crate::streamlet::*;
pub use crate::port::*;
pub use crate::implement::Implement;
pub use crate::type_alias::TypeAlias;
pub use crate::instances::{Instance, InstanceArray};
pub use crate::if_for::{IfScope, ForScope, ElifScope, ElseScope};
pub use crate::connection::{Connection, PortOwner};

pub use crate::inferable::*;
pub use crate::error::ErrorCode;
pub use crate::util::*;

use crate::{generate_get, generate_set, generate_access};

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum ScopeRelationType {
    GroupScopeRela,
    UnionScopeRela,
    StreamScopeRela,
    StreamletScopeRela,
    ImplementScopeRela,
    IfForScopeRela,

    ParentScopeRela, // a placeholder, should never be used
}

impl DeepClone for ScopeRelationType {
    fn deep_clone(&self) -> Self {
        return self.clone();
    }
}

impl From<ScopeRelationType> for String {
    fn from(relationship: ScopeRelationType) -> Self {
        match relationship {
            ScopeRelationType::GroupScopeRela => {String::from("GroupScope")}
            ScopeRelationType::UnionScopeRela => {String::from("UnionScope")}
            ScopeRelationType::StreamScopeRela => {String::from("StreamScope")}
            ScopeRelationType::StreamletScopeRela => {String::from("StreamletScope")}
            ScopeRelationType::ImplementScopeRela => {String::from("ImplementScope")}
            ScopeRelationType::IfForScopeRela => {String::from("IfForScope")}
            ScopeRelationType::ParentScopeRela => {String::from("ParentScope")}
        }
    }
}

#[derive(Clone, Debug)]
pub struct ScopeRelationship {
    name: String,
    target_scope: Arc<RwLock<Scope>>,
    relationship: ScopeRelationType,
}

impl DeepClone for ScopeRelationship {
    fn deep_clone(&self) -> Self {
        return Self{
            name: self.name.clone(),
            target_scope: self.target_scope.clone(),
            relationship: self.relationship.deep_clone(),
        }
    }
}

impl ScopeRelationship {
    generate_get!(name, String, get_name);
    generate_access!(target_scope, Arc<RwLock<Scope>>, get_target_scope, set_target_scope);

    pub fn new(target_scope_: Arc<RwLock<Scope>>, relationship_: ScopeRelationType) -> Self {
        Self {
            name: format!("{}_{}", target_scope_.read().unwrap().get_name(), String::from(relationship_.clone())),
            target_scope: target_scope_.clone(),
            relationship: relationship_.clone(),
        }
    }

    pub fn new_with_name(target_scope_: Arc<RwLock<Scope>>, target_scope_name: String, relationship_: ScopeRelationType) -> Self {
        Self {
            name: format!("{}_{}", target_scope_name.clone(), String::from(relationship_.clone())),
            target_scope: target_scope_.clone(),
            relationship: relationship_.clone(),
        }
    }

}

impl PrettyPrint for ScopeRelationship {
    fn pretty_print(&self, depth: u32, _: bool) -> String {
        return format!("{}--{}-->{}", generate_padding(depth), String::from(self.relationship.clone()), self.target_scope.read().unwrap().get_name() );
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ScopeType {
    BasicScope,

    GroupScope,
    UnionScope,
    StreamScope,
    StreamletScope,
    ImplementScope,

    IfForScope,

    ParentScope, // a placeholder, should never be used
}

impl DeepClone for ScopeType {
    fn deep_clone(&self) -> Self {
        return self.clone();
    }
}

impl From<ScopeType> for String {
    fn from(scope_type: ScopeType) -> Self {
        match scope_type {
            ScopeType::BasicScope => {String::from("BasicScope")}
            ScopeType::GroupScope => {String::from("GroupScope")}
            ScopeType::UnionScope => {String::from("UnionScope")}
            ScopeType::StreamScope => {String::from("StreamScope")}
            ScopeType::StreamletScope => {String::from("StreamletScope")}
            ScopeType::ImplementScope => {String::from("ImplementScope")}
            ScopeType::IfForScope => {String::from("IfForScope")}
            ScopeType::ParentScope => {String::from("ParentScope")}
        }
    }
}

#[derive(Clone, Debug)]
pub struct Scope {
    name: String,
    pub scope_type: ScopeType,
    pub self_ref: Option<Arc<RwLock<Scope>>>,

    pub scope_relationships: HashMap<String, ScopeRelationship>,
    pub types: HashMap<String, Arc<RwLock<TypeAlias>>>,
    pub vars: HashMap<String, Arc<RwLock<Variable>>>,

    pub streamlets: HashMap<String, Arc<RwLock<Streamlet>>>,
    pub ports: HashMap<String, Arc<RwLock<Port>>>,
    pub implements: HashMap<String, Arc<RwLock<Implement>>>,
    pub instances: HashMap<String, Arc<RwLock<Instance>>>,
    pub connections: HashMap<String, Arc<RwLock<Connection>>>,

    pub if_blocks: HashMap<String, Arc<RwLock<IfScope>>>,
    pub for_blocks: HashMap<String, Arc<RwLock<ForScope>>>,
}

impl DeepClone for Scope {
    fn deep_clone(&self) -> Self {
        let mut output = Self {
            name: self.name.clone(),
            scope_type: self.scope_type.deep_clone(),
            self_ref: None,

            scope_relationships: self.scope_relationships.deep_clone(),
            types: self.types.deep_clone(),
            vars: self.vars.deep_clone(),

            streamlets: self.streamlets.deep_clone(),
            ports: self.ports.deep_clone(),
            implements: self.implements.deep_clone(),
            instances: self.instances.deep_clone(),
            connections: self.connections.deep_clone(),

            if_blocks: self.if_blocks.deep_clone(),
            for_blocks: self.for_blocks.deep_clone(),
        };

        output
    }
}

impl Scope {
    generate_access!(name, String, get_name, set_name);

    pub fn new(name_: String, scope_type_: ScopeType) -> Self {
        Self {
            name: name_,
            self_ref: None,
            scope_type: scope_type_,
            scope_relationships: HashMap::new(),
            types: HashMap::new(),
            vars: HashMap::new(),
            streamlets: HashMap::new(),
            ports: HashMap::new(),
            implements: HashMap::new(),
            instances: HashMap::new(),
            connections: HashMap::new(),
            if_blocks: HashMap::new(),
            for_blocks: HashMap::new(),
        }
    }

    pub fn set_self_ref(&mut self, self_ref: Arc<RwLock<Scope>>) {
        self.self_ref = Some(self_ref);
    }

    pub fn new_relationship(&mut self, target_scope: Arc<RwLock<Scope>>, relationship: ScopeRelationType) {
        let new_scope_relationship = ScopeRelationship::new(target_scope, relationship);
        self.scope_relationships.insert(new_scope_relationship.get_name(), new_scope_relationship);
    }

    pub fn new_relationship_with_name(&mut self, target_scope: Arc<RwLock<Scope>>, name: String, relationship: ScopeRelationType) {
        let new_scope_relationship = ScopeRelationship::new_with_name(target_scope, name, relationship);
        self.scope_relationships.insert(new_scope_relationship.get_name(), new_scope_relationship);
    }

}

impl PrettyPrint for Scope {
    fn pretty_print(&self, depth: u32, verbose: bool) -> String {
        let mut output = String::new();

        //enter scope
        output.push_str(&format!("{}Scope({}){{\n", generate_padding(depth), self.name.clone()));

        if !self.vars.is_empty() || verbose {
            //enter vars
            output.push_str(&format!("{}Variables{{\n", generate_padding(depth+1)));
            for (_, var) in &self.vars {
                output.push_str(&format!("{}\n", var.read().unwrap().pretty_print(depth+2, verbose)));
            }
            output.push_str(&format!("{}}}\n", generate_padding(depth+1)));
        }
        if !self.types.is_empty() || verbose{
            //enter types
            output.push_str(&format!("{}Types{{\n", generate_padding(depth+1)));
            for (type_name, t) in &self.types {
                output.push_str(&format!("{}{}:{}\n", generate_padding(depth+2), type_name, t.read().unwrap().pretty_print(depth+2, verbose)));
            }
            output.push_str(&format!("{}}}\n", generate_padding(depth+1)));
        }
        if !self.scope_relationships.is_empty() || verbose {
            //enter scope_relationships
            output.push_str(&format!("{}ScopeRelations{{\n", generate_padding(depth+1)));
            for (_, scope_relation) in &self.scope_relationships {
                output.push_str(&format!("{}\n", scope_relation.pretty_print(depth+2, verbose)) );
            }
            output.push_str(&format!("{}}}\n", generate_padding(depth+1)));
        }
        if !self.streamlets.is_empty() || verbose {
            //enter scope_relationships
            output.push_str(&format!("{}Streamlets{{\n", generate_padding(depth+1)));
            for (_, streamlet) in &self.streamlets {
                output.push_str(&format!("{}\n", streamlet.read().unwrap().pretty_print(depth+2, verbose)) );
            }
            output.push_str(&format!("{}}}\n", generate_padding(depth+1)));
        }
        if !self.ports.is_empty() || verbose {
            //enter scope_relationships
            output.push_str(&format!("{}Ports{{\n", generate_padding(depth+1)));
            for (_, port) in &self.ports {
                output.push_str(&format!("{}\n", port.read().unwrap().pretty_print(depth+2, verbose)) );
            }
            output.push_str(&format!("{}}}\n", generate_padding(depth+1)));
        }
        if !self.implements.is_empty() || verbose {
            //enter scope_relationships
            output.push_str(&format!("{}Implements{{\n", generate_padding(depth+1)));
            for (_, implement) in &self.implements {
                output.push_str(&format!("{}\n", implement.read().unwrap().pretty_print(depth+2, verbose)) );
            }
            output.push_str(&format!("{}}}\n", generate_padding(depth+1)));
        }
        if !self.instances.is_empty() || verbose {
            //enter scope_relationships
            output.push_str(&format!("{}Instances{{\n", generate_padding(depth+1)));
            for (_, inst) in &self.instances {
                output.push_str(&format!("{}\n", inst.read().unwrap().pretty_print(depth+2, verbose)) );
            }
            output.push_str(&format!("{}}}\n", generate_padding(depth+1)));
        }
        if !self.connections.is_empty() || verbose {
            //enter scope_relationships
            output.push_str(&format!("{}Connections{{\n", generate_padding(depth+1)));
            for (_, inst) in &self.connections {
                output.push_str(&format!("{}\n", inst.read().unwrap().pretty_print(depth+2, verbose)) );
            }
            output.push_str(&format!("{}}}\n", generate_padding(depth+1)));
        }
        if !self.if_blocks.is_empty() || verbose {
            //enter scope_relationships
            output.push_str(&format!("{}IfBlocks{{\n", generate_padding(depth+1)));
            for (_, inst) in &self.if_blocks {
                output.push_str(&format!("{}\n", inst.read().unwrap().pretty_print(depth+2, verbose)) );
            }
            output.push_str(&format!("{}}}\n", generate_padding(depth+1)));
        }
        if !self.for_blocks.is_empty() || verbose {
            //enter scope_relationships
            output.push_str(&format!("{}ForBlocks{{\n", generate_padding(depth+1)));
            for (_, inst) in &self.for_blocks {
                output.push_str(&format!("{}\n", inst.read().unwrap().pretty_print(depth+2, verbose)) );
            }
            output.push_str(&format!("{}}}\n", generate_padding(depth+1)));
        }

        //leave scope
        output.push_str(&format!("{}}}\n", generate_padding(depth)));
        return output;
    }
}
