use lazy_static::lazy_static;

lazy_static! {
    pub static ref PACKAGE_PREFIX: String = format!("$package$");
    pub static ref ARG_PREFIX: String = format!("$arg$");
    pub static ref GENERATED_ID_PREFIX: String = format!("$generated$");
}
