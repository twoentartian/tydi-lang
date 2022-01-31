use std::collections::HashSet;
use std::sync::{Arc, RwLock};
use crate::scope::Implement;
use crate::streamlet::Streamlet;
use crate::error::ErrorCode;
use crate::logical_data_type::LogicalDataType;
use crate::scope::{Scope, ScopeRelationType, TypeAlias};
use crate::util::PrettyPrint;
use crate::variable::Variable;

#[derive(Clone, Debug)]
pub enum DataType {
    UnknownType,  /// reserved Unknown type
    UnableToInfer,  /// reserved UnableToInfer type, means currently the type is unknown, but might be inferred in the future. Eg, template.
    PackageType,

    IntType,
    StringType,
    BoolType,
    FloatType,
    ArrayType(Arc<RwLock<DataType>>),

    EmptyLogicalDataType,
    LogicalDataType(Arc<RwLock<LogicalDataType>>),

    ExternalProxyType(String, Arc<RwLock<LogicalDataType>>),

    ProxyStreamlet(String, Vec<Arc<RwLock<Variable>>>),
    ExternalProxyStreamlet(String, String, Vec<Arc<RwLock<Variable>>>),

    ProxyImplement(String, Vec<Arc<RwLock<Variable>>>),
    ExternalProxyImplement(String, String, Vec<Arc<RwLock<Variable>>>),

    ProxyImplementOfStreamlet(String, Vec<Arc<RwLock<Variable>>>),
    ExternalProxyImplementOfStreamlet(String, String, Vec<Arc<RwLock<Variable>>>),
}

impl From<DataType> for String {
    fn from(t: DataType) -> Self {
        return match t {
            DataType::UnknownType => { String::from("UnknownType") }
            DataType::UnableToInfer => { String::from("UnableToInfer") }
            DataType::PackageType => { String::from("PackageType") }
            DataType::IntType => { String::from("int") }
            DataType::StringType => { String::from("string") }
            DataType::BoolType => { String::from("bool") }
            DataType::FloatType => { String::from("float") }
            DataType::ArrayType(inner_data_type) => {
                let inner_type = inner_data_type.read().unwrap();
                let inner_type_str = String::from((*inner_type).clone());
                format!("array({})", inner_type_str)
            }
            DataType::EmptyLogicalDataType => { String::from("EmptyLogicalData") }
            DataType::LogicalDataType(logical_data_type) => {
                let inner_type = logical_data_type.read().unwrap();
                let inner_type_str = String::from((*inner_type).clone());
                format!("LogicalDataType({})", inner_type_str)
            }

            DataType::ExternalProxyType(name,type_) => { format!("ExternalProxyType({}.{})", name.clone(), String::from((*type_.read().unwrap()).clone())) }

            DataType::ProxyStreamlet(name,vars) => {
                let mut vars_string = String::from("");
                for var in vars {
                    vars_string.push_str(&format!("@{}", String::from(var.read().unwrap().get_var_value().get_raw_exp())) );
                }
                format!("ProxyStreamlet({}<{}>)", name, vars_string)
            }
            DataType::ExternalProxyStreamlet(package,name,vars) => {
                let mut vars_string = String::from("");
                for var in vars {
                    vars_string.push_str(&format!("@{}", String::from(var.read().unwrap().get_var_value().get_raw_exp())) );
                }
                format!("ProxyStreamlet({}.{}<{}>)", package, name, vars_string)
            }

            DataType::ProxyImplement(name, vars) => {
                let mut vars_string = String::from("");
                for var in vars {
                    vars_string.push_str(&format!("@{}", String::from(var.read().unwrap().get_var_value().get_raw_exp())) );
                }
                format!("ProxyImplement({}<{}>)", name, vars_string)
            }
            DataType::ExternalProxyImplement(package, name, vars) => {
                let mut vars_string = String::from("");
                for var in vars {
                    vars_string.push_str(&format!("@{}", String::from(var.read().unwrap().get_var_value().get_raw_exp())) );
                }
                format!("ExternalProxyImplement({}.{}<{}>)", package, name, vars_string)
            }

            DataType::ProxyImplementOfStreamlet(implement, vars) => {
                let mut vars_string = String::from("");
                for var in vars {
                    vars_string.push_str(&format!("@{}", String::from(var.read().unwrap().get_var_value().get_raw_exp())) );
                }
                format!("ProxyImplementOfStreamlet({}<{}>)", implement.clone(), vars_string)
            }
            DataType::ExternalProxyImplementOfStreamlet(package, implement, vars) => {
                let mut vars_string = String::from("");
                for var in vars {
                    vars_string.push_str(&format!("@{}", String::from(var.read().unwrap().get_var_value().get_raw_exp())) );
                }
                format!("ProxyImplementOfStreamlet({}.{}<{}>)", package.clone(), implement.clone(), vars_string)
            }
        }
    }
}

impl PrettyPrint for DataType {
    fn pretty_print(&self, depth: u32, verbose: bool) -> String {
        return match self {
            DataType::ArrayType(inner_data_type) => {
                let inner_type = inner_data_type.read().unwrap();
                let inner_type_str = inner_type.pretty_print(depth, verbose);
                format!("array({})", inner_type_str)
            }
            DataType::EmptyLogicalDataType => { String::from(self.clone()) }
            DataType::LogicalDataType(logical_data_type) => {
                let logical_data_type = logical_data_type.read().unwrap();
                let output = logical_data_type.pretty_print(depth, verbose);
                output
            }
            _ => { String::from(self.clone()) }
        }
    }
}

impl Scope {
    pub fn resolve_type_in_current_scope(& self, name_: String) -> Result<Arc<RwLock<TypeAlias>>, ErrorCode> {
        return match self.types.get(&name_) {
            None => { Err(ErrorCode::IdNotFound(format!("type {} not found", name_))) }
            Some(type_) => { Ok(type_.clone()) }
        };
    }

    fn _resolve_type_in_scope(target_scope: Arc<RwLock<Scope>>, name_: &String, allowed_relationships: &HashSet<ScopeRelationType>) -> Result<Arc<RwLock<TypeAlias>>, ErrorCode> {
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

    pub fn resolve_type_with_relation(& self, name_: String, allowed_relationships: Vec<ScopeRelationType>) -> Result<Arc<RwLock<TypeAlias>>, ErrorCode> {
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

    pub fn resolve_type_from_scope(& self, name_: String) -> Result<Arc<RwLock<TypeAlias>>, ErrorCode> {
        use crate::scope::ScopeRelationType::*;
        let allowed_relationships = vec![GroupScopeRela, UnionScopeRela,
                                         StreamScopeRela, StreamletScopeRela, ImplementScopeRela];
        return self.resolve_type_with_relation(name_, allowed_relationships);
    }

}