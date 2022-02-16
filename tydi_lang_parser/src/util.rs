use std::sync::{Arc, RwLock};
use ParserErrorCode;
use tydi_lang_raw_ast::scope::{Scope, Variable};

pub fn generate_template_instance_name(template_name: String, template_exps: &Vec<Arc<RwLock<Variable>>>) -> String
{
    let mut template_exp_string = String::from("");
    for template_exp in template_exps {
        template_exp_string.push_str(&format!("@{}", String::from(template_exp.read().unwrap().get_var_value().get_raw_exp())));
    }
    return format!("{}{}", template_name, template_exp_string);
}

pub fn check_import_package(package_name: String, scope: Arc<RwLock<Scope>>) -> Result<(), ParserErrorCode> {
    let package_var_result = scope.read().unwrap().resolve_variable_from_scope(format!("{}{}", *crate::built_in_ids::PACKAGE_PREFIX, package_name.clone()));
    if package_var_result.is_err() { return Err(ParserErrorCode::PackageNotImported(format!("package {} not imported", package_name.clone()))); }
    return Ok(());
}
