use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use error::ErrorCode;
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
