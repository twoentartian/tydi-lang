use crate::database::ProjectElement;
use crate::identifier::Identifier;
use crate::id::Id;

/// Intern an identifier and return its id.
// pub fn ident<T: Into<String>>(db: &dyn ProjectElement, identifier: T) -> Id<Identifier> {
//     db.intern_identifier(Identifier::from(identifier))
// }

pub fn ident(db: &dyn ProjectElement, identifier: String) -> Id<Identifier> {
    db.intern_identifier(Identifier{id:identifier})
}