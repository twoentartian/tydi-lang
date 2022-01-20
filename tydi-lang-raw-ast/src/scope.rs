pub use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};

pub use crate::logical_data_type::*;
pub use crate::variable::*;
pub use crate::data_type::*;
pub use crate::group_union_type::*;
pub use crate::steam_type::*;
pub use crate::streamlet::*;
pub use crate::port::*;
pub use crate::implement::Implement;
pub use crate::type_alias::TypeAlias;
pub use crate::instances::Instance;

pub use crate::error::ErrorCode;
pub use crate::util::*;

use crate::{generate_get};



#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum ScopeRelationType {
    GroupScopeRela,
    UnionScopeRela,
    StreamScopeRela,
    StreamletScopeRela,
    ImplementScopeRela,

    ParentScopeRela, // a placeholder, should never be used
}

impl From<ScopeRelationType> for String {
    fn from(relationship: ScopeRelationType) -> Self {
        match relationship {
            ScopeRelationType::GroupScopeRela => {String::from("GroupScope")}
            ScopeRelationType::UnionScopeRela => {String::from("UnionScope")}
            ScopeRelationType::StreamScopeRela => {String::from("StreamScope")}
            ScopeRelationType::StreamletScopeRela => {String::from("StreamletScope")}
            ScopeRelationType::ImplementScopeRela => {String::from("ImplementScope")}
            ScopeRelationType::ParentScopeRela => {String::from("ParentScope")}
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
    generate_get!(target_scope, Arc<RwLock<Scope>>, get_target_scope);

    pub fn new(target_scope_: Arc<RwLock<Scope>>, relationship_: ScopeRelationType) -> Self {
        Self {
            name: format!("{}_{}", target_scope_.read().unwrap().get_name(), String::from(relationship_.clone())),
            target_scope: target_scope_.clone(),
            relationship: relationship_.clone(),
        }
    }

    pub fn new_with_name(target_scope_: Arc<RwLock<Scope>>, target_scope_name: String, relationship_: ScopeRelationType) -> Self {
        Self {
            name: format!("{}_{}", target_scope_name.clone(), String::from(relationship_.clone())),
            target_scope: target_scope_.clone(),
            relationship: relationship_.clone(),
        }
    }

}

impl PrettyPrint for ScopeRelationship {
    fn pretty_print(&self, depth: u32, verbose: bool) -> String {
        return format!("{}--{}-->{}", generate_padding(depth), String::from(self.relationship.clone()), self.target_scope.read().unwrap().get_name() );
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ScopeType {
    BasicScope,

    GroupScope,
    UnionScope,
    StreamScope,
    StreamletScope,
    ImplementScope,

    ParentScope, // a placeholder, should never be used
}

impl From<ScopeType> for String {
    fn from(scope_type: ScopeType) -> Self {
        match scope_type {
            ScopeType::BasicScope => {String::from("BasicScope")}
            ScopeType::GroupScope => {String::from("GroupScope")}
            ScopeType::UnionScope => {String::from("UnionScope")}
            ScopeType::StreamScope => {String::from("StreamScope")}
            ScopeType::StreamletScope => {String::from("StreamletScope")}
            ScopeType::ImplementScope => {String::from("ImplementScope")}
            ScopeType::ParentScope => {String::from("ParentScope")}
        }
    }
}

#[derive(Clone, Debug)]
pub struct Scope {
    pub name: String,
    pub scope_type: ScopeType,
    pub self_ref: Option<Arc<RwLock<Scope>>>,

    pub scope_relationships: HashMap<String, ScopeRelationship>,
    pub types: HashMap<String, Arc<RwLock<TypeAlias>>>,
    pub vars: HashMap<String, Arc<RwLock<Variable>>>,

    pub streamlets: HashMap<String, Arc<RwLock<Streamlet>>>,
    pub ports: HashMap<String, Arc<RwLock<Port>>>,
    pub implements: HashMap<String, Arc<RwLock<Implement>>>,
    pub instances: HashMap<String, Arc<RwLock<Instance>>>,

    //pub implements: HashMap<String, Arc<RwLock<Streamlet>>>,
}

impl Scope {
    generate_get!(name, String, get_name);

    pub fn new(name_: String, scope_type_: ScopeType) -> Self {
        Self {
            name: name_,
            self_ref: None,
            scope_type: scope_type_,
            scope_relationships: HashMap::new(),
            types: HashMap::new(),
            vars: HashMap::new(),
            streamlets: HashMap::new(),
            ports: HashMap::new(),
            implements: HashMap::new(),
            instances: HashMap::new(),
        }
    }

    pub fn set_self_ref(&mut self, self_ref: Arc<RwLock<Scope>>) {
        self.self_ref = Some(self_ref);
    }

    pub fn new_relationship(&mut self, target_scope: Arc<RwLock<Scope>>, relationship: ScopeRelationType) {
        let new_scope_relationship = ScopeRelationship::new(target_scope, relationship);
        self.scope_relationships.insert(new_scope_relationship.get_name(), new_scope_relationship);
    }

    pub fn new_relationship_with_name(&mut self, target_scope: Arc<RwLock<Scope>>, name: String, relationship: ScopeRelationType) {
        let new_scope_relationship = ScopeRelationship::new_with_name(target_scope, name, relationship);
        self.scope_relationships.insert(new_scope_relationship.get_name(), new_scope_relationship);
    }

}

impl PrettyPrint for Scope {
    fn pretty_print(&self, depth: u32, verbose: bool) -> String {
        let mut output = String::new();

        //enter scope
        output.push_str(&format!("{}Scope({}){{\n", generate_padding(depth), self.name.clone()));

        if !self.vars.is_empty() || verbose {
            //enter vars
            output.push_str(&format!("{}Variables{{\n", generate_padding(depth+1)));
            for (_, var) in &self.vars {
                output.push_str(&format!("{}\n", var.read().unwrap().pretty_print(depth+2, verbose)));
            }
            output.push_str(&format!("{}}}\n", generate_padding(depth+1)));
        }
        if !self.types.is_empty() || verbose{
            //enter types
            output.push_str(&format!("{}Types{{\n", generate_padding(depth+1)));
            for (type_name, t) in &self.types {
                output.push_str(&format!("{}{}:{}\n", generate_padding(depth+2), type_name, t.read().unwrap().pretty_print(depth+2, verbose)));
            }
            output.push_str(&format!("{}}}\n", generate_padding(depth+1)));
        }
        if !self.scope_relationships.is_empty() || verbose {
            //enter scope_relationships
            output.push_str(&format!("{}ScopeRelations{{\n", generate_padding(depth+1)));
            for (_, scope_relation) in &self.scope_relationships {
                output.push_str(&format!("{}\n", scope_relation.pretty_print(depth+2, verbose)) );
            }
            output.push_str(&format!("{}}}\n", generate_padding(depth+1)));
        }
        if !self.streamlets.is_empty() || verbose {
            //enter scope_relationships
            output.push_str(&format!("{}Streamlets{{\n", generate_padding(depth+1)));
            for (_, streamlet) in &self.streamlets {
                output.push_str(&format!("{}\n", streamlet.read().unwrap().pretty_print(depth+2, verbose)) );
            }
            output.push_str(&format!("{}}}\n", generate_padding(depth+1)));
        }
        if !self.ports.is_empty() || verbose {
            //enter scope_relationships
            output.push_str(&format!("{}Ports{{\n", generate_padding(depth+1)));
            for (_, port) in &self.ports {
                output.push_str(&format!("{}\n", port.read().unwrap().pretty_print(depth+2, verbose)) );
            }
            output.push_str(&format!("{}}}\n", generate_padding(depth+1)));
        }
        if !self.implements.is_empty() || verbose {
            //enter scope_relationships
            output.push_str(&format!("{}Implements{{\n", generate_padding(depth+1)));
            for (_, implement) in &self.implements {
                output.push_str(&format!("{}\n", implement.read().unwrap().pretty_print(depth+2, verbose)) );
            }
            output.push_str(&format!("{}}}\n", generate_padding(depth+1)));
        }
        if !self.instances.is_empty() || verbose {
            //enter scope_relationships
            output.push_str(&format!("{}Instances{{\n", generate_padding(depth+1)));
            for (_, inst) in &self.instances {
                output.push_str(&format!("{}\n", inst.read().unwrap().pretty_print(depth+2, verbose)) );
            }
            output.push_str(&format!("{}}}\n", generate_padding(depth+1)));
        }

        //leave scope
        output.push_str(&format!("{}}}\n", generate_padding(depth)));
        return output;
    }
}



#[cfg(test)]
mod tests {
    use std::sync::RwLockReadGuard;
    use crate::implement::ImplementType;
    use crate::inferable::{Inferable, NewInferable};
    use crate::{inferred, not_inferred};
    use crate::project_arch::Project;
    use crate::scope::*;
    use crate::scope::DataType::StringType;


    #[test]
    fn var_scope() {
        let mut project0 = Project::new(String::from("project0"));
        let package_name = String::from("package0");
        let result = project0.new_package(package_name.clone());
        let result = project0.find_package(package_name.clone()).unwrap();
        let mut package = result.write().unwrap();
        let mut package_scope = package.scope.write().unwrap();
        match package_scope.new_variable(String::from("var1"), DataType::IntType, String::from("")) {
            Ok(()) => {}
            Err(err_code) => {
                println!("error: {:?}", err_code);
                assert!(false);
            }
        }
        match package_scope.new_variable(String::from("var1"), DataType::IntType, String::from("")) {
            Ok(()) => {}
            Err(err_code) => {
                match err_code {
                    ErrorCode::UnknownError(_) => {assert!(false)}
                    ErrorCode::IdRedefined(_) => {assert!(true)}
                    ErrorCode::IdNotFound(_) => {assert!(false)}
                    ErrorCode::ScopeNotAllowed(_) => {assert!(false)}
                }
            }
        }
        package_scope.new_variable(String::from("var2"), DataType::StringType, String::from(""));

        println!("{}", package_scope.pretty_print(0, false));

    }

    #[test]
    fn print_project() {
        let mut project0 = Project::new(String::from("project0"));

        //set default stream parameter
        {
            let mut default_stream = DefaultLogicalStream.write().unwrap();
            default_stream.set_dimension(Variable::new_int(String::from(""), 2));
            default_stream.set_complexity(Variable::new_int(String::from(""), 6));
        }

        //generate project
        {
            let package_name = String::from("package0");
            let package_scope_l = project0.new_package(package_name.clone()).unwrap();
            let mut package_scope = package_scope_l.write().unwrap();
            package_scope.new_variable(String::from("var1"), DataType::IntType, String::from(""));
            package_scope.new_variable(String::from("var2"), DataType::StringType, String::from(""));
            package_scope.new_variable(String::from("f0"), DataType::FloatType, String::from(""));
            let new_group = package_scope.new_logical_group(String::from("group0")).unwrap();
            {
                let mut group_scope = new_group.write().unwrap();
                group_scope.new_variable(String::from("var3"), DataType::StringType, String::from(""));
                group_scope.new_logical_bit(String::from("bit16"), String::from("16"));
                group_scope.new_logical_bit_with_definite(String::from("bit16_"), 16);
            }
            package_scope.new_logical_union(String::from("union0"));
            package_scope.new_logical_null(String::from("null"));
            package_scope.new_logical_bit(String::from("bit8"), String::from("8"));
            let temp_type = package_scope.resolve_type_in_current_scope(String::from("group0")).unwrap();

            let streamlet_new = package_scope.new_streamlet(String::from("streamlet0"), StreamletType::NormalStreamlet).unwrap();
            {
                let mut streamlet_scope = streamlet_new.write().unwrap();
                streamlet_scope.new_variable(String::from("var4"), DataType::StringType, String::from(""));
                streamlet_scope.new_logical_bit(String::from("bit2"), String::from("2"));
            }

            {
                let type_alias = temp_type.read().unwrap();
                let t = type_alias.get_type_infer().get_raw_value();

                package_scope.new_logical_stream(String::from("stream0"), t.clone());

                match package_scope.resolve_streamlet_from_scope(String::from("streamlet0")) {
                    Ok(streamlet) => {
                        //streamlet.read().unwrap().new_port(String::from("port0"), <Inferable<Arc<RwLock<LogicalDataType>>> as NewInferable<Arc<RwLock<LogicalDataType>>>>::_new_inferred(String::from(""), t.clone()) , PortDirection::Input);
                        streamlet.read().unwrap().new_port(String::from("port0"), inferred!(Arc<RwLock<LogicalDataType>>, t.clone()) , PortDirection::Input);
                        streamlet.read().unwrap().new_port(String::from("port1"), not_inferred!(Arc<RwLock<LogicalDataType>>, String::from("port1_type")) , PortDirection::Input);
                    }
                    Err(_) => { assert!(false) }
                }
            }

            let implement0_box = package_scope.new_implement(String::from("impl0"), ImplementType::NormalImplement).unwrap();
            implement0_box.read().unwrap().new_instance(String::from("instance"),
                <Inferable<Arc<RwLock<Streamlet>>> as NewInferable<Arc<RwLock<Streamlet>>>>::_new(String::from("streamlet_unknown")));

        }
        println!("{}", project0.pretty_print(0, false));

        //access
        {
            let package_container = project0.find_package(String::from("package0")).unwrap();
            let package = package_container.read().unwrap();
            let group_type = package.scope.read().unwrap().resolve_type_in_current_scope(String::from("group0")).unwrap();
            let group_type_alias = group_type.read().unwrap();

            match &*(group_type_alias.get_type_infer().get_raw_value().read().unwrap()) {
                LogicalDataType::DataGroupType(group_name, group_scope) => {
                    let result = group_scope.read().unwrap().get_scope().read().unwrap().resolve_variable_from_scope(String::from("var1"));
                    let var = result.unwrap();

                    let result = group_scope.read().unwrap().get_scope().read().unwrap().resolve_variable_in_current_scope(String::from("var1"));
                    match result {
                        Ok(_) => { assert!(false) }
                        Err(_) => { assert!(true) }
                    }

                    let result = group_scope.read().unwrap().get_scope().read().unwrap().resolve_type_from_scope(String::from("bit8"));
                    let type_ = result.unwrap();

                    let result = group_scope.read().unwrap().get_scope().read().unwrap().resolve_type_in_current_scope(String::from("bit8"));
                    match result {
                        Ok(_) => { assert!(false) }
                        Err(_) => { assert!(true) }
                    }
                }
                _ => {}
            }
            let output_str = group_type.read().unwrap().pretty_print(0, false);
            println!("{}", output_str);
        }
    }
}