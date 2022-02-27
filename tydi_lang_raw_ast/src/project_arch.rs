use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use deep_clone::DeepClone;
use error::ErrorCode;
use implement::ImplementType;
use scope::StreamletType;
use tydi_il::ToTydiIL;
use crate::{generate_get};
use crate::scope::{Scope, ScopeType};
use crate::util::*;

#[derive(Clone, Debug)]
pub struct Project {
    pub name: String,
    pub packages: HashMap<String, Arc<RwLock<Package>>>,
}

impl Project {
    pub fn new(name_: String) -> Self {
        Self {
            name: name_,
            packages: HashMap::new(),
        }
    }

    pub fn new_package(&mut self, name_: String) -> Result<Arc<RwLock<Scope>>,ErrorCode> {
        match self.packages.get(&name_) {
            None => {}
            Some(_) => {return Err(ErrorCode::ProjectArchError(format!("package name: {} already exists", name_)));}
        }

        let new_package = Package::new(name_.clone());
        let new_package_scope = new_package.scope.clone();
        let package_arc = Arc::new(RwLock::new(new_package));
        self.packages.insert(name_.clone(), package_arc);
        return Ok(new_package_scope);
    }

    pub fn with_package(&mut self, package: Package) -> Result<(),ErrorCode> {
        let name_ = package.get_name();
        match self.packages.get(&name_) {
            None => {}
            Some(_) => {return Err(ErrorCode::ProjectArchError(format!("package name: {} already exists", name_)));}
        }

        self.packages.insert(name_.clone(), Arc::new(RwLock::new(package)));
        return Ok(());
    }

    pub fn find_package(&self, name_: String) -> Result<Arc<RwLock<Package>>,ErrorCode> {
        match self.packages.get(&name_) {
            None => {return Err(ErrorCode::IdNotFound(format!("package name: {} already exists", name_))); }
            Some(package) => {return Ok(package.clone()); }
        }
    }

    pub fn to_tydi_il(&self, project_name: String) -> HashMap<String, String> {
        let mut output = HashMap::new();

        for (package_name, package) in self.packages.clone() {
            let file_name = format!("{}_{}", project_name.clone(), package_name.clone());
            let file_content = package.read().unwrap().to_tydi_il(project_name.clone());
            output.insert(file_name, file_content);
        }

        return output;
    }
}

impl PrettyPrint for Project {
    fn pretty_print(&self, depth: u32, verbose: bool) -> String {
        let mut output = String::new();

        //enter scope
        output.push_str(&format!("{}Project({}){{\n", generate_padding(depth), self.name.clone()));

        //enter packages
        for (_, package) in &self.packages {
            output.push_str(&format!("{}", package.read().unwrap().pretty_print(depth+1, verbose)));
        }

        //leave scope
        output.push_str(&format!("{}}}\n", generate_padding(depth)));
        return output;
    }
}

#[derive(Clone, Debug)]
pub struct Package {
    pub name: String,
    pub scope: Arc<RwLock<Scope>>,
}

impl Package {
    generate_get!(name, String, get_name);
    generate_get!(scope, Arc<RwLock<Scope>>, get_scope);

    pub fn set_name(&mut self, name_: String) {
        self.name = name_.clone();
        self.scope.write().unwrap().set_name(format!("package_{}", name_.clone()));
    }

    pub fn new(name_: String) -> Self {
        let scope_ = Arc::new(RwLock::new(Scope::new(format!("package_{}", name_.clone()), ScopeType::BasicScope)));
        scope_.write().unwrap().set_self_ref(scope_.clone());
        Self {
            name: name_.clone(),
            scope: scope_,
        }
    }

    pub fn to_tydi_il(&self, project_name: String) -> String {
        let mut type_alias_map: HashMap<String, (String, Vec<String>)> = HashMap::new();

        let mut output_streamlet = String::from("");
        {
            let streamlets = self.scope.read().unwrap().streamlets.clone();
            for (_, streamlet) in streamlets {
                match streamlet.read().unwrap().get_type() {
                    StreamletType::NormalStreamlet => {}
                    StreamletType::TemplateStreamlet(_) => { continue }
                    _ => unreachable!()
                }
                output_streamlet.push_str(&format!("{}\n", streamlet.read().unwrap().to_tydi_il(&mut type_alias_map, 1)));
            }
        }

        let mut output_implement = String::from("");
        {
            let mut output_implement_dependency: HashMap<String, Vec<String>> = HashMap::new();
            let mut output_implement_content: HashMap<String, String> = HashMap::new();
            let mut dependency_exist: HashMap<String, Arc<RwLock<bool>>> = HashMap::new();
            let implements = self.scope.read().unwrap().implements.clone();
            for (implement_name, implement) in implements {
                match implement.read().unwrap().get_type() {
                    ImplementType::NormalImplement => {}
                    ImplementType::TemplateImplement(_) => { continue }
                    _ => unreachable!()
                }
                dependency_exist.insert(implement_name.clone(),  Arc::new(RwLock::new(false)));
                output_implement_dependency.insert(implement_name.clone(), implement.read().unwrap().get_instance_impl_dependency());
                output_implement_content.insert(implement_name.clone(), implement.read().unwrap().to_tydi_il(&mut type_alias_map, 1));
            }

            let global_implement_instances = self.scope.read().unwrap().instances.clone();
            for (implement_name, instance) in global_implement_instances {
                let implement = instance.read().unwrap().get_implement_type().get_raw_value();
                let mut implement = implement.read().unwrap().deep_clone();
                implement.set_name(implement_name.clone());

                dependency_exist.insert(implement_name.clone(),  Arc::new(RwLock::new(false)));
                output_implement_dependency.insert(implement_name.clone(), implement.get_instance_impl_dependency());
                output_implement_content.insert(implement_name.clone(), implement.to_tydi_il(&mut type_alias_map, 1));
            }

            //analyzing dependency
            let mut output_implement_dependency_bool: HashMap<String, Vec<Arc<RwLock<bool>>>> = HashMap::new();
            for (impl_name, dependency) in &output_implement_dependency {
                let mut dependency_bool: Vec<Arc<RwLock<bool>>> = vec![];
                for impl_dependency in dependency {
                    match dependency_exist.get(impl_dependency) {
                        None => { unreachable!("we should never find an implement which isn't in scope, consider bugs in evaluation") }
                        Some(impl_exist) => { dependency_bool.push(impl_exist.clone()); }
                    }
                }
                let result = output_implement_dependency_bool.insert(impl_name.clone(), dependency_bool);
                if result.is_some() { unreachable!() }
            }

            let mut remain_impl = output_implement_dependency_bool.len();
            while remain_impl > 0 {
                for (impl_name, dependency) in &output_implement_dependency_bool {
                    let mut meet_dependency = true;
                    for single_dependency in dependency {
                        if !(*single_dependency.read().unwrap()) {
                            meet_dependency = false;
                            break;
                        }
                    }

                    let impl_in_exist = dependency_exist.get(impl_name).unwrap();
                    if meet_dependency && (*impl_in_exist.read().unwrap() == false) {
                        //put impl to the buffer
                        output_implement.push_str(&format!("{}\n", output_implement_content.get(impl_name).unwrap()));
                        let mut impl_in_exist = impl_in_exist.write().unwrap();
                        *impl_in_exist = true;
                        remain_impl = remain_impl - 1;
                    }
                }
            }
        }

        //types: and analyzing type dependency
        let mut types_content = String::from("");
        let mut type_exist: HashMap<String, Arc<RwLock<bool>>> = HashMap::new();
        for (type_name, (_, _)) in type_alias_map.clone() {
            type_exist.insert(type_name.clone(), Arc::new(RwLock::new(false)));
        }
        let mut type_dependency_map: HashMap<String, (String, Vec<Arc<RwLock<bool>>>)> = HashMap::new();
        for (type_name, (type_content, type_dependency)) in &type_alias_map {
            let mut type_dependencies = vec![];
            for single_type_dependency in type_dependency {
                if single_type_dependency.starts_with("Bits(") { continue; }
                if *single_type_dependency == String::from("Null") { continue; }
                let raw_type_dependency = type_exist.get(single_type_dependency).unwrap();
                type_dependencies.push(raw_type_dependency.clone());
            }
            type_dependency_map.insert(type_name.clone(), (type_content.clone(), type_dependencies));
        }

        let mut current_processed_type = 0;
        while current_processed_type < type_dependency_map.len() {
            for (type_name, (type_content, type_dependencies)) in &type_dependency_map {
                let mut type_dependencies_satisfied = true;
                for type_dependency in type_dependencies {
                    if *type_dependency.read().unwrap() == false {
                        type_dependencies_satisfied = false;
                    }
                }

                let mut target_type_exist = type_exist.get(type_name).unwrap().write().unwrap();
                if type_dependencies_satisfied && (*target_type_exist == false) {
                    types_content.push_str(&format!("{}type {} = {};\n", generate_padding(1) , crate::util::rename_id_to_il(type_name.clone()), type_content));
                    current_processed_type = current_processed_type + 1;
                    {
                        *target_type_exist = true;
                    }
                }
            }
        }

        let output = format!("namespace {}::{} {{\n\
        {}\n\
        {}\n\
        {}\n\
        }}", project_name.clone(), self.get_name(), types_content.clone(), output_streamlet.clone(), output_implement.clone());

        return output;
    }
}

impl PrettyPrint for Package {
    fn pretty_print(&self, depth: u32, verbose: bool) -> String {
        let mut output = String::new();

        //enter package
        output.push_str(&format!("{}Package({}){{\n", generate_padding(depth), self.name.clone()));
        //enter scope
        output.push_str(&format!("{}", self.scope.read().unwrap().pretty_print(depth+1, verbose)));
        //leave package
        output.push_str(&format!("{}}}\n", generate_padding(depth)));

        return output;
    }
}
