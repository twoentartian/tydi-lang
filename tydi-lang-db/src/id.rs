use std::{
    fmt::{self, Debug, Display, Formatter},
    hash::{Hash, Hasher},
    marker::PhantomData,
};

pub struct Id<T> {
    // The Salsa intern id field.
    id: salsa::InternId,
    // `fn(T) -> T` makes Id invariant over `T` i.e. Id gets an auto trait
    // implementation for `Send` + `Sync` without a `Send` + `Sync` bound on
    // `T`.
    // Reference: https://doc.rust-lang.org/nomicon/phantom-data.html#table-of-phantomdata-patterns
    _ty: PhantomData<fn(T) -> T>,
}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Id<T> {
        Id {
            id: self.id,
            _ty: PhantomData,
        }
    }
}

impl<T> Copy for Id<T> {}

impl<T> Debug for Id<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self.id, f)
    }
}

impl<T> Display for Id<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.id, f)
    }
}

impl<T> Hash for Id<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T> Eq for Id<T> {}

impl<T> salsa::InternKey for Id<T> {
    fn from_intern_id(id: salsa::InternId) -> Self {
        Self {
            id,
            _ty: PhantomData,
        }
    }
    fn as_intern_id(&self) -> salsa::InternId {
        self.id
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of;

    #[test]
    fn id_size() {
        struct Foo {
            _x: String,
        }
        assert_eq!(size_of::<Id<Foo>>(), size_of::<salsa::InternId>());
    }

}
