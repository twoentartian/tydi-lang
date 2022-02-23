use std::sync::{Arc, RwLock};
use ParserErrorCode;
use tydi_lang_raw_ast::scope::{Scope, ScopeType, Variable, VariableValue};

pub fn generate_template_instance_name(template_name: String, template_exps: &Vec<Arc<RwLock<Variable>>>) -> String
{
    let mut template_exp_string = String::from("");
    for template_exp in template_exps {
        let var_value = template_exp.read().unwrap().get_var_value().get_raw_value();

        match var_value {
            VariableValue::Int(v) => {
                template_exp_string.push_str(&format!("@{}", v.to_string()));
            }
            VariableValue::Bool(v) => {
                template_exp_string.push_str(&format!("@{}", v.to_string()));
            }
            VariableValue::Float(v) => {
                template_exp_string.push_str(&format!("@{}", v.to_string()));
            }
            VariableValue::Str(v) => {
                template_exp_string.push_str(&format!("@{}", v.to_string()));
            }
            VariableValue::LogicalDataType(type_) => {
                template_exp_string.push_str(&format!("@{}", String::from((*type_.read().unwrap()).clone())))
            }
            VariableValue::Unknown => {
                template_exp_string.push_str(&format!("@{}", String::from(template_exp.read().unwrap().get_var_value().get_raw_exp())));
            }

            _ => unreachable!()
        }
    }
    return format!("{}{}", template_name, template_exp_string);
}

pub fn check_import_package(package_name: String, scope: Arc<RwLock<Scope>>) -> Result<(), ParserErrorCode> {
    let package_var_result = scope.read().unwrap().resolve_variable_from_scope(format!("{}{}", *crate::built_in_ids::PACKAGE_PREFIX, package_name.clone()));
    if package_var_result.is_err() { return Err(ParserErrorCode::PackageNotImported(format!("package {} not imported", package_name.clone()))); }
    return Ok(());
}

pub fn goto_basic_scope(scope: Arc<RwLock<Scope>>) -> Result<Arc<RwLock<Scope>>, ParserErrorCode> {
    if scope.clone().read().unwrap().scope_type.clone() == ScopeType::BasicScope {
        return Ok(scope);
    }
    else {
        let relationships = scope.read().unwrap().scope_relationships.clone();
        for (_, relationship) in relationships {
            let parent = relationship.get_target_scope();
            let result = goto_basic_scope(parent);
            if result.is_ok() { return Ok(result.ok().unwrap()); }
        }
    }
    return Err(ParserErrorCode::ParserError(format!("cannot find the basic scope")));
}
