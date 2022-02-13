use std::sync::{Arc, RwLock, RwLockReadGuard};
use ParserErrorCode::StreamletEvaluationFail;
use tydi_lang_raw_ast::data_type::DataType;
use tydi_lang_raw_ast::{inferred, infer_logical_data_type};
use tydi_lang_raw_ast::inferable::{NewInferable, Inferable};
use tydi_lang_raw_ast::logical_data_type::LogicalDataType;
use tydi_lang_raw_ast::port::PortArray;

use tydi_lang_raw_ast::scope::{Streamlet, Scope, StreamletType, VariableValue, Variable, Port};
use tydi_lang_raw_ast::project_arch::Project;

use crate::{evaluation_type, evaluation_var, ParserErrorCode};

pub fn infer_streamlet(streamlet: Arc<RwLock<Streamlet>>, streamlet_template_exps: Vec<Arc<RwLock<Variable>>>, scope: Arc<RwLock<Scope>>, project: Arc<RwLock<Project>>) -> Result<(), ParserErrorCode> {
    let streamlet_copy = (*streamlet.read().unwrap()).clone();
    match streamlet_copy.get_type() {
        StreamletType::UnknownType => { unreachable!() }
        StreamletType::NormalStreamlet => {
            if streamlet_template_exps.len() != 0 { return Err(StreamletEvaluationFail(format!("normal streamlet cannot have template expressions"))); }

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
                                    if value <= 0 { return Err(StreamletEvaluationFail(format!("the length streamlet port array must be a positive number"))); }
                                    let port_type = port_read.get_type().get_raw_value();
                                    for i in 0..value {
                                        let result = streamlet_scope.write().unwrap().new_port(format!("{}@{}", port_read.get_name(), i.to_string()), inferred!(infer_logical_data_type!(), port_type.clone()), port_read.get_direction());
                                        if result.is_err() { return Err(StreamletEvaluationFail(String::from(result.err().unwrap()))); }
                                        //expand_ports.push(Arc::new(RwLock::new(Port::new(format!("{}@{}", port_read.get_name(), i.to_string()), inferred!(infer_logical_data_type!(), port_type.clone()), port_read.get_direction()))));
                                    }
                                }
                                _ => return Err(StreamletEvaluationFail(format!("the length of a port array must be an integer")))
                            }
                        }
                    }
                }
            }
        }
        StreamletType::TemplateStreamlet(template_args) => {
            //the template streamlet should never be inferred, only instantiated streamlet can be inferred.
        }
    }

    return Ok(());
}