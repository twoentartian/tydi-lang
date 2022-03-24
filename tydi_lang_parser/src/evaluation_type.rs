use std::sync::{Arc, RwLock};
use evaluation::verify_assert;
use ParserErrorCode::TypeEvaluationFail;
use tydi_lang_raw_ast::data_type::DataType;

use tydi_lang_raw_ast::project_arch::Project;
use tydi_lang_raw_ast::scope::{LogicalDataType, LogicalStreamDirection, LogicalStreamSynchronicity, Scope, TypeAlias, Variable, VariableValue};

use crate::{ParserErrorCode};
use crate::evaluation_var::infer_variable;

pub fn infer_logical_type(logical_type: Arc<RwLock<LogicalDataType>>, scope: Arc<RwLock<Scope>>, project: Arc<RwLock<Project>>) -> Result<(),ParserErrorCode> {
    let real_type_infer = (*logical_type.read().unwrap()).clone();
    match real_type_infer {
        LogicalDataType::DummyLogicalData => {
            unreachable!()
        }
        LogicalDataType::UnknownLogicalDataType => { unreachable!() }
        LogicalDataType::ExternalLogicalDataType(package_id, type_name) => {
            //check import
            {
                let package_var_result = scope.read().unwrap().resolve_variable_from_scope(format!("$package${}", package_id.clone()));
                if package_var_result.is_err() { return Err(TypeEvaluationFail(format!("package {} not imported", package_id.clone()))); }
            }

            let project_unwrap = project.read().unwrap();
            let package = project_unwrap.packages.get(&package_id);
            if package.is_none() { return Err(TypeEvaluationFail(format!("package {} doesn't exist", package_id.clone()))); }
            let package = package.unwrap().clone();
            let package_scope = package.read().unwrap().scope.clone();
            let type_result = package_scope.read().unwrap().resolve_type_from_scope(type_name.clone());
            if type_result.is_err() { return Err(TypeEvaluationFail(String::from(type_result.err().unwrap()))); }
            let type_result = type_result.ok().unwrap();
            let infer_result = infer_type_alias(type_result.clone(), package_scope.clone(), project.clone());
            if infer_result.is_err() { return Err(infer_result.err().unwrap()); }

            let result_logical_type = type_result.read().unwrap().get_type_infer().get_raw_value();
            *logical_type.write().unwrap() = (*result_logical_type.read().unwrap()).clone();
            //type_alias.write().unwrap().set_type_infer(inferred!(infer_logical_data_type!(), result_logical_type.clone()));
        }
        LogicalDataType::DataNull => {
            //nothing to do
        }
        LogicalDataType::DataBitType(mut logical_bit) => {
            let bit_var = logical_bit.get_bit();
            let result = infer_variable(bit_var.clone(), scope.clone(), project.clone());
            if result.is_err() { return Err(result.err().unwrap()); }
            logical_bit.set_bit(bit_var);
        }
        LogicalDataType::DataGroupType(_, group_type) => {
            let group_type_clone = (*group_type.read().unwrap()).clone();
            let group_type_scope = group_type_clone.get_scope();
            for (_, single_type) in group_type_scope.read().unwrap().types.clone() {
                let result = infer_type_alias(single_type.clone(), group_type_scope.clone(), project.clone());
                if result.is_err() { return Err(result.err().unwrap()); }
            }

            //infer assert
            for (_, assert) in group_type_scope.read().unwrap().asserts.clone() {
                verify_assert(assert.clone(), group_type_scope.clone(), project.clone())?;
            }
        }
        LogicalDataType::DataUnionType(_, union_type) => {
            let union_type_clone = (*union_type.read().unwrap()).clone();
            let union_type_scope = union_type_clone.get_scope();
            for (_, single_type) in union_type_scope.read().unwrap().types.clone() {
                let result = infer_type_alias(single_type.clone(), union_type_scope.clone(), project.clone());
                if result.is_err() { return Err(result.err().unwrap()); }
            }

            //infer assert
            for (_, assert) in union_type_scope.read().unwrap().asserts.clone() {
                verify_assert(assert.clone(), union_type_scope.clone(), project.clone())?;
            }
        }
        LogicalDataType::DataStreamType(_, stream_type) => {
            let stream_copy = (*stream_type.read().unwrap()).clone();
            //data type
            {
                let data_type = stream_type.read().unwrap().get_data_type();
                let data_type = data_type.get_raw_value();
                let result = infer_logical_type(data_type, scope.clone(), project.clone());
                if result.is_err() { return Err(result.err().unwrap()); }
            }
            //dimension
            {
                let dimension = stream_copy.get_dimension();
                let result = infer_variable(dimension.clone(), scope.clone(), project.clone());
                if result.is_err() { return Err(result.err().unwrap()); }
                {
                    let dimension = dimension.read().unwrap();
                    let dimension_type = dimension.get_type();
                    if *dimension_type.read().unwrap() != DataType::IntType { return Err(TypeEvaluationFail(format!("dimension must be an integer"))); }
                }
            }
            //user type
            {
                let user_type = stream_type.read().unwrap().get_user_type();
                let user_type = user_type.get_raw_value();
                let result = infer_logical_type(user_type, scope.clone(), project.clone());
                if result.is_err() { return Err(result.err().unwrap()); }
            }
            //throughput
            {
                let throughput = stream_copy.get_throughput();
                let result = infer_variable(throughput.clone(), scope.clone(), project.clone());
                if result.is_err() { return Err(result.err().unwrap()); }
                {
                    let throughput_type = throughput.read().unwrap().get_type();
                    let throughput_type = (*throughput_type.read().unwrap()).clone();
                    if throughput_type != DataType::FloatType && throughput_type != DataType::IntType {
                        return Err(TypeEvaluationFail(format!("throughput must be an float or integer")));
                    }
                }
            }
            //synchronicity
            {
                let synchronicity = stream_copy.get_synchronicity();
                match synchronicity.clone() {
                    LogicalStreamSynchronicity::Unknown(exp) => {
                        let var = Variable::new_str(String::from(""), exp);
                        let var = Arc::new(RwLock::new(var));
                        let result = infer_variable(var.clone(), scope.clone(), project.clone());
                        if result.is_err() { return Err(result.err().unwrap()); }
                        let var_value = var.read().unwrap().get_var_value().get_raw_value();
                        match var_value {
                            VariableValue::Str(str_raw) => {
                                if str_raw.eq_ignore_ascii_case("\"sync\"") {
                                    stream_type.write().unwrap().set_synchronicity(LogicalStreamSynchronicity::Sync);
                                }
                                else if str_raw.eq_ignore_ascii_case("\"flatten\"") {
                                    stream_type.write().unwrap().set_synchronicity(LogicalStreamSynchronicity::Flatten);
                                }
                                else if str_raw.eq_ignore_ascii_case("\"desync\"") {
                                    stream_type.write().unwrap().set_synchronicity(LogicalStreamSynchronicity::Desync);
                                }
                                else if str_raw.eq_ignore_ascii_case("\"flatdesync\"") {
                                    stream_type.write().unwrap().set_synchronicity(LogicalStreamSynchronicity::FlatDesync);
                                }
                                else {
                                    return Err(TypeEvaluationFail(format!("synchronicity must be a string of \"Sync\", \"Flatten\", \"Desync\" or \"FlatDesync\"")));
                                }
                            }
                            _ => { return Err(TypeEvaluationFail(format!("synchronicity must be a string of \"Sync\", \"Flatten\", \"Desync\" or \"FlatDesync\""))); }
                        }
                    }
                    _ => {
                        //do nothing
                    }
                }
            }
            //complexity
            {
                let complexity = stream_copy.get_complexity();
                let result = infer_variable(complexity.clone(), scope.clone(), project.clone());
                if result.is_err() { return Err(result.err().unwrap()); }
                {
                    let complexity_r = complexity.read().unwrap();
                    if *complexity_r.get_type().read().unwrap() != DataType::IntType { return Err(TypeEvaluationFail(format!("complexity must be an integer"))); }

                    match complexity_r.get_var_value().get_raw_value() {
                        VariableValue::Int(value) => {
                            if value > 7 || value < 1 { return Err(TypeEvaluationFail(format!("1 <= complexity level <= 7"))); }
                        }
                        _ => unreachable!()
                    }
                }
            }
            //direction
            {
                let direction = stream_copy.get_direction();
                match direction.clone() {
                    LogicalStreamDirection::Unknown(exp) => {
                        let var = Variable::new_str(String::from(""), exp);
                        let var = Arc::new(RwLock::new(var));
                        let result = infer_variable(var.clone(), scope.clone(), project.clone());
                        if result.is_err() { return Err(result.err().unwrap()); }
                        let var_value = var.read().unwrap().get_var_value().get_raw_value();
                        match var_value {
                            VariableValue::Str(str_raw) => {
                                if str_raw.eq_ignore_ascii_case("\"forward\"") {
                                    stream_type.write().unwrap().set_direction(LogicalStreamDirection::Forward);
                                }
                                else if str_raw.eq_ignore_ascii_case("\"reverse\"") {
                                    stream_type.write().unwrap().set_direction(LogicalStreamDirection::Reverse);
                                }
                                else {
                                    return Err(TypeEvaluationFail(format!("stream direction must be a string of \"forward\", or \"reverse\"")));
                                }
                            }
                            _ => { return Err(TypeEvaluationFail(format!("stream direction must be a string of \"forward\", or \"reverse\""))); }
                        }
                    }
                    _ => {
                        //do nothing
                    }
                }
            }
            //keep
            {
                let keep = stream_copy.get_keep();
                let result = infer_variable(keep.clone(), scope.clone(), project.clone());
                if result.is_err() { return Err(result.err().unwrap()); }
                {
                    let keep_type = keep.read().unwrap().get_type();
                    if *keep_type.read().unwrap() != DataType::BoolType { return Err(TypeEvaluationFail(format!("complexity must be a bool"))); }
                }
            }
        }
        LogicalDataType::DataUserDefinedVarType(name) => {
            let result = scope.read().unwrap().resolve_type_from_scope(name.clone());
            if result.is_err() {
                //resolve in var, it might be a dummy logical type
                let result_in_dummy = scope.read().unwrap().resolve_variable_from_scope(name.clone());
                if result_in_dummy.is_err() { return Err(TypeEvaluationFail(format!("type {} not found", name.clone()))); }
                let result_in_dummy = result_in_dummy.ok().unwrap();
                let result_in_dummy_type = result_in_dummy.read().unwrap().get_type();
                match (*result_in_dummy_type.read().unwrap()).clone() {
                    DataType::LogicalDataType(logical_type) => {
                        match (*logical_type.read().unwrap()) .clone() {
                            LogicalDataType::DummyLogicalData => {
                                //resolve the variable expression as a VarType
                                let var_type_expression = result_in_dummy.read().unwrap().get_var_value().get_raw_exp();
                                let temp_type_var = Arc::new(RwLock::new(LogicalDataType::DataUserDefinedVarType(var_type_expression)));
                                let result = infer_logical_type(temp_type_var.clone(), scope.clone(), project.clone());
                                if result.is_err() { return Err(TypeEvaluationFail(format!("type {} is declared as template arg but not found its instantiation", name.clone()))); }
                                //assign to the original type arc
                                *logical_type.write().unwrap() = (*temp_type_var.read().unwrap()).clone();
                            }
                            _ => { return Err(TypeEvaluationFail(format!("type {} not found", name.clone()))); }
                        }
                    }
                    _ => { return Err(TypeEvaluationFail(format!("type {} not found", name.clone()))); }
                };
            }
            else {
                let resolve_result = result.ok().unwrap();
                let result_logical_type = resolve_result.read().unwrap().get_type_infer().get_raw_value();

                //infer the inner logical type
                let result = infer_logical_type(result_logical_type.clone(), scope.clone(), project.clone());
                if result.is_err() { return Err(result.err().unwrap()); }
                //assign to the original type arc
                *logical_type.write().unwrap() = (*result_logical_type.read().unwrap()).clone();
            }
        }
    }

    return Ok(());
}

pub fn infer_type_alias(type_alias: Arc<RwLock<TypeAlias>>, scope: Arc<RwLock<Scope>>, project: Arc<RwLock<Project>>) -> Result<(), ParserErrorCode> {
    let type_clone = (*type_alias.read().unwrap()).clone();
    let real_type_infer = type_clone.get_type_infer().get_raw_value();
    return infer_logical_type(real_type_infer, scope.clone(), project.clone());
}