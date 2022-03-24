use std::sync::{Arc, RwLock};
use deep_clone::DeepClone;
use crate::data_type::DataType;
use crate::error::ErrorCode;
use crate::{generate_get, generate_access, generate_set, inferred, infer_logical_data_type};
use crate::logical_data_type::LogicalDataType;
use crate::scope::{Scope, ScopeType};
use crate::type_alias::TypeAlias;
use crate::variable::{Variable};
use crate::inferable::{Inferable, NewInferable};

#[derive(Clone, Debug)]
pub struct LogicalNull {
    name: String,
}

impl DeepClone for LogicalNull {
    fn deep_clone(&self) -> Self {
        return self.clone();
    }
}

impl LogicalNull {
    generate_get!(name, String, get_name);

    pub fn new(name_: String) -> Self {
        Self {
            name: name_.deep_clone(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct LogicalBit {
    name: String,
    bit: Arc<RwLock<Variable>>,
}

impl PartialEq for LogicalBit {
    fn eq(&self, other: &Self) -> bool {
        return self.bit.read().unwrap().get_var_value().get_raw_exp() == other.bit.read().unwrap().get_var_value().get_raw_exp();
    }
}

impl DeepClone for LogicalBit {
    fn deep_clone(&self) -> Self {
        return Self {
            name: self.name.deep_clone(),
            bit: self.bit.deep_clone(),
        }
    }
}

impl LogicalBit {
    generate_get!(name, String, get_name);
    generate_access!(bit, Arc<RwLock<Variable>>, get_bit, set_bit);

    pub fn new(name_: String, exp_: String) -> Self {
        Self {
            name: name_.clone(),
            bit: Arc::new(RwLock::new(Variable::new(format!("!{{bit_int}}_{}", name_.clone()), DataType::IntType, exp_))),
        }
    }

    pub fn new_with_definite(name_: String, bit_: i64) -> Self {
        Self {
            name: name_.clone(),
            bit: Arc::new(RwLock::new(Variable::new_int(format!("!{{bit_int}}_{}", name_.clone()), bit_))),
        }
    }

    pub fn compare_value(&self, other: &Self) -> bool {
        let self_value = self.bit.read().unwrap().get_var_value().get_raw_value();
        let other_value = other.bit.read().unwrap().get_var_value().get_raw_value();
        return self_value == other_value;
    }
}

impl From<LogicalBit> for String {
    fn from(t: LogicalBit) -> Self {
        return format!("Bit({})", String::from((*t.bit.read().unwrap()).clone()));
    }
}

impl Scope {
    pub fn new_logical_null(&mut self, name_: String) -> Result<(), ErrorCode> {
        if self.scope_type == ScopeType::StreamScope { return Err(ErrorCode::ScopeNotAllowed(String::from("not allowed to define group type in Stream scope"))); }
        if self.scope_type == ScopeType::ImplementScope { return Err(ErrorCode::ScopeNotAllowed(String::from("not allowed to define group type in Implement scope"))); }
        if self.scope_type == ScopeType::IfForScope { return Err(ErrorCode::ScopeNotAllowed(String::from("not allowed to define group type in If/For scope"))); }

        match self.types.get(&name_) {
            None => {}
            Some(_) => { return Err(ErrorCode::IdRedefined(format!("type {} already defined", name_.clone()))); }
        };

        let logical_null = Arc::new(RwLock::new(LogicalDataType::DataNull));
        self.types.insert(name_.clone(), Arc::new(RwLock::new(TypeAlias::new(name_.clone(), inferred!(infer_logical_data_type!(), logical_null)))));
        return Ok(());
    }

    pub fn new_logical_bit(&mut self, name_: String, exp_: String) -> Result<(), ErrorCode> {
        if self.scope_type == ScopeType::StreamScope { return Err(ErrorCode::ScopeNotAllowed(String::from("not allowed to define logical bit in Stream scope"))); }
        if self.scope_type == ScopeType::StreamletScope { return Err(ErrorCode::ScopeNotAllowed(String::from("not allowed to define logical bit in Streamlet scope"))); }
        if self.scope_type == ScopeType::ImplementScope { return Err(ErrorCode::ScopeNotAllowed(String::from("not allowed to define logical bit in Implement scope"))); }

        match self.types.get(&name_) {
            None => {}
            Some(_) => { return Err(ErrorCode::IdRedefined(format!("type {} already defined", name_.clone()))); }
        };

        let logical_bit = LogicalBit::new(format!("!{{logical_bit}}_{}", name_.clone()), exp_.clone());
        let logical_bit = Arc::new(RwLock::new(LogicalDataType::DataBitType(logical_bit)));
        self.types.insert(name_.clone(), Arc::new(RwLock::new(TypeAlias::new(name_.clone(), inferred!(infer_logical_data_type!(), logical_bit)))));
        return Ok(());
    }

    pub fn new_logical_bit_with_definite(&mut self, name_: String, bit: i64) -> Result<(), ErrorCode> {
        if self.scope_type == ScopeType::StreamScope { return Err(ErrorCode::ScopeNotAllowed(String::from("not allowed to define group type in Stream scope"))); }
        if self.scope_type == ScopeType::StreamletScope { return Err(ErrorCode::ScopeNotAllowed(String::from("not allowed to define group type in Streamlet scope"))); }
        if self.scope_type == ScopeType::ImplementScope { return Err(ErrorCode::ScopeNotAllowed(String::from("not allowed to define group type in Implement scope"))); }

        match self.types.get(&name_) {
            None => {}
            Some(_) => { return Err(ErrorCode::IdRedefined(format!("type {} already defined", name_.clone()))); }
        };

        let logical_bit = LogicalBit::new_with_definite(format!("!{{logical_bit}}_{}", name_.clone()), bit);
        let logical_bit = Arc::new(RwLock::new(LogicalDataType::DataBitType(logical_bit)));
        self.types.insert(name_.clone(), Arc::new(RwLock::new(TypeAlias::new(name_.clone(), inferred!(infer_logical_data_type!(), logical_bit)))));
        return Ok(());
    }
}