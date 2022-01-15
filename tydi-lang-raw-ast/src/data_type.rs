use std::sync::{Arc, RwLock};
use crate::logical_data_type::LogicalDataType;
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
    fn pretty_print(&self, depth: u32) -> String {
        return String::from(self.clone());
    }
}