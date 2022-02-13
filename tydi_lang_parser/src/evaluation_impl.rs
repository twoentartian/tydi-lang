use std::sync::{Arc, RwLock};
use tydi_lang_raw_ast::data_type::DataType;

use tydi_lang_raw_ast::project_arch::Project;
use tydi_lang_raw_ast::scope::{Implement, Scope, StreamletType, Variable};

use crate::ParserErrorCode;
use crate::ParserErrorCode::ImplementEvaluationFail;

pub fn infer_streamlet(implement: Arc<RwLock<Implement>>, streamlet_template_exps: Vec<Arc<RwLock<Variable>>>, scope: Arc<RwLock<Scope>>, project: Arc<RwLock<Project>>) -> Result<(), ParserErrorCode> {
    let implement_scope = implement.read().unwrap().get_scope();

    //infer derived streamlet
    let streamlet_var = implement.read().unwrap().get_derived_streamlet_var();
    let streamlet_var_type = streamlet_var.read().unwrap().get_type();
    match (*streamlet_var_type.read().unwrap()).clone() {
        DataType::ProxyStreamlet(streamlet_name, template_exps) => {
            let result = implement_scope.read().unwrap().resolve_streamlet_from_scope(streamlet_name.clone());
            if result.is_err() { return Err(ImplementEvaluationFail(String::from(result.err().unwrap()))); }
            let streamlet = result.ok().unwrap();
            match streamlet.read().unwrap().get_type() {
                StreamletType::NormalStreamlet => {
                    if streamlet_template_exps.len() != 0 { return Err(ImplementEvaluationFail(format!("template expressions mismatch"))); }
                }
                StreamletType::TemplateStreamlet(template_args) => {
                    if streamlet_template_exps.len() != template_args.len() { return Err(ImplementEvaluationFail(format!("template expressions mismatch"))); }
                    for i in 0 .. streamlet_template_exps.len() {
                        let template_exp = &streamlet_template_exps[i];
                        let streamlet_arg = &template_args[i];
                        let template_exp_type_arc = template_exp.read().unwrap().get_type();
                        let streamlet_arg_type_arc = streamlet_arg.read().unwrap().get_type();
                        let template_exp_type = (*template_exp_type_arc.read().unwrap()).clone();
                        let streamlet_arg_type = (*streamlet_arg_type_arc.read().unwrap()).clone();
                        match streamlet_arg_type {
                            DataType::IntType => {
                                match template_exp_type.clone() {
                                    DataType::IntType => {

                                    }
                                    _ => { return Err(ImplementEvaluationFail(format!("template expressions mismatch, template type({}) != exp type({})", String::from(streamlet_arg_type.clone()), String::from(template_exp_type.clone())))); }
                                };
                            }
                            DataType::StringType => {
                                match template_exp_type.clone() {
                                    DataType::StringType => {

                                    }
                                    _ => { return Err(ImplementEvaluationFail(format!("template expressions mismatch, template type({}) != exp type({})", String::from(streamlet_arg_type.clone()), String::from(template_exp_type.clone())))); }
                                };
                            }
                            DataType::BoolType => {
                                match template_exp_type.clone() {
                                    DataType::BoolType => {

                                    }
                                    _ => { return Err(ImplementEvaluationFail(format!("template expressions mismatch, template type({}) != exp type({})", String::from(streamlet_arg_type.clone()), String::from(template_exp_type.clone())))); }
                                };
                            }
                            DataType::FloatType => {
                                match template_exp_type.clone() {
                                    DataType::FloatType => {

                                    }
                                    _ => { return Err(ImplementEvaluationFail(format!("template expressions mismatch, template type({}) != exp type({})", String::from(streamlet_arg_type.clone()), String::from(template_exp_type.clone())))); }
                                };
                            }
                            DataType::ArrayType(inner_type) => {}
                            DataType::LogicalDataType(logical_data) => {

                            }

                            _ => { unreachable!() }
                        };
                    }

                }
                _ => unreachable!()
            };

        }
        DataType::ExternalProxyStreamlet(package_name, streamlet_name, template_exps) => {

        }
        _ => { return Err(ImplementEvaluationFail(format!("implement must derive from a streamlet"))); }
    }


    return Ok(());
}