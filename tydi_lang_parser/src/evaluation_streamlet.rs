use std::sync::{Arc, RwLock};
use evaluation_var::infer_variable;
use ParserErrorCode::StreamletEvaluationFail;
use tydi_lang_raw_ast::data_type::DataType;
use tydi_lang_raw_ast::deep_clone::DeepClone;
use tydi_lang_raw_ast::evaluated_flag::{EvaluatedFlag, EvaluatedState};
use tydi_lang_raw_ast::logical_data_type::LogicalDataType;
use tydi_lang_raw_ast::port::PortArray;

use tydi_lang_raw_ast::scope::{Streamlet, Scope, StreamletType, VariableValue, Variable, ErrorCode};
use tydi_lang_raw_ast::project_arch::Project;

use crate::{evaluation_type, evaluation_var, ParserErrorCode};

pub fn infer_streamlet(streamlet: Arc<RwLock<Streamlet>>, streamlet_template_exps: Vec<Arc<RwLock<Variable>>>, scope: Arc<RwLock<Scope>>, project: Arc<RwLock<Project>>) -> Result<Arc<RwLock<Streamlet>>, ParserErrorCode> {
    let streamlet_copy = (*streamlet.read().unwrap()).clone();
    match streamlet_copy.get_type() {
        StreamletType::UnknownType => { unreachable!() }
        StreamletType::NormalStreamlet => {
            if streamlet_template_exps.len() != 0 { return Err(StreamletEvaluationFail(format!("normal streamlet cannot have template expressions"))); }

            let mut sleep = false;
            loop {
                if sleep { std::thread::sleep(std::time::Duration::from_micros(10)); }
                let mut streamlet_write = streamlet.write().unwrap();
                let evaluate_state = streamlet_write.get_evaluate_flag();
                match evaluate_state {
                    EvaluatedState::NotEvaluate => {
                        //evaluating
                        streamlet_write.set_evaluate_flag(EvaluatedState::Evaluating);
                        break;
                    }
                    EvaluatedState::Evaluating => {
                        sleep = true;
                        continue;
                    }
                    EvaluatedState::Evaluated => {
                        return Ok(streamlet.clone());
                    }
                }
            }

            let streamlet_scope = streamlet.read().unwrap().get_scope();
            let streamlet_ports = streamlet_scope.read().unwrap().ports.clone();
            for (_, port) in streamlet_ports {
                let port_read = port.read().unwrap();
                //infer port type
                {
                    let port_type = port_read.get_type().get_raw_value();
                    let result = evaluation_type::infer_logical_type(port_type.clone(), streamlet_scope.clone(), project.clone());
                    if result.is_err() { return Err(result.err().unwrap()); }
                    match (*port_type.read().unwrap()).clone() {
                        LogicalDataType::DataStreamType(_, _) => { /*correct*/ }
                        _ => { return Err(StreamletEvaluationFail(format!("the logical type of streamlet port must be a stream"))); }
                    };
                }

                //expand port array
                {
                    let port_array = port_read.get_array_type();
                    match port_array {
                        PortArray::UnknownPortArray => { unreachable!() }
                        PortArray::SinglePort => { /*do nothing*/ }
                        PortArray::ArrayPort(array_var) => {
                            let result = evaluation_var::infer_variable(array_var.clone(), streamlet_scope.clone(), project.clone());
                            if result.is_err() { return Err(result.err().unwrap()); }

                            let array_var_read = array_var.read().unwrap();
                            let array_var_value = array_var_read.get_var_value().get_raw_value();
                            match array_var_value {
                                VariableValue::Int(value) => {
                                    if value <= 0 { return Err(StreamletEvaluationFail(format!("the length of streamlet port array must be a positive number"))); }
                                    for i in 0..value {
                                        let mut new_port = (*port.read().unwrap()).clone();
                                        new_port.set_name(format!("{}@{}", port_read.get_name(), i.to_string()));
                                        new_port.set_array_type(PortArray::SinglePort);
                                        //let result = streamlet_scope.write().unwrap().new_port(format!("{}@{}", port_read.get_name(), i.to_string()), inferred!(infer_logical_data_type!(), port_type.clone()), port_read.get_direction());
                                        let result = streamlet_scope.write().unwrap().with_port(Arc::new(RwLock::new(new_port)));
                                        if result.is_err() { return Err(StreamletEvaluationFail(String::from(result.err().unwrap()))); }
                                    }
                                    //remove the array port in scope
                                    {
                                        streamlet_scope.write().unwrap().ports.remove(&port_read.get_name()).unwrap();
                                    }
                                }
                                _ => return Err(StreamletEvaluationFail(format!("the length of a port array must be an integer")))
                            }
                        }
                    }
                }
            }

            {
                streamlet.write().unwrap().set_evaluate_flag(EvaluatedState::Evaluated);
            }
            return Ok(streamlet.clone());
        }
        StreamletType::TemplateStreamlet(template_args) => {
            //infer template expressions
            for template_exp in &streamlet_template_exps {
                infer_variable(template_exp.clone(), scope.clone(), project.clone())?;
            }

            //get instantiate template name
            let streamlet_instance_name = crate::util::generate_template_instance_name(streamlet.read().unwrap().get_name(), &streamlet_template_exps);

            //clone / instantiate streamlet
            let mut cloned_streamlet = streamlet.read().unwrap().deep_clone();
            cloned_streamlet.set_name(streamlet_instance_name);
            cloned_streamlet.set_type(StreamletType::NormalStreamlet);
            let cloned_streamlet = Arc::new(RwLock::new(cloned_streamlet));
            {
                let basic_scope = crate::util::goto_basic_scope(scope.clone())?;
                let result = basic_scope.write().unwrap().with_streamlet(cloned_streamlet.clone());
                if result.is_err() {
                    match result.clone().err().unwrap() {
                        ErrorCode::IdRedefined(_) => { /*that streamlet might have already exists, so we don't check result*/ }
                        _ => return Err(StreamletEvaluationFail(String::from(result.err().unwrap())))
                    }
                }
            }

            //remove the template var in scope
            let cloned_streamlet_scope = cloned_streamlet.read().unwrap().get_scope();
            for i in 0 .. template_args.len() {
                let name = template_args[i].read().unwrap().get_name();
                let index = name.find(&*crate::built_in_ids::ARG_PREFIX).unwrap();
                let name = (&name[index+5 ..]).to_string();
                let result = cloned_streamlet_scope.write().unwrap().vars.remove(&name);
                match result {
                    None => { unreachable!() }
                    Some(_) => {}
                }
            }

            //create corresponding linking var
            if streamlet_template_exps.len() != template_args.len() { return Err(StreamletEvaluationFail(format!("template expressions mismatch"))); }
            for i in 0 .. streamlet_template_exps.len() {
                let template_exp = &streamlet_template_exps[i];
                let streamlet_arg = &template_args[i];
                let template_exp_type_arc = template_exp.read().unwrap().get_type();
                let streamlet_arg_type_arc = streamlet_arg.read().unwrap().get_type();
                let template_exp_type = (*template_exp_type_arc.read().unwrap()).clone();
                let streamlet_arg_type = (*streamlet_arg_type_arc.read().unwrap()).clone();
                if streamlet_arg_type != template_exp_type.clone() { return Err(StreamletEvaluationFail(format!("template expressions mismatch, template type({}) != exp type({})", String::from(streamlet_arg_type.clone()), String::from(template_exp_type.clone())))); }
                let linking_var_name = streamlet_arg.read().unwrap().get_name();
                let linking_var_name_index = linking_var_name.find(&*crate::built_in_ids::ARG_PREFIX).unwrap();
                let linking_var_name = (&linking_var_name[linking_var_name_index+5 ..]).to_string();
                match streamlet_arg_type.clone() {
                    DataType::IntType | DataType::StringType | DataType::BoolType | DataType::FloatType | DataType::ArrayType(_) => {
                        let linking_var = Arc::new(RwLock::new(Variable::new_with_value(linking_var_name.clone(), streamlet_arg_type.clone(), template_exp.read().unwrap().get_var_value().get_raw_value())));
                        let result = cloned_streamlet_scope.write().unwrap().with_variable(linking_var);
                        if result.is_err() { return Err(StreamletEvaluationFail(format!("failed to create linking variable({}): {}", linking_var_name.clone(), String::from(result.err().unwrap())))); }
                    }
                    DataType::LogicalDataType(_) => {
                        match template_exp_type {
                            DataType::LogicalDataType(logical_data_type) => {
                                let result = cloned_streamlet_scope.write().unwrap().new_logical_data_type(linking_var_name.clone(), (*logical_data_type.read().unwrap()).clone());
                                if result.is_err() { return Err(StreamletEvaluationFail(format!("failed to create linking type({}): {}", linking_var_name.clone(), String::from(result.err().unwrap())))); }
                            },
                            _ => unreachable!(),
                        }
                    }
                    _ => { unreachable!() }
                };
            }

            //evaluation the new generated streamlet
            infer_streamlet(cloned_streamlet.clone(), vec![], scope.clone(), project.clone())?;

            return Ok(cloned_streamlet.clone());
        }
    }

}

pub fn resolve_and_infer_streamlet(streamlet_name: String, package_name: Option<String>, streamlet_template_exps: Vec<Arc<RwLock<Variable>>>, scope: Arc<RwLock<Scope>>, project: Arc<RwLock<Project>>) -> Result<Arc<RwLock<Streamlet>>, ParserErrorCode> {
     match package_name {
        None => {
            let result = scope.read().unwrap().resolve_streamlet_from_scope(streamlet_name.clone());
            if result.is_err() { return Err(StreamletEvaluationFail(String::from(result.err().unwrap()))); }
            let streamlet = result.ok().unwrap();
            return infer_streamlet(streamlet.clone(), streamlet_template_exps.clone(), scope.clone(), project.clone())
        }
        Some(package_name) => {
            //check import
            {
                let package_var_result = scope.read().unwrap().resolve_variable_from_scope(format!("{}{}", *crate::built_in_ids::PACKAGE_PREFIX, package_name.clone()));
                if package_var_result.is_err() { return Err(StreamletEvaluationFail(format!("package {} not imported", package_name.clone()))); }
            }
            let project_read = project.read().unwrap();
            let external_package = project_read.packages.get(&package_name);
            if external_package.is_none() { return Err(StreamletEvaluationFail(format!("package {} not found", package_name.clone()))); }
            let external_package = external_package.unwrap();
            let external_scope = external_package.read().unwrap().get_scope();

            let result = external_scope.read().unwrap().resolve_streamlet_in_current_scope(streamlet_name.clone());
            if result.is_err() { return Err(StreamletEvaluationFail(String::from(result.err().unwrap()))); }
            let streamlet = result.ok().unwrap();
            return infer_streamlet(streamlet.clone(), streamlet_template_exps.clone(), scope.clone(), project.clone())
        }
    }
}
