use std::sync::{Arc, RwLock};
use deep_clone::DeepClone;
use scope::HashMap;
use tydi_il::ToTydiIL;
use crate::data_type::DataType;
use crate::error::ErrorCode;
use crate::{generate_get, inferred};
use crate::scope::{Scope, ScopeRelationType, ScopeType, TypeAlias};
use crate::util::{generate_padding, PrettyPrint};
use crate::inferable::{Inferable, NewInferable};
use crate::logical_data_type::LogicalDataType;

#[derive(Clone, Debug)]
pub struct LogicalGroup {
    name: String,

    scope: Arc<RwLock<Scope>>,
}

impl DeepClone for LogicalGroup {
    fn deep_clone(&self) -> Self {
        let output = Self {
            name: self.name.deep_clone(),
            scope: self.scope.deep_clone(),
        };
        {
            output.scope.write().unwrap().set_self_ref(output.scope.clone());
        }
        return output;
    }
}

impl ToTydiIL for LogicalGroup {
    fn to_tydi_il(&self, type_alias_map: &mut HashMap<String, String>, _:u32) -> String {
        let mut output_alias_map = String::from("");
        let types_in_group = self.scope.read().unwrap().types.clone();
        let mut first = true;
        for (name, type_) in types_in_group {
            if first {
                first = false;
            }
            else {
                output_alias_map.push_str(",");
            }

            let logical_type = type_.read().unwrap().get_type_infer().get_raw_value();
            let logical_type = (*logical_type.read().unwrap()).clone();
            output_alias_map.push_str(&format!("{}: {}", name, logical_type.to_tydi_il(type_alias_map, 1)));
        }
        let output_alias_map = format!("Group({})", output_alias_map);

        type_alias_map.insert(crate::util::rename_id_to_il(self.name.clone()), output_alias_map);

        return crate::util::rename_id_to_il(self.name.clone());
    }
}

impl LogicalGroup {
    generate_get!(name, String, get_name);
    generate_get!(scope, Arc<RwLock<Scope>>, get_scope);

    pub fn set_name(&mut self, name_: String) {
        self.name = name_.clone();
        self.scope.write().unwrap().set_name(format!("group_{}", name_.clone()));
    }

    pub fn new(name_: String) -> Self {
        let scope_ = Arc::new(RwLock::new(Scope::new(format!("group_{}", name_.clone()), ScopeType::GroupScope)));
        {
            scope_.write().unwrap().set_self_ref(scope_.clone());
        }
        Self {
            name: name_.clone(),
            scope: scope_,
        }
    }
}

impl From<LogicalGroup> for String {
    fn from(group: LogicalGroup) -> Self {
        return format!("DataGroup({})", group.get_name());
    }
}

impl PrettyPrint for LogicalGroup {
    fn pretty_print(&self, depth: u32, verbose: bool) -> String {
        let mut output = String::new();

        //enter group
        output.push_str(&format!("DataGroup({}){{\n", self.name.clone()));
        //enter scope
        output.push_str(&format!("{}", self.scope.read().unwrap().pretty_print(depth+1, verbose)));
        //leave group
        output.push_str(&format!("{}}}", generate_padding(depth)));

        return output;
    }
}

#[derive(Clone, Debug)]
pub struct LogicalUnion {
    name: String,

    scope: Arc<RwLock<Scope>>,
}

impl DeepClone for LogicalUnion {
    fn deep_clone(&self) -> Self {
        let output = Self {
            name: self.name.deep_clone(),
            scope: self.scope.deep_clone(),
        };
        {
            output.scope.write().unwrap().set_self_ref(output.scope.clone());
        }
        return output;
    }
}

impl ToTydiIL for LogicalUnion {
    fn to_tydi_il(&self, type_alias_map: &mut HashMap<String, String>, _:u32) -> String {
        let mut output_alias_map = String::from("");
        let types_in_union = self.scope.read().unwrap().types.clone();
        let mut first = true;
        for (name, type_) in types_in_union {
            if first {
                first = false;
            }
            else {
                output_alias_map.push_str(",");
            }

            let logical_type = type_.read().unwrap().get_type_infer().get_raw_value();
            let logical_type = (*logical_type.read().unwrap()).clone();
            output_alias_map.push_str(&format!("{}: {}", name, logical_type.to_tydi_il(type_alias_map, 1)));
        }
        let output_alias_map = format!("Union({})", output_alias_map);

        type_alias_map.insert(crate::util::rename_id_to_il(self.name.clone()), output_alias_map);

        return crate::util::rename_id_to_il(self.name.clone());
    }
}

impl LogicalUnion {
    generate_get!(name, String, get_name);
    generate_get!(scope, Arc<RwLock<Scope>>, get_scope);

    pub fn set_name(&mut self, name_: String) {
        self.name = name_.clone();
        self.scope.write().unwrap().set_name(format!("union_{}", name_.clone()));
    }

    pub fn new(name_: String) -> Self {
        let scope_ = Arc::new(RwLock::new(Scope::new(format!("union_{}", name_.clone()), ScopeType::UnionScope)));
        {
            scope_.write().unwrap().set_self_ref(scope_.clone());
        }
        Self {
            name: name_.clone(),
            scope: scope_,
        }
    }

    pub fn new_variable(& self, name_: String, type_: DataType, exp_: String) -> Result<(), ErrorCode> {
        return self.scope.write().unwrap().new_variable(name_.clone(), type_.clone(), exp_.clone());
    }

}

impl From<LogicalUnion> for String {
    fn from(union: LogicalUnion) -> Self {
        return format!("DataUnion({})", union.get_name());
    }
}

impl PrettyPrint for LogicalUnion {
    fn pretty_print(&self, depth: u32, verbose: bool) -> String {
        let mut output = String::new();

        //enter union
        output.push_str(&format!("DataUnion({}){{\n", self.name.clone()));
        //enter union
        output.push_str(&format!("{}", self.scope.read().unwrap().pretty_print(depth+1, verbose)));
        //leave union
        output.push_str(&format!("{}}}", generate_padding(depth)));

        return output;
    }
}

impl Scope {
    pub fn new_logical_group(&mut self, name_: String) -> Result<Arc<RwLock<Scope>>, ErrorCode> {
        if self.scope_type != ScopeType::BasicScope { return Err(ErrorCode::ScopeNotAllowed(String::from("not allowed to define group type outside of base scope"))); }

        match self.types.get(&name_) {
            None => {}
            Some(_) => { return Err(ErrorCode::IdRedefined(format!("type {} already defined", name_.clone()))); }
        };

        let logical_group = LogicalGroup::new(name_.clone());
        {
            let parent_scope = self.self_ref.clone().unwrap();
            logical_group.scope.write().unwrap().new_relationship_with_name(parent_scope.clone(), String::from("base"), ScopeRelationType::GroupScopeRela);
        }

        let scope_clone = logical_group.scope.clone();
        let group_data_type = Arc::new(RwLock::new(LogicalDataType::DataGroupType(name_.clone(), Arc::new(RwLock::new(logical_group)))));
        self.types.insert(name_.clone(), Arc::new(RwLock::new( TypeAlias::new(format!("!{{union_type}}_{}", name_.clone()), inferred!(Arc<RwLock<LogicalDataType>>, group_data_type)))));
        return Ok(scope_clone);
    }

    pub fn new_logical_union(&mut self, name_: String) -> Result<Arc<RwLock<Scope>>, ErrorCode> {
        if self.scope_type != ScopeType::BasicScope { return Err(ErrorCode::ScopeNotAllowed(String::from("not allowed to define union type outside of base scope"))); }

        match self.types.get(&name_) {
            None => {}
            Some(_) => { return Err(ErrorCode::IdRedefined(format!("type {} already defined", name_.clone()))); }
        };

        let logical_union = LogicalUnion::new(name_.clone());
        {
            let parent_scope = self.self_ref.clone().unwrap();
            logical_union.scope.write().unwrap().new_relationship_with_name(parent_scope.clone(), String::from("base"), ScopeRelationType::UnionScopeRela);
        }

        let scope_clone = logical_union.scope.clone();
        let union_data_type = Arc::new(RwLock::new(LogicalDataType::DataUnionType(name_.clone(), Arc::new(RwLock::new(logical_union)))));
        self.types.insert(name_.clone(), Arc::new(RwLock::new(TypeAlias::new(format!("!{{union_type}}_{}", name_.clone()), inferred!(Arc<RwLock<LogicalDataType>>, union_data_type) ))));
        return Ok(scope_clone);
    }

}