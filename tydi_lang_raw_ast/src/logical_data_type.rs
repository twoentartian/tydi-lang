use std::sync::{Arc, RwLock};
use crate::bit_null_type::LogicalBit;
use crate::group_union_type::{LogicalGroup, LogicalUnion};
use crate::steam_type::LogicalStream;
use crate::util::{PrettyPrint};

#[derive(Clone, Debug)]
pub enum LogicalDataType {
    EmptyLogicalData,
    DataNull,
    DataBitType(LogicalBit),
    DataGroupType(String, Arc<RwLock<LogicalGroup>>),
    DataUnionType(String, Arc<RwLock<LogicalUnion>>),
    DataStreamType(String, Arc<RwLock<LogicalStream>>),
    DataVar(String),
}

impl From<LogicalDataType> for String {
    fn from(logical_data_type: LogicalDataType) -> Self {
        return match logical_data_type {
            LogicalDataType::EmptyLogicalData => { String::from("EmptyLogicalData") }
            LogicalDataType::DataNull => { String::from("DataNull") }
            LogicalDataType::DataBitType(v) => { format!("{}", String::from(v.clone())) }
            LogicalDataType::DataGroupType(name, _) => { format!("DataGroup({})", name) }
            LogicalDataType::DataUnionType(name, _) => { format!("DataUnion({})", name) }
            LogicalDataType::DataStreamType(name, _) => { format!("Stream({})", name.clone()) }
            LogicalDataType::DataVar(name) => { format!("Var({})", name.clone()) }
        }
    }
}

impl PrettyPrint for LogicalDataType {
    fn pretty_print(&self, depth: u32, verbose: bool) -> String {
        return match self {
            LogicalDataType::EmptyLogicalData => { String::from(self.clone()) }
            LogicalDataType::DataNull => { String::from(self.clone()) }
            LogicalDataType::DataBitType(_) => { String::from(self.clone()) }
            LogicalDataType::DataGroupType(_, group) => {
                group.read().unwrap().pretty_print(depth, verbose)
            }
            LogicalDataType::DataUnionType(_, union) => {
                union.read().unwrap().pretty_print(depth, verbose)
            }
            LogicalDataType::DataStreamType(_, stream) => {
                stream.read().unwrap().pretty_print(depth, verbose)
            }
            LogicalDataType::DataVar(name) => { name.clone() }
        }
    }
}
