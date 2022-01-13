use std::fmt::Display;

/// region: Identifier
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Identifier {
    pub id : String,
}

impl Identifier {
    /// Construct a new identifier from something that can be turned into a String.
    pub fn from<T: Into<String>>(identifier: T) -> Self {
        Self {id: identifier.into()}
    }
}

impl From<Identifier> for String {
    fn from(i: Identifier) -> Self {
        i.id
    }
}

impl From<String> for Identifier {
    fn from(str: String) -> Self {
        Identifier{
            id: str,
        }
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.id.fmt(f)
    }
}

#[test]
fn test_convert_id_identifier(){
    let id = Identifier::from(String::from("abc"));
    assert_eq!(id.id, "abc");
    let identifier = Identifier{
        id : String::from("123"),
    };
    let identifier_str = String::from(identifier);
    assert_eq!(identifier_str, "123");
}

