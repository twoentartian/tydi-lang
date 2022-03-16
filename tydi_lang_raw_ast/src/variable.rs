use std::sync::{Arc, RwLock};
use std::collections::HashSet;
use deep_clone::DeepClone;
use implement::Implement;
use logical_data_type::LogicalDataType;
use streamlet::Streamlet;
use crate::scope::{DataType, ScopeRelationType, Scope};
use crate::util::{generate_padding, PrettyPrint};
use crate::{generate_get, generate_set, generate_access};
pub use crate::error::ErrorCode;
use crate::inferable::{Inferable, NewInferable};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum ClockDomainValue {
    Unknown,
    Default,
    ClockDomain(String),
}

impl From<ClockDomainValue> for String {
    fn from(cd: ClockDomainValue) -> Self {
        match cd {
            ClockDomainValue::Unknown => { format!("UnknownClockDomain") }
            ClockDomainValue::Default => { format!("DefaultClockDomain") }
            ClockDomainValue::ClockDomain(str) => { format!("ClockDomain({})", str) }
        }
    }
}

impl DeepClone for ClockDomainValue {
    fn deep_clone(&self) -> Self {
        return self.clone();
    }
}

#[derive(Clone, Debug)]
pub enum VariableValue {
    Unknown,
    Int(i32),
    Bool(bool),
    Float(f32),
    Str(String),
    ClockDomain(ClockDomainValue),
    ArrayInt(Vec<i32>),
    ArrayBool(Vec<bool>),
    ArrayFloat(Vec<f32>),
    ArrayStr(Vec<String>),

    LogicalDataType(Arc<RwLock<LogicalDataType>>),
    Streamlet(Arc<RwLock<Streamlet>>),
    Implement(Arc<RwLock<Implement>>),
}

impl PartialEq for VariableValue {
    fn eq(&self, other: &Self) -> bool {
        match self {
            VariableValue::Unknown => {
                return match other {
                    VariableValue::Unknown => { true }
                    _ => { false }
                }
            }
            VariableValue::Int(v0) => {
                return match other {
                    VariableValue::Int(v1) => { v0 == v1 }
                    _ => { false }
                }
            }
            VariableValue::Bool(v0) => {
                return match other {
                    VariableValue::Bool(v1) => { v0 == v1 }
                    _ => { false }
                }
            }
            VariableValue::Float(v0) => {
                return match other {
                    VariableValue::Float(v1) => { v0 == v1 }
                    _ => { false }
                }
            }
            VariableValue::Str(v0) => {
                return match other {
                    VariableValue::Str(v1) => { v0 == v1 }
                    _ => { false }
                }
            }
            VariableValue::ClockDomain(v0) => {
                return match other {
                    VariableValue::ClockDomain(v1) => { v0 == v1 }
                    _ => { false }
                }
            }
            VariableValue::ArrayInt(v0) => {
                return match other {
                    VariableValue::ArrayInt(v1) => { v0 == v1 }
                    _ => { false }
                }
            }
            VariableValue::ArrayBool(v0) => {
                return match other {
                    VariableValue::ArrayBool(v1) => { v0 == v1 }
                    _ => { false }
                }
            }
            VariableValue::ArrayFloat(v0) => {
                return match other {
                    VariableValue::ArrayFloat(v1) => { v0 == v1 }
                    _ => { false }
                }
            }
            VariableValue::ArrayStr(v0) => {
                return match other {
                    VariableValue::ArrayStr(v1) => { v0 == v1 }
                    _ => { false }
                }
            }
            _ => todo!()
        }
    }
}

impl DeepClone for VariableValue {
    fn deep_clone(&self) -> Self {
        return self.clone();
    }
}

impl From<VariableValue> for String {
    fn from(v: VariableValue) -> Self {
        return match v {
            VariableValue::Unknown => { format!("Unknown") }
            VariableValue::Int(v) => { format!("{}", v) }
            VariableValue::Bool(v) => { format!("{}", v) }
            VariableValue::Float(v) => { format!("{}", v) }
            VariableValue::Str(v) => { format!("{}", v) }
            VariableValue::ClockDomain(v) => { String::from(v) }
            VariableValue::ArrayInt(v) => {
                let mut output = String::from("");
                if v.len() == 0 { output = String::from(""); }
                else if v.len() == 1 { output = format!("{}", v[0]); }
                else {
                    output.push_str(&format!("{}", v[0]));
                    for i in 1 .. v.len() {
                        output.push_str(&format!(",{}", v[i]));
                    }
                }
                format!("{{{}}}", output)
            },
            VariableValue::ArrayBool(v) => {
                let mut output = String::from("");
                if v.len() == 0 { output = String::from(""); }
                else if v.len() == 1 { output = format!("{}", v[0]); }
                else {
                    output.push_str(&format!("{}", v[0]));
                    for i in 1 .. v.len() {
                        output.push_str(&format!(",{}", v[i]));
                    }
                }
                format!("{{{}}}", output)
            },
            VariableValue::ArrayFloat(v) => {
                let mut output = String::from("");
                if v.len() == 0 { output = String::from(""); }
                else if v.len() == 1 { output = format!("{}", v[0]); }
                else {
                    output.push_str(&format!("{}", v[0]));
                    for i in 1 .. v.len() {
                        output.push_str(&format!(",{}", v[i]));
                    }
                }
                format!("{{{}}}", output)
            },
            VariableValue::ArrayStr(v) => {
                let mut output = String::from("");
                if v.len() == 0 { output = String::from(""); }
                else if v.len() == 1 { output = format!("{}", v[0]); }
                else {
                    output.push_str(&format!("{}", v[0]));
                    for i in 1 .. v.len() {
                        output.push_str(&format!(",{}", v[i]));
                    }
                }
                format!("{{{}}}", output)
            },

            VariableValue::LogicalDataType(logical_data) => {
                format!("LogicalData({})", String::from((*logical_data.read().unwrap()).clone()))
            },
            VariableValue::Streamlet(streamlet) => {
                format!("Streamlet({})", String::from((*streamlet.read().unwrap()).clone()))
            },
            VariableValue::Implement(implement) => {
                format!("Implement({})", String::from((*implement.read().unwrap()).clone()))
            },
        }
    }
}

impl PrettyPrint for VariableValue {
    fn pretty_print(&self, _: u32, _: bool) -> String {
        return String::from(self.clone());
    }
}

impl VariableValue {
    pub fn try_convert_to(&self, target_type: &DataType) -> Result<Self, String> {
        let target;
        match self {
            VariableValue::Str(cd_name) => {
                match target_type {
                    DataType::ClockDomainType => {
                        target = VariableValue::ClockDomain(ClockDomainValue::ClockDomain(cd_name.clone()));
                        return Ok(target);
                    }
                    _ => { }
                }
            }
            _ => { }
        };

        return Err(format!("fail to convert from {} to {}", String::from(self.clone()), String::from(target_type.clone())))
    }

    pub fn try_convert_to_in_place(&self, target: &mut Self, target_type: &DataType) -> Result<(), String> {
        match self {
            VariableValue::Str(cd_name) => {
                match target_type {
                    DataType::ClockDomainType => {
                        *target = VariableValue::ClockDomain(ClockDomainValue::ClockDomain(cd_name.clone()));
                        return Ok(());
                    }
                    _ => { }
                }
            }
            _ => { }
        };

        return Err(format!("fail to convert from {} to {}", String::from(self.clone()), String::from(target.clone())))
    }
}

#[derive(Clone, Debug)]
pub struct Variable {
    name: String,
    var_type: Arc<RwLock<DataType>>,

    var_value: Inferable<VariableValue>,
}

impl DeepClone for Variable {
    fn deep_clone(&self) -> Self {
        return Self {
            name: self.name.clone(),
            var_type: self.var_type.deep_clone(),
            var_value: self.var_value.deep_clone(),
        }
    }
}

impl Variable {
    generate_get!(name, String, get_name);
    generate_access!(var_type, Arc<RwLock<DataType>>, get_type, set_type);
    generate_access!(var_value, Inferable<VariableValue>, get_var_value, set_var_value);

    pub fn new(name_: String, type_: DataType, exp_: String) -> Self {
        Self {
            name: name_.clone(),
            var_type: Arc::new(RwLock::new(type_)),
            var_value: <Inferable<VariableValue> as NewInferable<VariableValue>>::_new(exp_.clone()),
        }
    }

    pub fn new_empty() -> Self {
        Self {
            name: String::from(""),
            var_type: Arc::new(RwLock::new(DataType::UnknownType)),
            var_value: <Inferable<VariableValue> as NewInferable<VariableValue>>::_new(String::from("")),
        }
    }

    pub fn new_with_value(name_: String, type_: DataType, value: VariableValue) -> Self {
        Self {
            name: name_.clone(),
            var_type: Arc::new(RwLock::new(type_)),
            var_value: <Inferable<VariableValue> as NewInferable<VariableValue>>::_new_inferred(name_.clone(),value),
        }
    }

    pub fn new_int(name_: String, v: i32) -> Self {
        Self {
            name: name_.clone(),
            var_type: Arc::new(RwLock::new(DataType::IntType)),
            var_value: <Inferable<VariableValue> as NewInferable<VariableValue>>::_new_inferred(name_.clone(), VariableValue::Int(v)),
        }
    }

    pub fn new_float(name_: String, v: f32) -> Self {
        Self {
            name: name_.clone(),
            var_type: Arc::new(RwLock::new(DataType::FloatType)),
            var_value: <Inferable<VariableValue> as NewInferable<VariableValue>>::_new_inferred(name_.clone(), VariableValue::Float(v)),
        }
    }

    pub fn new_bool(name_: String, v: bool) -> Self {
        Self {
            name: name_.clone(),
            var_type: Arc::new(RwLock::new(DataType::BoolType)),
            var_value: <Inferable<VariableValue> as NewInferable<VariableValue>>::_new_inferred(name_.clone(), VariableValue::Bool(v)),
        }
    }

    pub fn new_str(name_: String, v: String) -> Self {
        Self {
            name: name_.clone(),
            var_type: Arc::new(RwLock::new(DataType::StringType)),
            var_value: <Inferable<VariableValue> as NewInferable<VariableValue>>::_new_inferred(name_.clone(), VariableValue::Str(v)),
        }
    }

    pub fn new_int_array(name_: String, v: Vec<i32>) -> Self {
        Self {
            name: name_.clone(),
            var_type: Arc::new(RwLock::new(DataType::ArrayType(Arc::new(RwLock::new(DataType::IntType))))),
            var_value: <Inferable<VariableValue> as NewInferable<VariableValue>>::_new_inferred(name_.clone(), VariableValue::ArrayInt(v)),
        }
    }

    pub fn new_float_array(name_: String, v: Vec<f32>) -> Self {
        Self {
            name: name_.clone(),
            var_type: Arc::new(RwLock::new(DataType::ArrayType(Arc::new(RwLock::new(DataType::FloatType))))),
            var_value: <Inferable<VariableValue> as NewInferable<VariableValue>>::_new_inferred(name_.clone(), VariableValue::ArrayFloat(v)),
        }
    }

    pub fn new_bool_array(name_: String, v: Vec<bool>) -> Self {
        Self {
            name: name_.clone(),
            var_type: Arc::new(RwLock::new(DataType::ArrayType(Arc::new(RwLock::new(DataType::BoolType))))),
            var_value: <Inferable<VariableValue> as NewInferable<VariableValue>>::_new_inferred(name_.clone(), VariableValue::ArrayBool(v)),
        }
    }

    pub fn new_str_array(name_: String, v: Vec<String>) -> Self {
        Self {
            name: name_.clone(),
            var_type: Arc::new(RwLock::new(DataType::ArrayType(Arc::new(RwLock::new(DataType::StringType))))),
            var_value: <Inferable<VariableValue> as NewInferable<VariableValue>>::_new_inferred(name_.clone(), VariableValue::ArrayStr(v)),
        }
    }

    pub fn clear_infer_result(&mut self) {
        use inferable::InferState;
        self.var_value.set_infer_state(InferState::NotInferred);
    }

}

impl From<Variable> for String {
    fn from(v: Variable) -> Self {
        return String::from(v.var_value);
    }
}

impl PrettyPrint for Variable {
    fn pretty_print(&self, depth: u32, verbose: bool) -> String {
        return format!("{}{}:{}({})", generate_padding(depth), self.name.clone(), self.var_type.read().unwrap().pretty_print(depth, verbose), String::from(self.var_value.clone()) );
    }
}

impl Scope {
    pub fn new_variable(&mut self, name_: String, type_: DataType, exp_: String) -> Result<(), ErrorCode> {
        match self.vars.get(&name_) {
            None => {}
            Some(_) => { return Err(ErrorCode::IdRedefined(format!("variable {} already defined", name_))); }
        };
        self.vars.insert(name_.clone(), Arc::new(RwLock::new(Variable::new(name_.clone(), type_.clone(), exp_.clone()))));
        return Ok(());
    }

    pub fn with_variable(&mut self, var: Arc<RwLock<Variable>>) -> Result<(), ErrorCode> {
        let name_ = var.read().unwrap().get_name();
        match self.vars.get(&name_) {
            None => {}
            Some(_) => { return Err(ErrorCode::IdRedefined(format!("variable {} already defined", name_))); }
        };
        self.vars.insert(name_.clone(), var.clone());
        return Ok(());
    }

    pub fn resolve_variable_in_current_scope(& self, name_: String) -> Result<Arc<RwLock<Variable>>, ErrorCode> {
        return match self.vars.get(&name_) {
            None => { Err(ErrorCode::IdNotFound(format!("variable {} not found", name_))) }
            Some(var) => { Ok(var.clone()) }
        };
    }

    fn _resolve_var_in_scope(target_scope: Arc<RwLock<Scope>>, name_: &String, allowed_relationships: &HashSet<ScopeRelationType>) -> Result<Arc<RwLock<Variable>>, ErrorCode> {
        let target_scope_r = target_scope.read().unwrap();

        //find self scope
        match target_scope_r.resolve_variable_in_current_scope(name_.clone()) {
            Ok(var) => { return Ok(var) }
            Err(_) => {}
        }

        //find in parent scope
        for (_, scope_real) in &(target_scope_r.scope_relationships) {
            let result = Scope::_resolve_var_in_scope(scope_real.get_target_scope().clone(), &name_, &allowed_relationships);
            match result {
                Ok(var) => {return Ok(var)}
                Err(_) => {}
            }
        }

        return Err(ErrorCode::IdNotFound(format!("variable {} not found", name_.clone())));
    }

    pub fn resolve_variable_with_relation(& self, name_: String, allowed_relationships: Vec<ScopeRelationType>) -> Result<Arc<RwLock<Variable>>, ErrorCode> {
        match self.resolve_variable_in_current_scope(name_.clone()) {
            Ok(var) => { return Ok(var) }
            Err(_) => {}
        }

        let mut allowed_relationships_hash = HashSet::new();
        for allowed_relationship in allowed_relationships {
            allowed_relationships_hash.insert(allowed_relationship.clone());
        }

        //find in parent scope
        for (_, scope_real) in &(self.scope_relationships) {
            let result = Scope::_resolve_var_in_scope(scope_real.get_target_scope().clone(), &name_, & allowed_relationships_hash);
            match result {
                Ok(var) => {return Ok(var)}
                Err(_) => {}
            }
        }

        return Err(ErrorCode::IdNotFound(format!("variable {} not found", name_.clone())));
    }

    pub fn resolve_variable_from_scope(& self, name_: String) -> Result<Arc<RwLock<Variable>>, ErrorCode> {
        use crate::scope::ScopeRelationType::*;
        let allowed_relationships = vec![GroupScopeRela, UnionScopeRela,
                                         StreamScopeRela, StreamletScopeRela, ImplementScopeRela];
        return self.resolve_variable_with_relation(name_, allowed_relationships);
    }
}