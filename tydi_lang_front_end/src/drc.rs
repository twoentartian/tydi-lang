use tydi_lang_raw_ast::generate_get;

#[derive(Clone, Debug)]
pub enum DrcResultType {
    Error,
    Warning,
}

#[derive(Clone, Debug)]
pub enum DesignRuleDetail {
    InvalidConnectionPortTypeMismatch,
    InvalidConnectionWrongPortDirection,
}

#[derive(Clone, Debug)]
pub struct DesignRuleErrorWarning {
    message: String,
    err_warn_type: DrcResultType,
    detail: DesignRuleDetail,
}

impl DesignRuleErrorWarning {
    generate_get!(message, String, get_message);
    generate_get!(err_warn_type, DrcResultType, get_err_warn_type);
    generate_get!(detail, DesignRuleDetail, get_detail_type);

    pub fn new(err_warn: DrcResultType, detail_type: DesignRuleDetail, msg: String) -> Self {
        return Self {
            message: msg,
            err_warn_type: err_warn,
            detail: detail_type,
        }
    }
}
