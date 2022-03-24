#[derive(Debug, Clone)]
pub enum Frequency {
    Unknown,

    Hz(f64),
    #[allow(non_camel_case_types)]
    kHz(f64),
    MHz(f64),
    GHz(f64),
}