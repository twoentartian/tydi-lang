use std::sync::{Arc};
use crate::identifier::*;

/// publicly use these to reduce the top-level "use" items.
pub use crate::id::Id;
pub use crate::ident::ident;
pub use crate::project_arch::{Project, Package, BTreeMap};
pub use crate::scope_var::{Scope, ScopeRelation, DataType, Variable};

#[salsa::query_group(ProjectSpace)]
pub trait ProjectElement {
    #[salsa::input]
    fn project(&self) -> Arc<Project>;

    #[salsa::interned]
    fn intern_identifier(&self, identifier: Identifier) -> Id<Identifier>;

    #[salsa::interned]
    fn intern_package(&self, package: Package) -> Id<Package>;

    #[salsa::interned]
    fn intern_scope(&self, scope: Scope) -> Id<Scope>;

    #[salsa::interned]
    fn intern_variable(&self, var: Variable) -> Id<Variable>;

    fn all_packages(&self) -> Arc<BTreeMap<String, Id<Package>>>;
}

fn all_packages(db: &dyn ProjectElement) -> Arc<BTreeMap<String,Id<Package>>> {
    Arc::new(db.project().packages.clone())
}
