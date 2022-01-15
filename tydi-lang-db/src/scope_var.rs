pub use std::collections::BTreeMap;
use crate::database::Project;
use crate::id::Id;
use crate::identifier::Identifier;

use crate::logical_data_type::LogicalDataType;
use crate::scope_var::DataType::Unknown;

/// region: Scope
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Scope {
    pub scope_relationships: Vec<Id<ScopeRelation>>,
    pub types: BTreeMap<String, Id<TypeProxy>>,
    pub vars: BTreeMap<String, Id<Variable>>,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            scope_relationships: vec![],
            types: BTreeMap::new(),
            vars: BTreeMap::new(),
        }
    }

    pub fn with_type_proxy(&mut self, type_name: String, type_proxy: Id<TypeProxy>) {
        self.types.insert(type_name.clone(), type_proxy);
    }

    pub fn find_type_proxy(& self, type_name: String) -> Result<Id<TypeProxy>, String> {
        match self.types.get(&type_name) {
            Some(type_proxy_id) => Ok(type_proxy_id.clone()),
            None => Err(format!("type (proxy) {} not found", type_name.clone())),
        }
    }

    pub fn with_variable(&mut self, var_name: String, var: Id<Variable>) {
        self.vars.insert(var_name.clone(), var);
    }

    pub fn find_variable(& self, var_name: String) -> Result<Id<Variable>, String> {
        match self.vars.get(&var_name) {
            Some(var_id) => Ok(var_id.clone()),
            None => Err(format!("variable {} not found", var_name.clone())),
        }
    }

}

/// region: Scope Relation
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ScopeRelationType {
    GroupScope,
    UnionScope,
    StreamScope,
    StreamletScope,
    ImplementationScope,

    ParentScope, // a placeholder, should never be used
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ScopeRelation {
    pub target_scope_relation_type: ScopeRelationType,
    pub target_scope_id: Id<Scope>,
    pub source_scope_id: Id<Scope>,
}

/// region: Type
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TypeProxy {
    pub raw_type: DataType,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum DataType {
    Unknown,  /// reserved Unknown type
    UnableToInfer,  /// reserved UnableToInfer type, means currently the type is unknown
                    /// but might be inferred in the future. Eg, template.

    Int,
    String,
    Bool,
    Float,
    Array(Id<TypeProxy>),

    EmptyLogicalDataType,
    LogicalDataType(LogicalDataType),
}

/// region: Variables
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Variable {
    pub identifier: Id<Identifier>,
    pub var_type: DataType,
}

impl Variable {
    pub fn new(name: Id<Identifier>) -> Self {
        Self {
            identifier: name,
            var_type: Unknown,
        }
    }
}
