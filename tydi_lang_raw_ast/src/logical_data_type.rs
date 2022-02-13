use std::sync::{Arc, RwLock};
use deep_clone::DeepClone;
use crate::bit_null_type::LogicalBit;
use crate::group_union_type::{LogicalGroup, LogicalUnion};
use crate::steam_type::LogicalStream;
use crate::util::{PrettyPrint};
use crate::scope::{ScopeType, ScopeRelationType, Scope};
use crate::type_alias::TypeAlias;
use crate::inferable::{NewInferable, Inferable};
use crate::{inferred, infer_logical_data_type};
use crate::error::ErrorCode;

#[derive(Clone, Debug)]
pub enum LogicalDataType {
    DummyLogicalData,
    UnknownLogicalDataType,
    ExternalLogicalDataType(String, String),

    DataNull,
    DataBitType(LogicalBit),
    DataGroupType(String, Arc<RwLock<LogicalGroup>>),
    DataUnionType(String, Arc<RwLock<LogicalUnion>>),
    DataStreamType(String, Arc<RwLock<LogicalStream>>),
    DataUserDefinedVarType(String),
}

impl DeepClone for LogicalDataType {
    fn deep_clone(&self) -> Self {
        return match self {
            LogicalDataType::DataBitType(logical_bit) => {
                LogicalDataType::DataBitType(logical_bit.deep_clone())
            }
            LogicalDataType::DataGroupType(name, target) => {
                LogicalDataType::DataGroupType(name.clone(), Arc::new(RwLock::new(target.read().unwrap().deep_clone())))
            }
            LogicalDataType::DataUnionType(name, target) => {
                LogicalDataType::DataUnionType(name.clone(), Arc::new(RwLock::new(target.read().unwrap().deep_clone())))
            }
            LogicalDataType::DataStreamType(name, target) => {
                LogicalDataType::DataStreamType(name.clone(), Arc::new(RwLock::new(target.read().unwrap().deep_clone())))
            }
            _ => { self.clone() }
        }
    }
}

impl PartialEq for LogicalDataType {
    fn eq(&self, other: &Self) -> bool {
        match self {
            LogicalDataType::DummyLogicalData => return true,
            LogicalDataType::UnknownLogicalDataType => {
                match other {
                    LogicalDataType::UnknownLogicalDataType => return true,
                    _ => return false,
                }
            },
            _ => { todo!() }
        }
    }
}

impl From<LogicalDataType> for String {
    fn from(logical_data_type: LogicalDataType) -> Self {
        return match logical_data_type {
            LogicalDataType::DummyLogicalData => { String::from("DummyLogicalData") }
            LogicalDataType::UnknownLogicalDataType => { String::from("UnknownLogicalDataType") }
            LogicalDataType::ExternalLogicalDataType(s1, s2) => { format!("ExternalLogicalDataType({}.{})", s1.clone(), s2.clone()) }
            LogicalDataType::DataNull => { String::from("DataNull") }
            LogicalDataType::DataBitType(v) => { format!("{}", String::from(v.clone())) }
            LogicalDataType::DataGroupType(name, _) => { format!("DataGroup({})", name) }
            LogicalDataType::DataUnionType(name, _) => { format!("DataUnion({})", name) }
            LogicalDataType::DataStreamType(name, _) => { format!("Stream({})", name.clone()) }
            LogicalDataType::DataUserDefinedVarType(name) => { format!("VarType({})", name.clone()) }
        }
    }
}

impl PrettyPrint for LogicalDataType {
    fn pretty_print(&self, depth: u32, verbose: bool) -> String {
        return match self {
            LogicalDataType::DummyLogicalData => { String::from(self.clone()) }
            LogicalDataType::UnknownLogicalDataType => { String::from(self.clone()) }
            LogicalDataType::ExternalLogicalDataType(_,_) => { String::from(self.clone()) }
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
            LogicalDataType::DataUserDefinedVarType(name) => { name.clone() }
        }
    }
}

impl Scope {
    pub fn new_external_type(&mut self, name_: String, package_name: String, exp_: String) -> Result<(), ErrorCode> {
        if self.scope_type == ScopeType::StreamScope { return Err(ErrorCode::ScopeNotAllowed(String::from("not allowed to define group type in Stream scope"))); }
        if self.scope_type == ScopeType::ImplementScope { return Err(ErrorCode::ScopeNotAllowed(String::from("not allowed to define group type in Implement scope"))); }
        if self.scope_type == ScopeType::IfForScope { return Err(ErrorCode::ScopeNotAllowed(String::from("not allowed to define group type in If/For scope"))); }
        match self.types.get(&name_) {
            None => {}
            Some(_) => { return Err(ErrorCode::IdRedefined(format!("type {} already defined", name_.clone()))); }
        };

        let data_type = LogicalDataType::ExternalLogicalDataType(package_name.clone(), exp_.clone());
        self.types.insert(name_.clone(), Arc::new(RwLock::new(TypeAlias::new(name_.clone(), inferred!(infer_logical_data_type!(), Arc::new(RwLock::new(data_type)))))));
        return Ok(());
    }

    pub fn new_logical_data_type(&mut self, name_:String, type_: LogicalDataType) -> Result<(), ErrorCode> {
        match self.types.get(&name_) {
            None => {}
            Some(_) => { return Err(ErrorCode::IdRedefined(format!("type {} already defined", name_.clone()))); }
        };

        match type_.clone() {
            LogicalDataType::DataGroupType(_, group) => {
                let group_scope = group.read().unwrap().get_scope();
                let parent_scope = self.self_ref.clone().unwrap();
                group_scope.write().unwrap().new_relationship_with_name(parent_scope.clone(), String::from("base"), ScopeRelationType::GroupScopeRela);
            },
            LogicalDataType::DataUnionType(_, union) => {
                let union_scope = union.read().unwrap().get_scope();
                let parent_scope = self.self_ref.clone().unwrap();
                union_scope.write().unwrap().new_relationship_with_name(parent_scope.clone(), String::from("base"), ScopeRelationType::GroupScopeRela);
            },

            _ => { },
        }

        self.types.insert(name_.clone(), Arc::new(RwLock::new(TypeAlias::new(name_.clone(), inferred!(infer_logical_data_type!(), Arc::new(RwLock::new(type_)))))));
        return Ok(());
    }
}