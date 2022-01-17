use std::sync::{Arc, RwLock};
use crate::data_type::DataType;
use crate::error::ErrorCode;
use crate::generate_get;
use crate::logical_data_type::LogicalDataType;
use crate::scope::{Scope, ScopeType};

pub struct LogicalNull {
    name: String,
}

impl LogicalNull {
    generate_get!(name, String, get_name);

    pub fn new(name_: String) -> Self {
        Self {
            name: name_.clone(),
        }
    }
}

pub struct LogicalBit {
    name: String,
    bit: u32,
}

impl LogicalBit {
    generate_get!(name, String, get_name);
    generate_get!(bit, u32, get_bit);

    pub fn new(name_: String, bit_: u32) -> Self {
        Self {
            name: name_.clone(),
            bit: bit_,
        }
    }
}

impl Scope {
    pub fn new_logical_null(&mut self, name_: String) -> Result<(), ErrorCode> {
        if self.scope_type == ScopeType::StreamScope { panic!("not allowed to define group type in Stream scope") }
        if self.scope_type == ScopeType::StreamletScope { panic!("not allowed to define group type in Streamlet scope") }
        if self.scope_type == ScopeType::ImplementScope { panic!("not allowed to define group type in Implement scope") }

        match self.types.get(&name_) {
            None => {}
            Some(_) => { return Err(ErrorCode::IdRedefined(format!("type {} already defined", name_.clone()))); }
        };

        let logical_null = DataType::LogicalDataType(Arc::new(RwLock::new(LogicalDataType::DataNull)));
        self.types.insert(name_.clone(), Arc::new(RwLock::new(logical_null)));
        return Ok(());
    }

    pub fn new_logical_bit(&mut self, name_: String, bit_: u32) -> Result<(), ErrorCode> {
        if self.scope_type == ScopeType::StreamScope { panic!("not allowed to define group type in Stream scope") }
        if self.scope_type == ScopeType::StreamletScope { panic!("not allowed to define group type in Streamlet scope") }
        if self.scope_type == ScopeType::ImplementScope { panic!("not allowed to define group type in Implement scope") }

        match self.types.get(&name_) {
            None => {}
            Some(_) => { return Err(ErrorCode::IdRedefined(format!("type {} already defined", name_.clone()))); }
        };

        let logical_bit = DataType::LogicalDataType(Arc::new(RwLock::new(LogicalDataType::DataBitType(bit_))));
        self.types.insert(name_.clone(), Arc::new(RwLock::new(logical_bit)));
        return Ok(());
    }
}