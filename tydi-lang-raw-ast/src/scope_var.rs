pub use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub use crate::logical_data_type::LogicalDataType;
pub use crate::error::ErrorCode;
pub use crate::error::ErrorCode::*;
use crate::{generate_get};
use crate::util::*;

#[derive(Clone, Debug)]
pub enum ScopeRelationType {
    GroupScope,
    UnionScope,
    StreamScope,
    StreamletScope,
    ImplementScope,

    ParentScope, // a placeholder, should never be used
}

impl From<ScopeRelationType> for String {
    fn from(relationship: ScopeRelationType) -> Self {
        match relationship {
            ScopeRelationType::GroupScope => {String::from("GroupScope")}
            ScopeRelationType::UnionScope => {String::from("UnionScope")}
            ScopeRelationType::StreamScope => {String::from("StreamScope")}
            ScopeRelationType::StreamletScope => {String::from("StreamletScope")}
            ScopeRelationType::ImplementScope => {String::from("ImplementScope")}
            ScopeRelationType::ParentScope => {String::from("ParentScope")}
        }
    }
}

#[derive(Clone, Debug)]
pub struct ScopeRelationship {
    name: String,
    target_scope: Arc<RwLock<Scope>>,
    relationship: ScopeRelationType,
}

impl ScopeRelationship {
    generate_get!(name, String, get_name);

    pub fn new(target_scope_: Arc<RwLock<Scope>>, relationship_: ScopeRelationType) -> Self {
        Self {
            name: format!("{}_{}", target_scope_.read().unwrap().get_name(), String::from(relationship_.clone())),
            target_scope: target_scope_.clone(),
            relationship: relationship_.clone(),
        }
    }

}

impl PrettyPrint for ScopeRelationship {
    fn pretty_print(&self, depth: u32) -> String {
        return format!("--{}-->{}", String::from(self.relationship.clone()), self.target_scope.read().unwrap().get_name() );
    }
}

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

#[derive(Clone, Debug)]
pub struct Scope {
    pub name: String,

    pub scope_relationships: HashMap<String, ScopeRelationship>,
    pub types: HashMap<String, Arc<RwLock<DataType>>>,
    pub vars: HashMap<String, Arc<RwLock<Variable>>>,

}

impl Scope {
    generate_get!(name, String, get_name);

    pub fn new(name_: String) -> Self {
        Self {
            name: name_,
            scope_relationships: HashMap::new(),
            types: HashMap::new(),
            vars: HashMap::new(),
        }
    }

    pub fn new_relationship(&mut self, target_scope: Arc<RwLock<Scope>>, relationship: ScopeRelationType) {
        let new_scope_relationship = ScopeRelationship::new(target_scope.clone(), relationship.clone());
        self.scope_relationships.insert(new_scope_relationship.get_name(), new_scope_relationship);
    }

}

impl PrettyPrint for Scope {
    fn pretty_print(&self, depth: u32) -> String {
        let mut output = String::new();

        //enter scope
        output.push_str(&format!("{}Scope({}){{\n", generate_padding(depth), self.name.clone()));

        //enter vars
        output.push_str(&format!("{}Variables{{\n", generate_padding(depth+1)));
        for (var_name, var) in &self.vars {
            output.push_str(&format!("{}\n", var.read().unwrap().pretty_print(depth+2)));
        }
        output.push_str(&format!("{}}}\n", generate_padding(depth+1)));

        //enter types
        output.push_str(&format!("{}Types{{\n", generate_padding(depth+1)));
        for (type_name, t) in &self.types {
            output.push_str(&format!("{}({})\n",  type_name, t.read().unwrap().pretty_print(depth+2)));
        }
        output.push_str(&format!("{}}}\n", generate_padding(depth+1)));

        //enter scope_relationships
        output.push_str(&format!("{}ScopeRelations{{\n", generate_padding(depth+1)));
        for (relation_name, scope_relation) in &self.scope_relationships {
            output.push_str(&format!("{}\n", scope_relation.pretty_print(depth+2)) );
        }
        output.push_str(&format!("{}}}\n", generate_padding(depth+1)));

        //leave scope
        output.push_str(&format!("{}}}\n", generate_padding(depth)));
        return output;
    }
}

#[derive(Clone, Debug)]
pub struct Variable {
    pub name: String,
    pub var_type: Arc<RwLock<DataType>>,
}

impl Variable {
    pub fn new(name_: String, type_: DataType) -> Self {
        Self {
            name: name_,
            var_type: Arc::new(RwLock::new(type_)),
        }
    }
}

impl PrettyPrint for Variable {
    fn pretty_print(&self, depth: u32) -> String {
        return format!("{}{}:{}", generate_padding(depth), self.name.clone(), self.var_type.read().unwrap().pretty_print(depth) );
    }
}

impl Scope {
    pub fn new_variable(&mut self, name_: String, type_: DataType) -> Result<(), ErrorCode> {
        match self.vars.get(&name_) {
            None => {}
            Some(_) => { return Err(IdRedefined(format!("variable {} already defined", name_))); }
        };
        self.vars.insert(name_.clone(), Arc::new(RwLock::new(Variable::new(name_.clone(), type_.clone()))));
        return Ok(());
    }

    pub fn resolve_variable(& self, name_: String) -> Result<Arc<RwLock<Variable>>, ErrorCode> {
        return match self.vars.get(&name_) {
            None => { Err(IdNotFound(format!("variable {} not found", name_))) }
            Some(var) => { Ok(var.clone()) }
        };
    }

    pub fn resolve_variable_in_scopes(& self, name_: String, allowed_relationships: Vec<ScopeRelationship>) -> Result<Arc<RwLock<Variable>>, ErrorCode> {
        match self.resolve_variable(name_.clone()) {
            Ok(var) => { return Ok(var) }
            Err(_) => {}
        }

        todo!();

        return Err(IdNotFound(format!("variable {} not found", name_.clone())));
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, RwLock};
    use std::sync::Mutex;
    use crate::error::ErrorCode;
    use crate::project_arch::Project;
    use crate::scope_var::DataType::*;
    use crate::error::ErrorCode::*;
    use crate::scope_var::ScopeRelationType::ParentScope;
    use crate::scope_var::Variable;
    use crate::util::PrettyPrint;

    #[test]
    fn var_scope() {
        let mut project0 = Project::new(String::from("project0"));
        let package_name = String::from("package0");
        let result = project0.new_package(package_name.clone());
        let result = project0.find_package(package_name.clone()).unwrap();
        let mut package = result.write().unwrap();
        let mut package_scope = package.scope.write().unwrap();
        match package_scope.new_variable(String::from("var1"), IntType) {
            Ok(()) => {}
            Err(err_code) => {
                println!("error: {:?}", err_code);
                assert!(false);
            }
        }
        match package_scope.new_variable(String::from("var1"), IntType) {
            Ok(()) => {}
            Err(err_code) => {
                match err_code {
                    UnknownError(_) => {assert!(false)}
                    IdRedefined(_) => {assert!(true)}
                    IdNotFound(_) => {assert!(false)}
                }
            }
        }
        package_scope.new_variable(String::from("var2"), StringType);

        println!("{}", package_scope.pretty_print(0));

    }

    #[test]
    fn print_project() {
        let mut project0 = Project::new(String::from("project0"));
        {
            let package_name = String::from("package0");
            let result = project0.new_package(package_name.clone());
            let result = project0.find_package(package_name.clone()).unwrap();
            let mut package = result.write().unwrap();
            let mut package_scope = package.scope.write().unwrap();
            match package_scope.new_variable(String::from("var1"), IntType) {
                Ok(()) => {}
                Err(err_code) => {
                    println!("error: {:?}", err_code);
                    assert!(false);
                }
            }
            match package_scope.new_variable(String::from("var1"), IntType) {
                Ok(()) => {}
                Err(err_code) => {
                    match err_code {
                        UnknownError(_) => {assert!(false)}
                        IdRedefined(_) => {assert!(true)}
                        IdNotFound(_) => {assert!(false)}
                    }
                }
            }
            package_scope.new_variable(String::from("var2"), StringType);
        }
        println!("{}", project0.pretty_print(0));

    }
}