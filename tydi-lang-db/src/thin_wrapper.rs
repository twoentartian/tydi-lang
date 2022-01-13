use std::sync::{Arc, Mutex};

use crate::database::{Scope, DataType, Project, Package, ProjectElement, Variable};
use crate::database::{BTreeMap, Id, ident};
use crate::Database;
use crate::database::DataType::Int;

/// region: ProjectWrapper
pub struct ProjectWrapper {
    pub db: Database,
    pub name: String,
    pub project: Project,
}

impl ProjectWrapper {
    pub fn new(name_: String) -> Self {
        let mut output_db = Database::default();
        let project_id = ident(&output_db, name_.clone());
        let output_project = Project::new(project_id);
        output_db.set_project(Arc::new(output_project.clone()));
        Self {
            db: output_db,
            name: name_.clone(),
            project: output_project,
        }
    }

    pub fn new_package(&mut self, name_: String) -> Result<(), String> {
        match self.find_package(name_.clone()) {
            Ok(package_id) => {return Err(format!("package {} redefined", name_.clone()))},
            Err(_) => {},
        }

        let package_id = ident(&self.db, name_.clone());
        let package_intern_id = self.db.intern_package(Package::new(package_id.clone()));
        self.project.with_package(name_, package_intern_id);
        self.db.set_project(Arc::new(self.project.clone()));
        Ok(())
    }

    pub fn find_package(&mut self, name_: String) -> Result<Package, String> {
        match self.project.packages.get(&name_) {
            Some(package_id) => {
                let package = self.db.lookup_intern_package(*package_id);
                Ok(package)
            },
            None => Err(format!("package {} not found", name_.clone()))
        }
    }

    pub fn new_variable(&mut self, scope: &mut Scope, var_name: String, var_type: DataType) -> Result<(), String> {
        let scope = self.db.lookup_intern_scope(scope.)

        let find_result = scope.find_variable(var_name.clone());
        match find_result {
            Ok(var_id) => {return Err(format!("variable {} has already been defined", var_name.clone()))}
            Err(info) => {}
        }

        //begin creat variable
        let var_name_id = ident(&self.db, var_name.clone());
        let mut var = Variable::new(var_name_id.clone());
        var.var_type = var_type.clone();
        let var_intern_id = self.db.intern_variable(var);
        scope.with_variable(var_name.clone(), var_intern_id.clone());

        Ok(())
    }
}

#[test]
fn test_db_structure() {
    let mut project = ProjectWrapper::new(String::from("test_project"));

    match project.new_package(String::from("package0")) {
        Ok(()) => {},
        Err(info) => panic!("{}", info),
    }
    match project.new_package(String::from("package1")) {
        Ok(()) => {},
        Err(info) => panic!("{}", info),
    }
    match project.new_package(String::from("package1")) {
        Ok(()) => panic!("redefine package test failed"),
        Err(_) => {}
    }

    match project.find_package(String::from("package0")) {
        Ok(mut package) => {
            println!("package found");
            match project.new_variable(&mut package.package_scope, String::from("var1"), Int) {
                Ok(()) => {},
                Err(_) => {panic!("failed to define var")},
            }
            match project.new_variable(&mut package.package_scope, String::from("var1"), Int) {
                Ok(()) => {panic!("var redefined test failed")},
                Err(_) => {},
            }
            project.update_package(package);
        },
        Err(str) => println!("package not found: {}", str)
    }

    match project.find_package(String::from("package0")) {
        Ok(mut package) => {
            println!("package found");
            match project.new_variable(&mut package.package_scope, String::from("var1"), Int) {
                Ok(()) => {panic!("var redefined test failed")},
                Err(_) => {},
            }
        },
        Err(str) => println!("package not found: {}", str)
    }

    let packages : Arc<BTreeMap<String, Id<Package>>> = project.db.all_packages();
    for (package_name, package_id) in &*packages {
        println!("{}", package_name);
    }
}