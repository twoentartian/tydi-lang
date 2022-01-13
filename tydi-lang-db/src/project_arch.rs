pub use std::collections::BTreeMap;
use crate::id::Id;
use crate::identifier::Identifier;
use crate::scope_var::Scope;

/// region: Project
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Project {
    pub identifier: Id<Identifier>,
    pub packages: BTreeMap<String, Id<Package>>
}

impl Project {
    pub fn new(identifier: Id<Identifier>) -> Self {
        Self {
            identifier: identifier,
            packages: BTreeMap::new(),
        }
    }

    pub fn with_package(&mut self, package_name: String, package_id: Id<Package> ) {
        self.packages.insert(package_name, package_id);
    }
}

/// region: Package
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Package {
    pub identifier: Id<Identifier>,
    pub package_scope: Scope,
}

impl Package {
    pub fn new(name: Id<Identifier>) -> Self {
        Self {
            identifier: id,
            package_scope: Scope::new(),
        }
    }


}
