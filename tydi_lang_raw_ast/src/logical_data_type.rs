use std::sync::{Arc, RwLock};
use deep_clone::DeepClone;
use scope::HashMap;
use tydi_il::ToTydiIL;
use crate::bit_null_type::LogicalBit;
use crate::group_union_type::{LogicalGroup, LogicalUnion};
use crate::steam_type::LogicalStream;
use crate::util::{PrettyPrint};
use crate::scope::{ScopeType, ScopeRelationType, Scope};
use crate::type_alias::TypeAlias;
use crate::inferable::{NewInferable, Inferable};
use crate::{inferred, infer_logical_data_type};
use crate::error::ErrorCode;
use crate::variable::Variable;

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
    DataUDVInStreamlet(String, Vec<Arc<RwLock<Variable>>>, String),
    DataUDVInImplement(String, Vec<Arc<RwLock<Variable>>>, String),
}

impl ToTydiIL for LogicalDataType {
    fn to_tydi_il(&self, type_alias_map: &mut HashMap<String, (String, Vec<String>)>, depth: u32) -> String {
        match self {
            LogicalDataType::DataNull => { return format!("Null"); }
            LogicalDataType::DataBitType(bit) => {
                return format!("Bits({})", String::from((*bit.get_bit().read().unwrap()).clone()));
            }
            LogicalDataType::DataGroupType(_, target) => {
                return target.read().unwrap().to_tydi_il(type_alias_map, depth);
            }
            LogicalDataType::DataUnionType(_, target) => {
                return target.read().unwrap().to_tydi_il(type_alias_map, depth);
            }
            LogicalDataType::DataStreamType(_, target) => {
                return target.read().unwrap().to_tydi_il(type_alias_map, depth);
            }
            _ => unreachable!()
        }
    }
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
    // this eq is for strict comparison
    fn eq(&self, other: &Self) -> bool {
        return match self {
            LogicalDataType::DummyLogicalData => true,
            LogicalDataType::UnknownLogicalDataType => {
                match other {
                    LogicalDataType::UnknownLogicalDataType => true,
                    _ => false,
                }
            },
            LogicalDataType::ExternalLogicalDataType(package, type_name) => {
                match other {
                    LogicalDataType::ExternalLogicalDataType(other_package, other_type_name) => {
                        other_package == package && other_type_name == type_name
                    },
                    _ => false,
                }
            }
            LogicalDataType::DataNull => {
                match other {
                    LogicalDataType::DataNull => true,
                    _ => false,
                }
            }
            LogicalDataType::DataBitType(bit) => {
                match other {
                    LogicalDataType::DataBitType(other_bit) => bit == other_bit,
                    _ => false,
                }
            }
            LogicalDataType::DataGroupType(name, ptr1) => {
                match other {
                    LogicalDataType::DataGroupType(other_name, ptr2) => name == other_name && std::sync::Arc::ptr_eq(ptr1, ptr2),
                    _ => false,
                }
            }
            LogicalDataType::DataUnionType(name, ptr1) => {
                match other {
                    LogicalDataType::DataUnionType(other_name, ptr2) => name == other_name && std::sync::Arc::ptr_eq(ptr1, ptr2),
                    _ => false,
                }
            }
            LogicalDataType::DataStreamType(name, _) => {
                match other {
                    LogicalDataType::DataStreamType(other_name, _) => name == other_name,
                    _ => false,
                }
            }
            LogicalDataType::DataUserDefinedVarType(type_name) => {
                match other {
                    LogicalDataType::DataUserDefinedVarType(other_type_name) => type_name == other_type_name,
                    _ => false,
                }
            }
            LogicalDataType::DataUDVInStreamlet(streamlet_name,_ , type_name) => {
                match other {
                    //warning: we don't compare the template arg expressions of the streamlet
                    LogicalDataType::DataUDVInStreamlet(other_streamlet_name,_ , other_type_name) => streamlet_name == other_streamlet_name && type_name == other_type_name,
                    _ => false,
                }
            }
            LogicalDataType::DataUDVInImplement(streamlet_name,_ , type_name) => {
                match other {
                    //warning: we don't compare the template arg expressions of the implement
                    LogicalDataType::DataUDVInImplement(other_streamlet_name,_ , other_type_name) => streamlet_name == other_streamlet_name && type_name == other_type_name,
                    _ => false,
                }
            }
        }
    }
}

impl LogicalDataType {
    pub fn non_strict_eq(&self, other: &Self) -> bool {
        match self {
            LogicalDataType::DataNull => {
                return if *other == LogicalDataType::DataNull { true } else { false }
            },
            LogicalDataType::DataBitType(i) => {
                return match other {
                    LogicalDataType::DataBitType(i_other) => {
                        i.compare_value(i_other)
                    }
                    _ => false
                }
            },
            LogicalDataType::DataGroupType(_, group) => {
                match other {
                    LogicalDataType::DataGroupType(_, other_group) => {
                        let group_scope = group.read().unwrap().get_scope();
                        let types = group_scope.read().unwrap().types.clone();
                        let other_group_scope = other_group.read().unwrap().get_scope();
                        let other_types = other_group_scope.read().unwrap().types.clone();
                        for (name, single_type) in types {
                            let type_in_other = other_types.get(&name);
                            if type_in_other.is_none() { return false; }
                            let type_in_other = type_in_other.unwrap();
                            let type_in_other = type_in_other.read().unwrap().get_type_infer().get_raw_value();
                            let type_in_other = type_in_other.read().unwrap().deep_clone();

                            let type_self = single_type.read().unwrap().get_type_infer().get_raw_value();

                            if !type_self.read().unwrap().non_strict_eq(&type_in_other) {
                                return false;
                            }
                        }
                        return true;
                    }
                    _ => return false
                }
            },
            LogicalDataType::DataUnionType(_, union) => {
                match other {
                    LogicalDataType::DataUnionType(_, other_union) => {
                        let union_scope = union.read().unwrap().get_scope();
                        let types = union_scope.read().unwrap().types.clone();
                        let other_union_scope = other_union.read().unwrap().get_scope();
                        let other_types = other_union_scope.read().unwrap().types.clone();
                        for (name, single_type) in types {
                            let type_in_other = other_types.get(&name);
                            if type_in_other.is_none() { return false; }
                            let type_in_other = type_in_other.unwrap();
                            let type_in_other = type_in_other.read().unwrap().get_type_infer().get_raw_value();
                            let type_in_other = type_in_other.read().unwrap().deep_clone();

                            let type_self = single_type.read().unwrap().get_type_infer().get_raw_value();

                            if !type_self.read().unwrap().non_strict_eq(&type_in_other) {
                                return false;
                            }
                        }
                        return true;
                    }
                    _ => return false
                }
            },
            LogicalDataType::DataStreamType(_, stream) => {
                match other {
                    LogicalDataType::DataStreamType(_, other_stream) => {
                        let other_stream = other_stream.read().unwrap().deep_clone();
                        if !stream.read().unwrap().eq(&other_stream, false) { return false; }
                        return true;
                    }
                    _ => return false
                }
            },
            _ => unreachable!()
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
            LogicalDataType::DataUDVInStreamlet(streamlet_name,_ , name) => { format!("VarType(streamlet[{}].{})", streamlet_name.clone(), name.clone()) }
            LogicalDataType::DataUDVInImplement(impl_name,_ , name) => { format!("VarType(impl[{}].{})", impl_name.clone(), name.clone()) }
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
            LogicalDataType::DataUserDefinedVarType(_) => { String::from(self.clone()) }
            LogicalDataType::DataUDVInStreamlet(_,_,_) => { String::from(self.clone()) }
            LogicalDataType::DataUDVInImplement(_,_,_) => { String::from(self.clone()) }
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

    pub fn with_type_alias(&mut self, type_: Arc<RwLock<TypeAlias>>) -> Result<(), ErrorCode> {
        let name_ = type_.read().unwrap().get_name();
        match self.types.get(&name_) {
            None => {}
            Some(_) => { return Err(ErrorCode::IdRedefined(format!("type {} already defined", name_.clone()))); }
        };

        self.types.insert(name_.clone(), type_.clone());
        return Ok(());
    }
}