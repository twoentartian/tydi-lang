use std::process::Output;
use std::sync::{Arc, RwLock};
use crate::logical_data_type::LogicalDataType;
use crate::bit_null_type::LogicalNull;
use crate::data_type::DataType;
use crate::error::ErrorCode;
use crate::{generate_access, generate_get, generate_set};
use crate::scope::{PrettyPrint, Scope, ScopeType};
use crate::util::generate_padding;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LogicalStreamSynchronicity {
    Sync,
    Flatten,
    Desync,
    FlatDesync,
}

impl From<LogicalStreamSynchronicity> for String {
    fn from(s: LogicalStreamSynchronicity) -> Self {
        match s {
            LogicalStreamSynchronicity::Sync => { String::from("Sync") }
            LogicalStreamSynchronicity::Flatten => { String::from("Flatten") }
            LogicalStreamSynchronicity::Desync => { String::from("Desync") }
            LogicalStreamSynchronicity::FlatDesync => { String::from("FlatDesync") }
        }
    }
}

impl PrettyPrint for LogicalStreamSynchronicity {
    fn pretty_print(&self, depth: u32, verbose: bool) -> String {
        return String::from(self.clone());
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LogicalStreamDirection {
    Forward,
    Reverse,
}

impl From<LogicalStreamDirection> for String {
    fn from(d: LogicalStreamDirection) -> Self {
        match d {
            LogicalStreamDirection::Forward => { String::from("Forward") }
            LogicalStreamDirection::Reverse => { String::from("Reverse") }
        }
    }
}

impl PrettyPrint for LogicalStreamDirection {
    fn pretty_print(&self, depth: u32, verbose: bool) -> String {
        return String::from(self.clone());
    }
}

#[derive(Clone, Debug)]
pub struct LogicalStream {
    name: String,

    data_type: Arc<RwLock<LogicalDataType>>,

    dimension: u32,
    user_type: Arc<RwLock<LogicalDataType>>,
    throughput: f32,
    synchronicity: LogicalStreamSynchronicity,
    complexity: u32,
    direction: LogicalStreamDirection,
    keep: bool,
}

impl LogicalStream {
    generate_get!(name, String, get_name);
    generate_access!(data_type, Arc<RwLock<LogicalDataType>>, get_data_type, set_data_type);
    generate_access!(dimension, u32, get_dimension, set_dimension);
    generate_access!(user_type, Arc<RwLock<LogicalDataType>>, get_user_type, set_user_type);
    generate_access!(throughput, f32, get_throughput, set_throughput);
    generate_access!(synchronicity, LogicalStreamSynchronicity, get_synchronicity, set_synchronicity);
    generate_access!(complexity, u32, get_complexity, set_complexity);
    generate_access!(direction, LogicalStreamDirection, get_direction, set_direction);
    generate_access!(keep, bool, get_keep, set_keep);

    pub fn new(name_: String, data_type_: Arc<RwLock<LogicalDataType>>) -> Self {
        let default_clone: Arc<RwLock<LogicalStream>> = DefaultLogicalStream.clone();
        let mut output = default_clone.read().unwrap().clone();
        output.name = name_.clone();
        output.data_type = data_type_.clone();
        return output;
    }

    pub fn new_raw() -> Self {
        Self {
            name: String::from("default"),
            data_type: Arc::new(RwLock::new(LogicalDataType::DataNull)),
            dimension: 0,
            user_type: Arc::new(RwLock::new(LogicalDataType::DataNull)),
            throughput: 1.0,
            synchronicity: LogicalStreamSynchronicity::Sync,
            complexity: 7,
            direction: LogicalStreamDirection::Forward,
            keep: false,
        }
    }
}

impl PrettyPrint for LogicalStream {
    fn pretty_print(&self, depth: u32, verbose: bool) -> String {
        let mut output = String::new();

        //enter union
        output.push_str(&format!("Stream({}){{\n", self.name.clone()));
        //enter union
        output.push_str(&format!("{}DataType={}\n", generate_padding(depth + 1), String::from((*self.data_type.read().unwrap()).clone())));
        output.push_str(&format!("{}dimension={}, user={}, throughput={}, synchronicity={}, complexity={}, direction={}, keep={}\n", generate_padding(depth + 1),
                                 self.dimension.clone(),
                                 String::from((*self.user_type.read().unwrap()).clone()),
                                 self.throughput, self.synchronicity.pretty_print(depth + 1, verbose),
                                 self.complexity, self.direction.pretty_print(depth + 1, verbose),
                                 self.keep));
        //leave union
        output.push_str(&format!("{}}}", generate_padding(depth)));
        return output;
    }
}

impl Scope {
    pub fn new_logical_stream(&mut self, name_: String, data_type_: Arc<RwLock<LogicalDataType>>) -> Result<(), ErrorCode> {
        if self.scope_type == ScopeType::StreamScope { panic!("not allowed to define group type in Stream scope") }
        if self.scope_type == ScopeType::StreamletScope { panic!("not allowed to define group type in Streamlet scope") }
        if self.scope_type == ScopeType::ImplementScope { panic!("not allowed to define group type in Implement scope") }

        match self.types.get(&name_) {
            None => {}
            Some(_) => { return Err(ErrorCode::IdRedefined(format!("type {} already defined", name_.clone()))); }
        };

        let logical_stream = LogicalStream::new(name_.clone(), data_type_.clone());
        let logical_stream = DataType::LogicalDataType(Arc::new(RwLock::new(LogicalDataType::DataStreamType(name_.clone(), Arc::new(RwLock::new(logical_stream))))));
        self.types.insert(name_.clone(), Arc::new(RwLock::new(logical_stream)));

        return Ok(());
    }
}

lazy_static! {
    pub static ref DefaultLogicalStream: Arc<RwLock<LogicalStream>> = {
        let mut default = Arc::new(RwLock::new(LogicalStream::new_raw()));
        return default;
    };
}