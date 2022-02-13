use std::sync::{Arc, RwLock};
use deep_clone::DeepClone;
use crate::logical_data_type::LogicalDataType;
use crate::{generate_get, generate_set, generate_access, infer_logical_data_type, not_inferred};
use crate::port::PortDirection;
use crate::scope::Port;
use crate::streamlet::{Streamlet,StreamletType};
use crate::util::PrettyPrint;
use crate::variable::VariableValue;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InferState {
    Inferred,
    NotInferred,
}

impl DeepClone for InferState {
    fn deep_clone(&self) -> Self {
        return self.clone();
    }
}

#[derive(Clone, Debug)]
pub struct Inferable<T> where T: Clone + DeepClone {
    raw_exp: String,

    infer_state: InferState,
    raw_value: T,
}

impl<T> DeepClone for Inferable<T> where T: Clone + DeepClone {
    fn deep_clone(&self) -> Self {
        return Self {
            raw_exp: self.raw_exp.deep_clone(),
            infer_state: self.infer_state.deep_clone(),
            raw_value: self.raw_value.deep_clone(),
        }
    }
}

impl<T> Inferable<T> where T: Clone + DeepClone {
    generate_get!(raw_exp, String, get_raw_exp);
    generate_access!(infer_state, InferState, get_infer_state, set_infer_state);
    generate_access!(raw_value, T, get_raw_value, set_raw_value);
}

pub trait NewInferable<T> {
    fn _new(exp: String) -> Self;

    fn _new_inferred(exp: String, type_: T) -> Self;
}

#[macro_export]
macro_rules! inferred {
    ($t: ty, $value: expr) => {
        <Inferable<$t> as NewInferable<$t>>::_new_inferred(String::from(""), $value.clone())
    }
}

#[macro_export]
macro_rules! not_inferred {
    ($t: ty, $value: expr) => {
        <Inferable<$t> as NewInferable<$t>>::_new(String::from($value))
    }
}

/// NewInferable trait implementation
impl NewInferable<Arc<RwLock<LogicalDataType>>> for Inferable<Arc<RwLock<LogicalDataType>>> {
    fn _new(exp: String) -> Self {
        Self {
            raw_exp: exp,
            infer_state: InferState::NotInferred,
            raw_value: Arc::new(RwLock::new(LogicalDataType::DummyLogicalData)),
        }
    }

    fn _new_inferred(exp: String, type_: Arc<RwLock<LogicalDataType>>) -> Self {
        Self {
            raw_exp: exp,
            infer_state: InferState::Inferred,
            raw_value: type_.clone(),
        }
    }
}

impl NewInferable<VariableValue> for Inferable<VariableValue> {
    fn _new(exp: String) -> Self {
        Self {
            raw_exp: exp,
            infer_state: InferState::NotInferred,
            raw_value: VariableValue::Unknown,
        }
    }

    fn _new_inferred(exp: String, value_: VariableValue) -> Self {
        Self {
            raw_exp: exp,
            infer_state: InferState::Inferred,
            raw_value: value_,
        }
    }
}

impl NewInferable<Arc<RwLock<Streamlet>>> for Inferable<Arc<RwLock<Streamlet>>> {
    fn _new(exp: String) -> Self {
        Self {
            raw_exp: exp,
            infer_state: InferState::NotInferred,
            raw_value: Arc::new(RwLock::new(Streamlet::new(String::from(""), StreamletType::UnknownType))),
        }
    }

    fn _new_inferred(exp: String, type_: Arc<RwLock<Streamlet>>) -> Self {
        Self {
            raw_exp: exp,
            infer_state: InferState::Inferred,
            raw_value: type_.clone(),
        }
    }
}

impl NewInferable<Arc<RwLock<Port>>> for Inferable<Arc<RwLock<Port>>> {
    fn _new(exp: String) -> Self {
        Self {
            raw_exp: exp,
            infer_state: InferState::NotInferred,
            raw_value: Arc::new(RwLock::new(Port::new(String::from(""), not_inferred!(infer_logical_data_type!(), String::from("")), PortDirection::Unknown))),
        }
    }

    fn _new_inferred(exp: String, type_: Arc<RwLock<Port>>) -> Self {
        Self {
            raw_exp: exp,
            infer_state: InferState::Inferred,
            raw_value: type_.clone(),
        }
    }
}

/// Inferable struct implementation
macro_rules! inferable_new_wrapper {
    ($t: ty) => {
        impl Inferable<$t> {
            pub fn new(exp: String) -> Self {
                return Self::_new(exp);
            }
            pub fn new_inferred(exp: String, type_: $t) -> Self {
                return Self::_new_inferred(exp, type_);
            }
        }
    };
}

inferable_new_wrapper!(Arc<RwLock<LogicalDataType>>);
inferable_new_wrapper!(VariableValue);
inferable_new_wrapper!(Arc<RwLock<Streamlet>>);

/// From Inferable to String
impl<T> From< Inferable<Arc<RwLock<T>>> > for String where T: Clone + DeepClone, String: From<T> {
    fn from(t: Inferable<Arc<RwLock<T>>>) -> Self {
        return match t.infer_state {
            InferState::Inferred => { String::from( (*t.raw_value.read().unwrap()).clone()) }
            InferState::NotInferred => { format!("NotInferred(\"{}\")", t.raw_exp.clone()) }
        }
    }
}

impl From< Inferable<VariableValue> > for String {
    fn from(t: Inferable<VariableValue>) -> Self {
        return match t.infer_state {
            InferState::Inferred => { String::from( t.raw_value.clone()) }
            InferState::NotInferred => { format!("NotInferred(\"{}\")", t.raw_exp.clone()) }
        }
    }
}

/// PrettyPrint for Inferable
impl<T> PrettyPrint for Inferable<Arc<RwLock<T>>> where T: PrettyPrint + DeepClone + Clone {
    fn pretty_print(&self, depth: u32, verbose: bool) -> String {
        return match self.infer_state {
            InferState::Inferred => { self.raw_value.read().unwrap().pretty_print(depth, verbose) }
            InferState::NotInferred => { format!("NotInferred(\"{}\")", self.raw_exp.clone()) }
        }
    }
}

impl PrettyPrint for Inferable<VariableValue> {
    fn pretty_print(&self, depth: u32, verbose: bool) -> String {
        return match self.infer_state {
            InferState::Inferred => { self.raw_value.pretty_print(depth, verbose) }
            InferState::NotInferred => { format!("NotInferred(\"{}\")", self.raw_exp.clone()) }
        }
    }
}

#[macro_export]
macro_rules! infer_logical_data_type {
    () => {
        Arc<RwLock<LogicalDataType>>
    }
}

#[macro_export]
macro_rules! infer_variable_value {
    () => {
        VariableValue
    }
}

#[macro_export]
macro_rules! infer_streamlet {
    () => {
        Arc<RwLock<Streamlet>>
    }
}

#[macro_export]
macro_rules! infer_port {
    () => {
        Arc<RwLock<Port>>
    }
}
