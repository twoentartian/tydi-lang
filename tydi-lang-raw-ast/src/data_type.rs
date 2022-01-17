use std::collections::HashSet;
use std::sync::{Arc, RwLock};
use crate::error::ErrorCode;
use crate::logical_data_type::LogicalDataType;
use crate::scope::{Scope, ScopeRelationType};
use crate::util::PrettyPrint;

#[derive(Clone, Debug)]
pub enum DataType {
    UnknownType,  /// reserved Unknown type
    UnableToInfer,  /// reserved UnableToInfer type, means currently the type is unknown, but might be inferred in the future. Eg, template.

    IntType,
    StringType,
    BoolType,
    FloatType,
    ArrayType(Arc<RwLock<DataType>>),

    EmptyLogicalDataType,
    LogicalDataType(Arc<RwLock<LogicalDataType>>),
}

impl From<DataType> for String {
    fn from(t: DataType) -> Self {
        match t {
            DataType::UnknownType => { return String::from("UnknownType"); }
            DataType::UnableToInfer => { return String::from("UnableToInfer"); }
            DataType::IntType => { return String::from("int"); }
            DataType::StringType => { return String::from("string"); }
            DataType::BoolType => { return String::from("bool"); }
            DataType::FloatType => { return String::from("float"); }
            DataType::ArrayType(inner_data_type) => {
                let inner_type = inner_data_type.read().unwrap();
                let inner_type_str = String::from((*inner_type).clone());
                return format!("array({})", inner_type_str);
            }
            DataType::EmptyLogicalDataType => { return String::from("EmptyLogicalData"); }
            DataType::LogicalDataType(logical_data_type) => {
                let inner_type = logical_data_type.read().unwrap();
                let inner_type_str = String::from((*inner_type).clone());
                return format!("LogicalDataType({})", inner_type_str);
            }
        }
    }
}

impl PrettyPrint for DataType {
    fn pretty_print(&self, depth: u32, verbose: bool) -> String {
        match self {
            DataType::UnknownType => { return String::from(self.clone()); }
            DataType::UnableToInfer => { return String::from(self.clone()); }
            DataType::IntType => { return String::from(self.clone()); }
            DataType::StringType => { return String::from(self.clone()); }
            DataType::BoolType => { return String::from(self.clone()); }
            DataType::FloatType => { return String::from(self.clone()); }
            DataType::ArrayType(inner_data_type) => {
                let inner_type = inner_data_type.read().unwrap();
                let inner_type_str = inner_type.pretty_print(depth, verbose);
                return format!("array({})", inner_type_str);
            }
            DataType::EmptyLogicalDataType => { return String::from(self.clone()); }
            DataType::LogicalDataType(logical_data_type) => {
                let logical_data_type = logical_data_type.read().unwrap();
                let output = logical_data_type.pretty_print(depth, verbose);
                return output;
            }
        }
    }
}

impl Scope {
    pub fn resolve_type_in_current_scope(& self, name_: String) -> Result<Arc<RwLock<DataType>>, ErrorCode> {
        return match self.types.get(&name_) {
            None => { Err(ErrorCode::IdNotFound(format!("type {} not found", name_))) }
            Some(type_) => { Ok(type_.clone()) }
        };
    }

    fn _resolve_type_in_scope(target_scope: Arc<RwLock<Scope>>, name_: &String, allowed_relationships: &HashSet<ScopeRelationType>) -> Result<Arc<RwLock<DataType>>, ErrorCode> {
        let target_scope_r = target_scope.read().unwrap();

        //find self scope
        match target_scope_r.resolve_type_in_current_scope(name_.clone()) {
            Ok(type_) => { return Ok(type_) }
            Err(_) => {}
        }

        //find in parent scope
        for (_, scope_real) in &(target_scope_r.scope_relationships) {
            let result = Scope::_resolve_type_in_scope(scope_real.get_target_scope().clone(), &name_, &allowed_relationships);
            match result {
                Ok(type_) => {return Ok(type_)}
                Err(_) => {}
            }
        }

        return Err(ErrorCode::IdNotFound(format!("type {} not found", name_.clone())));
    }

    pub fn resolve_type_with_relation(& self, name_: String, allowed_relationships: Vec<ScopeRelationType>) -> Result<Arc<RwLock<DataType>>, ErrorCode> {
        match self.resolve_type_in_current_scope(name_.clone()) {
            Ok(type_) => { return Ok(type_) }
            Err(_) => {}
        }

        let mut allowed_relationships_hash = HashSet::new();
        for allowed_relationship in allowed_relationships {
            allowed_relationships_hash.insert(allowed_relationship.clone());
        }

        //find in parent scope
        for (_, scope_real) in &(self.scope_relationships) {
            let result = Scope::_resolve_type_in_scope(scope_real.get_target_scope().clone(), &name_, & allowed_relationships_hash);
            match result {
                Ok(type_) => {return Ok(type_)}
                Err(_) => {}
            }
        }

        return Err(ErrorCode::IdNotFound(format!("variable {} not found", name_.clone())));
    }

    pub fn resolve_type_from_scope(& self, name_: String) -> Result<Arc<RwLock<DataType>>, ErrorCode> {
        use crate::scope::ScopeRelationType::*;
        let allowed_relationships = vec![GroupScopeRela, UnionScopeRela,
                                         StreamScopeRela, StreamletScopeRela, ImplementScopeRela];
        return self.resolve_type_with_relation(name_, allowed_relationships);
    }

}