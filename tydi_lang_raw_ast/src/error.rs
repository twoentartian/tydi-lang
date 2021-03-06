#[derive(Clone, Debug)]
pub enum ErrorCode {
    UnknownError(String),
    ProjectArchError(String),
    IdRedefined(String),
    IdNotFound(String),
    ScopeNotAllowed(String),
}

impl From<ErrorCode> for String {
    fn from(e: ErrorCode) -> Self {
        return match e {
            ErrorCode::UnknownError(s) => { format!("UnknownError:{}", s) }
            ErrorCode::ProjectArchError(s) => { format!("ProjectArchError:{}", s) }
            ErrorCode::IdRedefined(s) => { format!("IdRedefined:{}", s) }
            ErrorCode::IdNotFound(s) => { format!("IdNotFound:{}", s) }
            ErrorCode::ScopeNotAllowed(s) => { format!("ScopeNotAllowed:{}", s) }
        }
    }
}