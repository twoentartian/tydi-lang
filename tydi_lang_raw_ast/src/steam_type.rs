use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use deep_clone::DeepClone;
use tydi_il::ToTydiIL;
use crate::logical_data_type::LogicalDataType;
use crate::error::ErrorCode;
use crate::{generate_access, generate_get, generate_set, generate_set_in_arc_rwlock, inferred, infer_logical_data_type};
use crate::scope::{PrettyPrint, Scope, ScopeType, TypeAlias};
use crate::util::generate_padding;
use crate::variable::Variable;
use crate::inferable::{NewInferable, Inferable};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LogicalStreamSynchronicity {
    Unknown(String),
    Sync,
    Flatten,
    Desync,
    FlatDesync,
}

impl DeepClone for LogicalStreamSynchronicity {
    fn deep_clone(&self) -> Self {
        return self.clone();
    }
}

impl From<LogicalStreamSynchronicity> for String {
    fn from(s: LogicalStreamSynchronicity) -> Self {
        match s {
            LogicalStreamSynchronicity::Unknown(s) => { format!("Unknown({})", s.clone()) }
            LogicalStreamSynchronicity::Sync => { String::from("Sync") }
            LogicalStreamSynchronicity::Flatten => { String::from("Flatten") }
            LogicalStreamSynchronicity::Desync => { String::from("Desync") }
            LogicalStreamSynchronicity::FlatDesync => { String::from("FlatDesync") }
        }
    }
}

impl From<String> for LogicalStreamSynchronicity {
    fn from(s: String) -> Self {
        if s == String::from("Sync") || s == String::from("sync") { return LogicalStreamSynchronicity::Sync; }
        else if s == String::from("Flatten") || s == String::from("flatten") { return LogicalStreamSynchronicity::Flatten; }
        else if s == String::from("Desync") || s == String::from("desync") { return LogicalStreamSynchronicity::Desync; }
        else if s == String::from("FlatDesync") || s == String::from("flatdesync") { return LogicalStreamSynchronicity::FlatDesync; }
        else { return LogicalStreamSynchronicity::Unknown(s.clone()); }
    }
}

impl PrettyPrint for LogicalStreamSynchronicity {
    fn pretty_print(&self, _: u32, _: bool) -> String {
        return String::from(self.clone());
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LogicalStreamDirection {
    Unknown(String),
    Forward,
    Reverse,
}

impl DeepClone for LogicalStreamDirection {
    fn deep_clone(&self) -> Self {
        return self.clone();
    }
}

impl From<LogicalStreamDirection> for String {
    fn from(d: LogicalStreamDirection) -> Self {
        match d {
            LogicalStreamDirection::Unknown(s) => { format!("Unknown({})", s.clone()) }
            LogicalStreamDirection::Forward => { String::from("Forward") }
            LogicalStreamDirection::Reverse => { String::from("Reverse") }
        }
    }
}

impl From<String> for LogicalStreamDirection {
    fn from(s: String) -> Self {
        if s == String::from("Forward") || s == String::from("forward") { return LogicalStreamDirection::Forward; }
        else if s == String::from("Reverse") || s == String::from("reverse") { return LogicalStreamDirection::Reverse; }
        else { return LogicalStreamDirection::Unknown(s.clone()); }
    }
}

impl PrettyPrint for LogicalStreamDirection {
    fn pretty_print(&self, _: u32, _: bool) -> String {
        return String::from(self.clone());
    }
}

#[derive(Clone, Debug)]
pub struct LogicalStream {
    name: String,

    data_type: Inferable<Arc<RwLock<LogicalDataType>>>,

    dimension: Arc<RwLock<Variable>>,
    user_type: Inferable<Arc<RwLock<LogicalDataType>>>,
    throughput: Arc<RwLock<Variable>>,
    synchronicity: LogicalStreamSynchronicity,
    complexity: Arc<RwLock<Variable>>,
    direction: LogicalStreamDirection,
    keep: Arc<RwLock<Variable>>,
}

impl DeepClone for LogicalStream {
    fn deep_clone(&self) -> Self {
        return Self {
            name: self.name.deep_clone(),
            data_type: self.data_type.deep_clone(),
            dimension: self.dimension.deep_clone(),
            user_type: self.user_type.deep_clone(),
            throughput: self.throughput.deep_clone(),
            synchronicity: self.synchronicity.deep_clone(),
            complexity: self.complexity.deep_clone(),
            direction: self.direction.deep_clone(),
            keep: self.keep.deep_clone(),
        }
    }
}

impl ToTydiIL for LogicalStream {
    fn to_tydi_il(&self, type_alias_map: &mut HashMap<String, (String, Vec<String>)>, depth:u32) -> String {
        let data_logical_type = self.data_type.get_raw_value().read().unwrap().to_tydi_il(type_alias_map, 1);
        let user_logical_type = self.user_type.get_raw_value().read().unwrap().to_tydi_il(type_alias_map, 1);

        let output_alias_map =
            format!("\
        Stream (\n\
        {}data: {},\n\
        {}throughput: {},\n\
        {}dimensionality: {},\n\
        {}synchronicity: {},\n\
        {}complexity: {},\n\
        {}direction: {},\n\
        {}user: {},\n\
        {}keep: {},\n\
        {})",
                    generate_padding(depth+1), data_logical_type.clone(),
                    generate_padding(depth+1), String::from((*self.throughput.read().unwrap()).clone()),
                    generate_padding(depth+1), String::from((*self.dimension.read().unwrap()).clone()),
                    generate_padding(depth+1), String::from(String::from(self.synchronicity.clone())),
                    generate_padding(depth+1), String::from((*self.complexity.read().unwrap()).clone()),
                    generate_padding(depth+1), String::from(String::from(self.direction.clone())),
                    generate_padding(depth+1), user_logical_type.clone(),
                    generate_padding(depth+1), String::from((*self.keep.read().unwrap()).clone()),
                    generate_padding(depth),
            );
        type_alias_map.insert(crate::util::rename_id_to_il(self.name.clone()), (output_alias_map, vec![data_logical_type, user_logical_type]));

        return crate::util::rename_id_to_il(self.name.clone());
    }
}

impl LogicalStream {
    generate_get!(name, String, get_name);
    generate_access!(data_type, Inferable<Arc<RwLock<LogicalDataType>>>, get_data_type, set_data_type);
    generate_get!(dimension, Arc<RwLock<Variable>>, get_dimension);generate_set_in_arc_rwlock!(dimension, Variable, set_dimension);
    generate_access!(user_type, Inferable<Arc<RwLock<LogicalDataType>>>, get_user_type, set_user_type);
    generate_get!(throughput, Arc<RwLock<Variable>>, get_throughput);generate_set_in_arc_rwlock!(throughput, Variable, set_throughput);
    generate_access!(synchronicity, LogicalStreamSynchronicity, get_synchronicity, set_synchronicity);
    generate_get!(complexity, Arc<RwLock<Variable>>, get_complexity);generate_set_in_arc_rwlock!(complexity, Variable, set_complexity);
    generate_access!(direction, LogicalStreamDirection, get_direction, set_direction);
    generate_get!(keep, Arc<RwLock<Variable>>, get_keep);generate_set_in_arc_rwlock!(keep, Variable, set_keep);

    pub fn new(name_: String, data_type_: Inferable<Arc<RwLock<LogicalDataType>>>) -> Self {
        let default_clone: Arc<RwLock<LogicalStream>> = DEFAULT_LOGICAL_STREAM.clone();
        let mut output = default_clone.read().unwrap().clone();
        output.name = name_.clone();
        output.data_type = data_type_.clone();
        return output;
    }

    pub fn new_raw() -> Self {
        let name = String::from("default");
        Self {
            name: name.clone(),
            data_type: inferred!(infer_logical_data_type!(), Arc::new(RwLock::new(LogicalDataType::DataNull))),
            dimension: Arc::new(RwLock::new(Variable::new_int(format!("!{{stream_{}}}_dimension", name.clone()), 0))),
            user_type: inferred!(infer_logical_data_type!(), Arc::new(RwLock::new(LogicalDataType::DataNull))),
            throughput: Arc::new(RwLock::new(Variable::new_float(format!("!{{stream_{}}}_throughput", name.clone()), 1.0))),
            synchronicity: LogicalStreamSynchronicity::Sync,
            complexity: Arc::new(RwLock::new(Variable::new_int(format!("!{{stream_{}}}_complexity", name.clone()), 7))),
            direction: LogicalStreamDirection::Forward,
            keep: Arc::new(RwLock::new(Variable::new_bool(format!("!{{stream_{}}}_keep", name.clone()), false))),
        }
    }
}

impl PrettyPrint for LogicalStream {
    fn pretty_print(&self, depth: u32, verbose: bool) -> String {
        let mut output = String::new();

        //enter union
        output.push_str(&format!("Stream({}){{\n", self.name.clone()));
        //enter union
        output.push_str(&format!("{}DataType={}\n", generate_padding(depth + 1), String::from(self.data_type.clone())));
        output.push_str(&format!("{}dimension={}, user={}, throughput={}, synchronicity={}, complexity={}, direction={}, keep={}\n", generate_padding(depth + 1),
                                 String::from( (*self.dimension.read().unwrap()).clone() ),
                                 String::from(self.user_type.clone()),
                                 String::from( (*self.throughput.read().unwrap()).clone() ), self.synchronicity.pretty_print(depth + 1, verbose),
                                 String::from( (*self.complexity.read().unwrap()).clone() ), self.direction.pretty_print(depth + 1, verbose),
                                 String::from( (*self.keep.read().unwrap()).clone() )));
        //leave union
        output.push_str(&format!("{}}}", generate_padding(depth)));
        return output;
    }
}

impl Scope {
    pub fn new_logical_stream(&mut self, name_: String, data_type_: Inferable<Arc<RwLock<LogicalDataType>>>) -> Result<(), ErrorCode> {
        if self.scope_type == ScopeType::StreamScope { return  Err(ErrorCode::ScopeNotAllowed(format!("not allowed to define group type in Stream scope"))); }
        if self.scope_type == ScopeType::StreamletScope { return Err(ErrorCode::ScopeNotAllowed(format!("not allowed to define group type in Streamlet scope"))); }
        if self.scope_type == ScopeType::ImplementScope { return Err(ErrorCode::ScopeNotAllowed(format!("not allowed to define group type in Implement scope"))); }

        match self.types.get(&name_) {
            None => {}
            Some(_) => { return Err(ErrorCode::IdRedefined(format!("type {} already defined", name_.clone()))); }
        };

        let logical_stream = LogicalStream::new(name_.clone(), data_type_.clone());
        let logical_stream = Arc::new(RwLock::new(LogicalDataType::DataStreamType(name_.clone(), Arc::new(RwLock::new(logical_stream)))));
        self.types.insert(name_.clone(), Arc::new(RwLock::new(TypeAlias::new(name_.clone(), inferred!(infer_logical_data_type!(), logical_stream))  )));

        return Ok(());
    }
}

lazy_static! {
    pub static ref DEFAULT_LOGICAL_STREAM: Arc<RwLock<LogicalStream>> = {
        let default = Arc::new(RwLock::new(LogicalStream::new_raw()));
        return default;
    };
}