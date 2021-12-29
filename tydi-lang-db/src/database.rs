use std::char::DecodeUtf16;
use crate::id::*;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Identifier(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Project {
    pub identifier: Id<Identifier>,
    pub package: HashMap<String, String>,
}

impl Project {
    pub fn new(identifier: Id<Identifier>) -> Self {
        Self {
            identifier,
            package: HashMap::new(),
        }
    }

    pub fn with_package(mut self, name : String, content : String) -> Self {
        self.package.insert(name, content);
        self
    }
}

#[salsa::query_group(ProjectSpace)]
pub trait ProjectElement {
    #[salsa::input]
    fn code(&self) -> Arc<Project>;

}

#[cfg(test)]
mod db_tests {
    use crate::database::Project;

    #[test]
    fn test_project(){
        //let test_project : Project = Project::new();

    }
}