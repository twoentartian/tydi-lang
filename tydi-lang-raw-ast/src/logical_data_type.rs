use std::sync::{Arc, RwLock};
use crate::group_union_type::{LogicalGroup, LogicalUnion};
use crate::scope::{Scope, DataType};
use crate::steam_type::LogicalStream;
use crate::util::{generate_padding, PrettyPrint};

#[derive(Clone, Debug)]
pub enum LogicalDataType {
    EmptyLogicalData,
    DataNull,
    DataBitType(u32),
    DataGroupType(String, Arc<RwLock<LogicalGroup>>),
    DataUnionType(String, Arc<RwLock<LogicalUnion>>),
    DataStreamType(String, Arc<RwLock<LogicalStream>>),
}

impl From<LogicalDataType> for String {
    fn from(logical_data_type: LogicalDataType) -> Self {
        match logical_data_type {
            LogicalDataType::EmptyLogicalData => { return String::from("EmptyLogicalData"); }
            LogicalDataType::DataNull => { return String::from("DataNull"); }
            LogicalDataType::DataBitType(i) => { return format!("Bit({})", i); }
            LogicalDataType::DataGroupType(name, _) => { return format!("DataGroup({})", name); }
            LogicalDataType::DataUnionType(name, _) => { return format!("DataUnion({})", name); }
            LogicalDataType::DataStreamType(name, _) => { return format!("Stream({})", name.clone()); }
        }
    }
}

impl PrettyPrint for LogicalDataType {
    fn pretty_print(&self, depth: u32, verbose: bool) -> String {
        match self {
            LogicalDataType::EmptyLogicalData => { return String::from(self.clone()); }
            LogicalDataType::DataNull => { return String::from(self.clone()); }
            LogicalDataType::DataBitType(_) => { return String::from(self.clone()); }
            LogicalDataType::DataGroupType(_, group) => {
                return group.read().unwrap().pretty_print(depth, verbose);
            }
            LogicalDataType::DataUnionType(_, union) => {
                return union.read().unwrap().pretty_print(depth, verbose);
            }
            LogicalDataType::DataStreamType(_, stream) => {
                return stream.read().unwrap().pretty_print(depth, verbose);
            }
        }
    }
}