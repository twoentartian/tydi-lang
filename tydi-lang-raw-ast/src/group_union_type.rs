use std::sync::{Arc, RwLock};
use crate::data_type::DataType;
use crate::error::ErrorCode;
use crate::{generate_get, inferred, not_inferred};
use crate::scope::{Scope, ScopeRelationType, ScopeType, TypeAlias};
use crate::util::{generate_padding, PrettyPrint};
use crate::inferable::{Inferable, NewInferable};
use crate::logical_data_type::LogicalDataType;

#[derive(Clone, Debug)]
pub struct LogicalGroup {
    name: String,

    scope: Arc<RwLock<Scope>>,
}

impl LogicalGroup {
    generate_get!(name, String, get_name);
    generate_get!(scope, Arc<RwLock<Scope>>, get_scope);

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

impl LogicalUnion {
    generate_get!(name, String, get_name);
    generate_get!(scope, Arc<RwLock<Scope>>, get_scope);

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
        {
            self.scope.write().unwrap().new_variable(name_.clone(), type_.clone(), exp_.clone());
        }
        return Ok(());
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
        if self.scope_type != ScopeType::BasicScope { panic!("not allowed to define group type outside of base scope") }

        match self.types.get(&name_) {
            None => {}
            Some(_) => { return Err(ErrorCode::IdRedefined(format!("type {} already defined", name_.clone()))); }
        };

        let mut logical_group = LogicalGroup::new(name_.clone());
        {
            let parent_scope = self.self_ref.clone().unwrap();
            logical_group.scope.write().unwrap().new_relationship_with_name(parent_scope.clone(), String::from("base"), ScopeRelationType::GroupScopeRela);
        }

        let scope_clone = logical_group.scope.clone();
        let mut group_data_type = Arc::new(RwLock::new(LogicalDataType::DataGroupType(name_.clone(), Arc::new(RwLock::new(logical_group)))));
        self.types.insert(name_.clone(), Arc::new(RwLock::new( TypeAlias::new(format!("!{{union_type}}_{}", name_.clone()), inferred!(Arc<RwLock<LogicalDataType>>, group_data_type)))));
        return Ok(scope_clone);
    }

    pub fn new_logical_union(&mut self, name_: String) -> Result<Arc<RwLock<Scope>>, ErrorCode> {
        if self.scope_type != ScopeType::BasicScope { panic!("not allowed to define group type outside of base scope") }

        match self.types.get(&name_) {
            None => {}
            Some(_) => { return Err(ErrorCode::IdRedefined(format!("type {} already defined", name_.clone()))); }
        };

        let mut logical_union = LogicalUnion::new(name_.clone());
        {
            let parent_scope = self.self_ref.clone().unwrap();
            logical_union.scope.write().unwrap().new_relationship_with_name(parent_scope.clone(), String::from("base"), ScopeRelationType::UnionScopeRela);
        }

        let scope_clone = logical_union.scope.clone();
        let mut union_data_type = Arc::new(RwLock::new(LogicalDataType::DataUnionType(name_.clone(), Arc::new(RwLock::new(logical_union)))));
        self.types.insert(name_.clone(), Arc::new(RwLock::new(TypeAlias::new(format!("!{{union_type}}_{}", name_.clone()), inferred!(Arc<RwLock<LogicalDataType>>, union_data_type) ))));
        return Ok(scope_clone);
    }

}