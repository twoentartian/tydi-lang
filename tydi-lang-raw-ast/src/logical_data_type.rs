use std::sync::Arc;
use crate::scope_var::{Scope, DataType};

#[derive(Clone, Debug)]
pub enum LogicalDataType {
    EmptyLogicalData,
    DataNull,
    DataBitType(u32),
    DataGroupType(Arc<Scope>, String),
    DataUnionType(Arc<Scope>, String),
    DataStreamType(Arc<DataType>, Arc<Scope>),
}

impl From<LogicalDataType> for String {
    fn from(logical_data_type: LogicalDataType) -> Self {
        match logical_data_type {
            LogicalDataType::EmptyLogicalData => { return String::from("EmptyLogicalData"); }
            LogicalDataType::DataNull => { return String::from("DataNull"); }
            LogicalDataType::DataBitType(i) => { return format!("Bit({})", i); }
            LogicalDataType::DataGroupType(_, name) => { return format!("DataGroup({})", name); }
            LogicalDataType::DataUnionType(_, name) => { return format!("DataUnion({})", name); }
            LogicalDataType::DataStreamType(data_type, _) => { return format!("Stream({})", String::from((*data_type).clone())); }
        }
    }
}