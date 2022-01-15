pub use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use crate::generate_get;
use crate::scope::Scope;
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

    pub fn new_package(&mut self, name_: String) -> Result<(),String> {
        match self.packages.get(&name_) {
            None => {}
            Some(_) => {return Err(format!("package name: {} already exists", name_));}
        }
        self.packages.insert(name_.clone(), Arc::new(RwLock::new(Package::new(name_.clone()))));
        return Ok(());
    }

    pub fn find_package(&mut self, name_: String) -> Result<Arc<RwLock<Package>>,String> {
        match self.packages.get(&name_) {
            None => {return Err(format!("package name: {} already exists", name_)); }
            Some(package) => {return Ok(package.clone()); }
        }
    }
}

impl PrettyPrint for Project {
    fn pretty_print(&self, depth: u32) -> String {
        let mut output = String::new();

        //enter scope
        output.push_str(&format!("{}Project({}){{\n", generate_padding(depth), self.name.clone()));

        //enter packages
        for (package_name, package) in &self.packages {
            output.push_str(&format!("{}", package.read().unwrap().pretty_print(depth+1) ));
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

    pub fn new(name_: String) -> Self {
        Self {
            name: name_.clone(),
            scope: Arc::new(RwLock::new(Scope::new(format!("package_{}", name_.clone())))),
        }
    }
}

impl PrettyPrint for Package {
    fn pretty_print(&self, depth: u32) -> String {
        let mut output = String::new();

        //enter package
        output.push_str(&format!("{}Package({}){{\n", generate_padding(depth), self.name.clone()));

        //enter scope
        output.push_str(&format!("{}", self.scope.read().unwrap().pretty_print(depth+1) ));

        //leave package
        output.push_str(&format!("{}}}\n", generate_padding(depth)));
        return output;
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::sync::Mutex;
    use crate::project_arch::Project;

    #[test]
    fn project_and_package() {
        let mut project0 = Project::new(String::from("project0"));
        let package_name = String::from("package0");
        let result = project0.new_package(package_name.clone());
        match result {
            Ok(_) => {}
            Err(_) => {assert!(false)}
        }
        let result = project0.find_package(package_name.clone());
        match result {
            Ok(package) => {
                let mut package = package.write().unwrap();
                println!("{}", package.get_name());
            }
            Err(_) => {assert!(false)}
        }


    }

    #[test]
    fn temp_test() {
        let mut value = Arc::new(Mutex::new(0));
        let mut value1 = value.clone();

        {
            let mut data = value1.lock().unwrap();
            *data = 1;
        }
        {
            let mut data = value.lock().unwrap();
            println!("{}", *data);
        }


    }
}
