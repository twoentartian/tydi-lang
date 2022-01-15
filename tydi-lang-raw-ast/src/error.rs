#[derive(Clone, Debug)]
pub enum ErrorCode {
    UnknownError(String),
    IdRedefined(String),
    IdNotFound(String),

}