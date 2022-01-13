pub use std::collections::BTreeMap;
use crate::id::Id;
use crate::identifier::Identifier;

use crate::scope_var::{Scope, TypeProxy};

/// region: LogicalDataType
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum LogicalDataType {
    EmptyLogicalData,
    DataNull,
    DataBitType(u32),
    DataGroupType(Id<Scope>, String),
    DataUnionType(Id<Scope>, String),
    DataStreamType(Id<TypeProxy>, Id<Scope>),
}