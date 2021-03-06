use std::collections::HashSet;
use std::sync::{Arc, RwLock};
use deep_clone::DeepClone;
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
    ClockDomainType,
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

impl DeepClone for DataType {
    fn deep_clone(&self) -> Self {
        return match self {
            DataType::ArrayType(datatype) => { DataType::ArrayType(datatype.deep_clone()) }
            DataType::LogicalDataType(logical_type) => { DataType::LogicalDataType(logical_type.deep_clone()) }
            DataType::ExternalProxyType(package, proxy) => { DataType::ExternalProxyType(package.deep_clone(), proxy.deep_clone()) }
            DataType::ProxyStreamlet(name, proxy) => { DataType::ProxyStreamlet(name.deep_clone(), proxy.deep_clone()) }
            DataType::ExternalProxyStreamlet(package, name, proxy) => { DataType::ExternalProxyStreamlet(package.deep_clone(), name.deep_clone(), proxy.deep_clone()) }
            DataType::ProxyImplement(name, proxy) => { DataType::ProxyImplement(name.deep_clone(), proxy.deep_clone()) }
            DataType::ExternalProxyImplement(package, name, proxy) => { DataType::ExternalProxyImplement(package.deep_clone(), name.deep_clone(), proxy.deep_clone()) }
            DataType::ProxyImplementOfStreamlet(name, proxy) => { DataType::ProxyImplementOfStreamlet(name.deep_clone(), proxy.deep_clone()) }
            DataType::ExternalProxyImplementOfStreamlet(package, name, proxy) => { DataType::ExternalProxyImplementOfStreamlet(package.deep_clone(), name.deep_clone(), proxy.deep_clone()) }
            _ => return self.clone(),
        }
    }
}

impl DataType {
    pub fn is_sub_type_of_other(&self, other: &Self) -> bool {
        if *self == DataType::UnknownType { return true; }
        if *self == DataType::ClockDomainType && *other == DataType::StringType { return true; }
        return false;
    }
}

impl DataType {
    pub fn compatible(&self, other: &Self) -> bool {
        if *self == *other { return true; }
        if *self == DataType::UnknownType || *other == DataType::UnknownType { return true; }

        return false;
    }
}

impl PartialEq for DataType {
    fn eq(&self, other: &Self) -> bool {
        match self {
            DataType::UnknownType => {
                return match other {
                    DataType::UnknownType => true,
                    _ => false,
                }
            },
            DataType::UnableToInfer => {
                return match other {
                    DataType::UnableToInfer => true,
                    _ => false,
                }
            },
            DataType::PackageType => {
                return match other {
                    DataType::PackageType => true,
                    _ => false,
                }
            },

            DataType::IntType => {
                return match other {
                    DataType::IntType => true,
                    _ => false,
                }
            },
            DataType::StringType => {
                return match other {
                    DataType::StringType => true,
                    _ => false,
                }
            },
            DataType::BoolType => {
                return match other {
                    DataType::BoolType => true,
                    _ => false,
                }
            },
            DataType::FloatType => {
                return match other {
                    DataType::FloatType => true,
                    _ => false,
                }
            },
            DataType::ClockDomainType => {
                return match other {
                    DataType::ClockDomainType => true,
                    _ => false,
                }
            },

            DataType::ArrayType(t0) => {
                return match other {
                    DataType::ArrayType(t1) => {
                        let t0 = (*(t0.read().unwrap())).clone();
                        let t1 = (*(t1.read().unwrap())).clone();
                        t0 == t1
                    },
                    _ => false,
                }
            },
            DataType::EmptyLogicalDataType => {
                return match other {
                    DataType::EmptyLogicalDataType => true,
                    _ => false,
                }
            },
            DataType::LogicalDataType(t0) => {
                return match other {
                    DataType::LogicalDataType(t1) => {
                        let t0 = (*(t0.read().unwrap())).clone();
                        let t1 = (*(t1.read().unwrap())).clone();
                        t0 == t1
                    },
                    _ => false,
                }
            },


            DataType::ProxyImplementOfStreamlet(_,_) | DataType::ExternalProxyImplementOfStreamlet(_,_,_) => {
                return match other {
                    DataType::ProxyImplement(_,_) => true,
                    DataType::ExternalProxyImplement(_,_,_) => true,
                    _ => false,
                }
            },
            DataType::ProxyImplement(_,_) | DataType::ExternalProxyImplement(_,_,_) => {
                return match other {
                    DataType::ProxyImplementOfStreamlet(_,_) => true,
                    DataType::ExternalProxyImplementOfStreamlet(_,_,_) => true,
                    _ => false,
                }
            }
            _ => { false }
        }
    }
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
            DataType::ClockDomainType => { String::from("clockdomain") }
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
                format!("[{}]", inner_type_str)
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
            if !allowed_relationships.contains(&scope_real.get_relationship()) { continue }
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
            if !allowed_relationships_hash.contains(&scope_real.get_relationship()) { continue }
            let result = Scope::_resolve_type_in_scope(scope_real.get_target_scope().clone(), &name_, & allowed_relationships_hash);
            match result {
                Ok(type_) => {return Ok(type_)}
                Err(_) => {}
            }
        }

        return Err(ErrorCode::IdNotFound(format!("type {} not found", name_.clone())));
    }

    pub fn resolve_type_from_scope(& self, name_: String) -> Result<Arc<RwLock<TypeAlias>>, ErrorCode> {
        use crate::scope::ScopeRelationType::*;
        let allowed_relationships = vec![GroupScopeRela, UnionScopeRela,
                                         StreamScopeRela, StreamletScopeRela, ImplementScopeRela];
        return self.resolve_type_with_relation(name_, allowed_relationships);
    }

}