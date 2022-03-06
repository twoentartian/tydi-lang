use std::sync::{Arc, RwLock};
use ParserErrorCode::ExpressionEvaluationFail;
use pest::{Parser};
use pest::iterators::{Pairs, Pair};
use pest::prec_climber::{PrecClimber};
use pest::prec_climber::Assoc::{Left, Right};
use pest::prec_climber as pcl;
use tydi_lang_raw_ast::logical_data_type::LogicalDataType;
use tydi_lang_raw_ast::project_arch::Project;
use tydi_lang_raw_ast::scope::{Scope, Variable, DataType, InferState, VariableValue};
use tydi_lang_raw_ast::variable::ClockDomainValue;
use crate::ParserErrorCode;
use crate::evaluation_type;

type Rule = crate::Rule;
type TydiParser = crate::TydiParser;

lazy_static! {
    static ref PREC_CLIMBER: PrecClimber<Rule> = {
        PrecClimber::new(vec![
            pcl::Operator::new(Rule::OP_LogicalEq, Left) | pcl::Operator::new(Rule::OP_LogicalNotEq, Left) |
            pcl::Operator::new(Rule::OP_Greater, Left) | pcl::Operator::new(Rule::OP_GreaterEq, Left) |
            pcl::Operator::new(Rule::OP_Less, Left) | pcl::Operator::new(Rule::OP_LessEq, Left) |

            // 3
            pcl::Operator::new(Rule::OP_LeftShift, Left) | pcl::Operator::new(Rule::OP_RightShift, Left) |
            pcl::Operator::new(Rule::OP_Add, Left) | pcl::Operator::new(Rule::OP_Minus, Left) |
            pcl::Operator::new(Rule::OP_BitAnd, Left) | pcl::Operator::new(Rule::OP_BitOr, Left) |
            pcl::Operator::new(Rule::OP_LogicalAnd, Left) | pcl::Operator::new(Rule::OP_LogicalOr, Left),
            // 4
            pcl::Operator::new(Rule::OP_Multiply, Left) | pcl::Operator::new(Rule::OP_Divide, Left) | pcl::Operator::new(Rule::OP_Mod, Left),
            // 5
            pcl::Operator::new(Rule::OP_Power, Right)
        ])
    };
}

fn eval_explog(explog: Pairs<Rule>, scope: Arc<RwLock<Scope>>, project: Arc<RwLock<Project>>) -> Result<Variable, ParserErrorCode> {
    let mut base_var = Variable::new_float(String::from(""), 0.0);
    let mut log_var = Variable::new_float(String::from(""), 0.0);
    let mut is_base = true;
    for exp in explog.into_iter() {
        match exp.clone().as_rule() {
            Rule::Exp => {
                if is_base {
                    is_base = false;
                    let base_result = eval_exp(exp.into_inner(), scope.clone(), project.clone());
                    if base_result.is_err() { return Err(base_result.err().unwrap()); }
                    base_var = base_result.ok().unwrap();
                }
                else {
                    let log_result = eval_exp(exp.into_inner(), scope.clone(), project.clone());
                    if log_result.is_err() { return Err(log_result.err().unwrap()); }
                    log_var = log_result.ok().unwrap();
                }
            }
            _ => { unreachable!() }
        }
    }

    //infer values
    let base_raw_value: f32;
    let log_raw_value: f32;
    {
        let base_val = base_var.get_var_value();
        if base_val.get_infer_state() == InferState::NotInferred {
            let result = parse_eval_exp(base_val.get_raw_exp(), scope.clone(), project.clone());
            if result.is_err() { return Err(result.err().unwrap()); }
            let new_base_val = result.ok().unwrap();

            //check the data type of base exp
            let new_base_val_type = new_base_val.get_type();
            let var_type = new_base_val_type.read().unwrap();
            if *var_type != DataType::FloatType && *var_type != DataType::IntType {
                return Err(ExpressionEvaluationFail(format!("log{}({}) can only be applied to int and float", String::from(base_var.clone()), String::from(log_var.clone()))));
            }
            base_var.set_var_value(new_base_val.get_var_value());
        }
        match base_val.get_raw_value() {
            VariableValue::Int(v) => {base_raw_value = v as f32},
            VariableValue::Float(v) => {base_raw_value = v},
            _ => { unreachable!() }
        }

        let log_val = log_var.get_var_value();
        if log_val.get_infer_state() == InferState::NotInferred {
            let result = parse_eval_exp(log_val.get_raw_exp(), scope.clone(), project.clone());
            if result.is_err() { return Err(result.err().unwrap()); }
            let new_log_val = result.ok().unwrap();

            //check the data type of log exp
            let new_log_val_type = new_log_val.get_type();
            let var_type = new_log_val_type.read().unwrap();
            if *var_type != DataType::FloatType && *var_type != DataType::IntType {
                return Err(ExpressionEvaluationFail(format!("log{}({}) can only be applied to int and float", String::from(base_var.clone()), String::from(log_var.clone()))));
            }
            log_var.set_var_value(new_log_val.get_var_value());
        }
        match log_val.get_raw_value() {
            VariableValue::Int(v) => {log_raw_value = v as f32},
            VariableValue::Float(v) => {log_raw_value = v},
            _ => { unreachable!() }
        }
    }

    //compute
    let output_val = (log_raw_value as f32).log(base_raw_value);
    return Ok(Variable::new_float(String::from(""), output_val));
}

fn eval_exp_bitwisenot(exp_bitwisenot: Pairs<Rule>, scope: Arc<RwLock<Scope>>, project: Arc<RwLock<Project>>) -> Result<Variable, ParserErrorCode> {
    let mut base_var = Variable::new_int(String::from(""), 0);
    let mut value_var = Variable::new_int(String::from(""), 0);
    let mut is_base = true;
    for exp in exp_bitwisenot.into_iter() {
        match exp.clone().as_rule() {
            Rule::Exp => {
                if is_base {
                    is_base = false;
                    let result = eval_exp(exp.into_inner(), scope.clone(), project.clone());
                    if result.is_err() { return Err(result.err().unwrap()); }
                    base_var = result.ok().unwrap();
                }
                else {
                    let result = eval_exp(exp.into_inner(), scope.clone(), project.clone());
                    if result.is_err() { return Err(result.err().unwrap()); }
                    value_var = result.ok().unwrap();
                }
            }
            _ => unreachable!()
        }
    }

    //check
    let base = match base_var.get_var_value().get_raw_value() {
        VariableValue::Int(i) => { i }
        _ => { return Err(ExpressionEvaluationFail(format!("bitwiseNot only acceptes integers"))); }
    };
    let value = match value_var.get_var_value().get_raw_value() {
        VariableValue::Int(i) => { i }
        _ => { return Err(ExpressionEvaluationFail(format!("bitwiseNot only acceptes integers"))); }
    };
    if base <= 0 { return Err(ExpressionEvaluationFail(format!("the base of BitwiseNot operation must be positive"))); }
    if value < 0 { return Err(ExpressionEvaluationFail(format!("the value of BitwiseNot operation must be positive or zero"))); }

    let output_value = (2 as i32).pow(base as u32) - value;
    if output_value < 0 { return Err(ExpressionEvaluationFail(format!("the value of BitwiseNot operation has larger bit width than the base width"))); }
    return Ok(Variable::new_int(String::from(""), output_value));
}

fn eval_exp_int(int_exp: Pairs<Rule>, _: Arc<RwLock<Scope>>, _: Arc<RwLock<Project>>) -> Result<Variable, ParserErrorCode> {
    for i in int_exp.into_iter() {
        let int_val = i.as_str().to_string().replace('_', "");
        match i.as_rule() {
            Rule::INT_RAW_NORAML => {
                let output = i64::from_str_radix(&int_val, 10);
                if output.is_err() { return Err(ExpressionEvaluationFail(output.err().unwrap().to_string())); }
                else { return Ok(Variable::new_int(String::from(""), output.ok().unwrap() as i32)); }
            },
            Rule::INT_RAW_HEX => {
                let int_val = int_val.replace("0x", "");
                let output = i64::from_str_radix(&int_val, 16);
                if output.is_err() { return Err(ExpressionEvaluationFail(output.err().unwrap().to_string())); }
                else { return Ok(Variable::new_int(String::from(""), output.ok().unwrap() as i32)); }
            },
            Rule::INT_RAW_BIN => {
                let int_val = int_val.replace("0b", "");
                let output = i64::from_str_radix(&int_val, 2);
                if output.is_err() { return Err(ExpressionEvaluationFail(output.err().unwrap().to_string())); }
                else { return Ok(Variable::new_int(String::from(""), output.ok().unwrap() as i32)); }
            },
            Rule::INT_RAW_OCT => {
                let int_val = int_val.replace("0o", "");
                let output = i64::from_str_radix(&int_val, 8);
                if output.is_err() { return Err(ExpressionEvaluationFail(output.err().unwrap().to_string())); }
                else { return Ok(Variable::new_int(String::from(""), output.ok().unwrap() as i32)); }
            },
            _ => { unreachable!() },
        }
    }
    unreachable!()
}

fn eval_exp_unary(unary_exp: Pairs<Rule>, scope: Arc<RwLock<Scope>>, project: Arc<RwLock<Project>>) -> Result<Variable, ParserErrorCode> {
    let mut unary_op = String::from("");
    let mut var = Variable::new_int(String::from(""), 0);
    for item in unary_exp.into_iter() {
        match item.as_rule() {
            Rule::UnaryOp => {
                unary_op = item.as_str().to_string();
            }
            Rule::Exp => {
                let result = eval_exp(item.into_inner(), scope.clone(), project.clone());
                if result.is_err() { return Err(result.err().unwrap()); }
                var = result.ok().unwrap();
            }
            _ => unreachable!()
        }
    }
    if unary_op == "-" {
        match var.get_var_value().get_raw_value() {
            VariableValue::Int(v) => {
                return Ok(Variable::new_int(String::from(""), -v));
            }
            VariableValue::Float(v) => {
                return Ok(Variable::new_float(String::from(""), -v));
            }
            _ => return Err(ExpressionEvaluationFail(format!("- operator only supports int and float")))
        }
    }
    else if unary_op == "!" {
        match var.get_var_value().get_raw_value() {
            VariableValue::Bool(v) => {
                return Ok(Variable::new_bool(String::from(""), !v));
            }
            _ => return Err(ExpressionEvaluationFail(format!("! operator only supports bool")))
        }
    }
    else {
        unreachable!()
    }
}

fn eval_op_equal(lhs:Variable, rhs: Variable, lhs_type: DataType, rhs_type: DataType) -> Result<bool, ParserErrorCode> {
    if lhs_type == DataType::IntType && rhs_type == DataType::IntType {
        let lhs_value = lhs.get_var_value().get_raw_value();
        let rhs_value = rhs.get_var_value().get_raw_value();
        let lhs;
        let rhs;
        match lhs_value {
            VariableValue::Int(i0) => { lhs = i0; }
            _ => unreachable!()
        }
        match rhs_value {
            VariableValue::Int(i1) => { rhs = i1; }
            _ => unreachable!()
        }
        return Ok(lhs == rhs);
    }
    else if lhs_type == DataType::BoolType && rhs_type == DataType::BoolType {
        let lhs_value = lhs.get_var_value().get_raw_value();
        let rhs_value = rhs.get_var_value().get_raw_value();
        let lhs;
        let rhs;
        match lhs_value {
            VariableValue::Bool(i0) => { lhs = i0; }
            _ => unreachable!()
        }
        match rhs_value {
            VariableValue::Bool(i1) => { rhs = i1; }
            _ => unreachable!()
        }
        return Ok(lhs == rhs);
    }
    else if lhs_type == DataType::StringType && rhs_type == DataType::StringType {
        let lhs_value = lhs.get_var_value().get_raw_value();
        let rhs_value = rhs.get_var_value().get_raw_value();
        let lhs;
        let rhs;
        match lhs_value {
            VariableValue::Str(i0) => { lhs = i0; }
            _ => unreachable!()
        }
        match rhs_value {
            VariableValue::Str(i1) => { rhs = i1; }
            _ => unreachable!()
        }
        return Ok(lhs == rhs);
    }
    else if lhs_type == DataType::FloatType && rhs_type == DataType::FloatType {
        let lhs_value = lhs.get_var_value().get_raw_value();
        let rhs_value = rhs.get_var_value().get_raw_value();
        let lhs;
        let rhs;
        match lhs_value {
            VariableValue::Float(i0) => { lhs = i0; }
            _ => unreachable!()
        }
        match rhs_value {
            VariableValue::Float(i1) => { rhs = i1; }
            _ => unreachable!()
        }
        return Ok(lhs == rhs);
    }
    else {
        return Err(ExpressionEvaluationFail(format!("== / != operator does not support {} + {}", String::from(lhs_type), String::from(rhs_type))));
    }
}

fn eval_op_greater(lhs:Variable, rhs: Variable, lhs_type: DataType, rhs_type: DataType) -> Result<bool, ParserErrorCode> {
    if lhs_type == DataType::IntType && rhs_type == DataType::IntType {
        let lhs_value = lhs.get_var_value().get_raw_value();
        let rhs_value = rhs.get_var_value().get_raw_value();
        let lhs;
        let rhs;
        match lhs_value {
            VariableValue::Int(i0) => { lhs = i0; }
            _ => unreachable!()
        }
        match rhs_value {
            VariableValue::Int(i1) => { rhs = i1; }
            _ => unreachable!()
        }
        return Ok(lhs > rhs);
    }
    else if (lhs_type == DataType::FloatType && rhs_type == DataType::FloatType) ||
        (lhs_type == DataType::FloatType && rhs_type == DataType::IntType) ||
        (lhs_type == DataType::IntType && rhs_type == DataType::FloatType) {
        let lhs_value = lhs.get_var_value().get_raw_value();
        let rhs_value = rhs.get_var_value().get_raw_value();
        let lhs;
        let rhs;
        match lhs_value {
            VariableValue::Int(v) => { lhs = v as f32; }
            VariableValue::Float(v) => { lhs = v; }
            _ => unreachable!()
        }
        match rhs_value {
            VariableValue::Int(v) => { rhs = v as f32; }
            VariableValue::Float(v) => { rhs = v; }
            _ => unreachable!()
        }
        return Ok(lhs > rhs);
    }
    else {
        return Err(ExpressionEvaluationFail(format!("> / <= operator does not support {} + {}", String::from(lhs_type), String::from(rhs_type))));
    }
}

fn eval_op_less(lhs:Variable, rhs: Variable, lhs_type: DataType, rhs_type: DataType) -> Result<bool, ParserErrorCode> {
    if lhs_type == DataType::IntType && rhs_type == DataType::IntType {
        let lhs_value = lhs.get_var_value().get_raw_value();
        let rhs_value = rhs.get_var_value().get_raw_value();
        let lhs;
        let rhs;
        match lhs_value {
            VariableValue::Int(i0) => { lhs = i0; }
            _ => unreachable!()
        }
        match rhs_value {
            VariableValue::Int(i1) => { rhs = i1; }
            _ => unreachable!()
        }
        return Ok(lhs < rhs);
    }
    else if (lhs_type == DataType::FloatType && rhs_type == DataType::FloatType) ||
        (lhs_type == DataType::FloatType && rhs_type == DataType::IntType) ||
        (lhs_type == DataType::IntType && rhs_type == DataType::FloatType) {
        let lhs_value = lhs.get_var_value().get_raw_value();
        let rhs_value = rhs.get_var_value().get_raw_value();
        let lhs;
        let rhs;
        match lhs_value {
            VariableValue::Int(v) => { lhs = v as f32; }
            VariableValue::Float(v) => { lhs = v; }
            _ => unreachable!()
        }
        match rhs_value {
            VariableValue::Int(v) => { rhs = v as f32; }
            VariableValue::Float(v) => { rhs = v; }
            _ => unreachable!()
        }
        return Ok(lhs < rhs);
    }
    else {
        return Err(ExpressionEvaluationFail(format!("< / >= operator does not support {} + {}", String::from(lhs_type), String::from(rhs_type))));
    }
}

/// support types within self packages or in external packages
fn eval_exp_in_type(term: Pairs<Rule>, scope: Arc<RwLock<Scope>>, project: Arc<RwLock<Project>>) -> Result<Variable, ParserErrorCode> {
    let mut name_vec: Vec<String> = vec![];
    for term_inner in term.clone().into_iter() {
        match term_inner.clone().as_rule() {
            Rule::ID => {
                name_vec.push(term_inner.as_str().to_string());
            },
            _ => unreachable!()
        }
    }

    //get target scope
    let target_scope;
    if name_vec.len() == 2 {
        target_scope = scope.clone();
    }
    else if name_vec.len() == 3 {
        //type in external package
        let external_package = project.read().unwrap().find_package(name_vec[0].clone());
        let external_package = external_package.ok().unwrap();
        let external_package_scope = external_package.read().unwrap().get_scope();
        target_scope = external_package_scope;
        let mut name_vec_without_package_name: Vec<String> = vec![];
        name_vec_without_package_name.push(name_vec[1].clone());
        name_vec_without_package_name.push(name_vec[2].clone());
        name_vec = name_vec_without_package_name;
    }
    else {
        unreachable!()
    }

    //resolve type
    let type_result = target_scope.read().unwrap().resolve_type_from_scope(name_vec[0].clone());
    if type_result.is_err() { return Err(ExpressionEvaluationFail(String::from(type_result.err().unwrap()))); }
    let resolved_type = type_result.ok().unwrap();

    //infer type
    crate::evaluation_type::infer_type_alias(resolved_type.clone(), target_scope.clone(), project.clone())?;
    let resolved_logical_type = resolved_type.read().unwrap().get_type_infer().get_raw_value();

    return match (*resolved_logical_type.read().unwrap()).clone() {
        LogicalDataType::DataGroupType(_, logical_group) => {
            let type_scope = logical_group.read().unwrap().get_scope();
            let output_var_result = type_scope.read().unwrap().resolve_variable_in_current_scope(name_vec[1].clone());
            if output_var_result.is_err() { return Err(ExpressionEvaluationFail(String::from(output_var_result.err().unwrap()))); }
            let output_var = output_var_result.ok().unwrap();
            infer_variable(output_var.clone(), target_scope.clone(), project.clone())?;
            let output_var_cloned = (*output_var.read().unwrap()).clone();
            Ok(output_var_cloned)
        }
        LogicalDataType::DataUnionType(_, logical_union) => {
            let type_scope = logical_union.read().unwrap().get_scope();
            let output_var_result = type_scope.read().unwrap().resolve_variable_in_current_scope(name_vec[1].clone());
            if output_var_result.is_err() { return Err(ExpressionEvaluationFail(String::from(output_var_result.err().unwrap()))); }
            let output_var = output_var_result.ok().unwrap();
            infer_variable(output_var.clone(), target_scope.clone(), project.clone())?;
            let output_var_cloned = (*output_var.read().unwrap()).clone();
            Ok(output_var_cloned)
        }
        _ => Err(ExpressionEvaluationFail(format!("only logical group and union can have const variable in types")))
    };
}

/// support implements within self packages or in external packages
fn eval_exp_in_implement(term: Pairs<Rule>, scope: Arc<RwLock<Scope>>, project: Arc<RwLock<Project>>) -> Result<Variable, ParserErrorCode> {
    let mut name_vec: Vec<String> = vec![];
    let mut arg_exps: Option<Vec<Arc<RwLock<Variable>>>> = None;
    for term_inner in term.clone().into_iter() {
        match term_inner.clone().as_rule() {
            Rule::ID => {
                name_vec.push(term_inner.as_str().to_string());
            },
            Rule::ArgExps => {
                arg_exps = Some(crate::parse_argexps(term_inner.into_inner(), scope.clone())?);
            }
            _ => unreachable!()
        }
    }

    //get target scope
    let target_scope;
    if name_vec.len() == 2 {
        target_scope = scope.clone();
    }
    else if name_vec.len() == 3 {
        //type in external package
        let external_package = project.read().unwrap().find_package(name_vec[0].clone());
        let external_package = external_package.ok().unwrap();
        let external_package_scope = external_package.read().unwrap().get_scope();
        target_scope = external_package_scope;
        let mut name_vec_without_package_name: Vec<String> = vec![];
        name_vec_without_package_name.push(name_vec[1].clone());
        name_vec_without_package_name.push(name_vec[2].clone());
        name_vec = name_vec_without_package_name;
    }
    else {
        unreachable!()
    }

    //resolve implement
    let implement_result = target_scope.read().unwrap().resolve_implement_from_scope(name_vec[0].clone());
    if implement_result.is_err() { return Err(ExpressionEvaluationFail(String::from(implement_result.err().unwrap()))); }
    let resolved_implement = implement_result.ok().unwrap();

    //infer implement
    let implement_result;
    match arg_exps {
        None => {
            implement_result = crate::evaluation_implement::infer_implement(resolved_implement.clone(), vec![], scope.clone(), project.clone())?;
        }
        Some(arg_exps) => {
            implement_result = crate::evaluation_implement::infer_implement(resolved_implement.clone(), arg_exps, scope.clone(), project.clone())?;
        }
    }

    let implement_scope = implement_result.read().unwrap().get_scope();
    let output_var_result = implement_scope.read().unwrap().resolve_variable_in_current_scope(name_vec[1].clone());
    if output_var_result.is_err() { return Err(ExpressionEvaluationFail(String::from(output_var_result.err().unwrap()))); }
    let output_var = output_var_result.ok().unwrap();
    infer_variable(output_var.clone(), target_scope.clone(), project.clone())?;
    let output_var_cloned = (*output_var.read().unwrap()).clone();
    Ok(output_var_cloned)
}

/// support streamlets within self packages or in external packages
fn eval_exp_in_streamlet(term: Pairs<Rule>, scope: Arc<RwLock<Scope>>, project: Arc<RwLock<Project>>) -> Result<Variable, ParserErrorCode> {
    let mut name_vec: Vec<String> = vec![];
    let mut arg_exps: Option<Vec<Arc<RwLock<Variable>>>> = None;
    for term_inner in term.clone().into_iter() {
        match term_inner.clone().as_rule() {
            Rule::ID => {
                name_vec.push(term_inner.as_str().to_string());
            },
            Rule::ArgExps => {
                arg_exps = Some(crate::parse_argexps(term_inner.into_inner(), scope.clone())?);
            }
            _ => unreachable!()
        }
    }

    //get target scope
    let target_scope;
    if name_vec.len() == 2 {
        target_scope = scope.clone();
    }
    else if name_vec.len() == 3 {
        //type in external package
        let external_package = project.read().unwrap().find_package(name_vec[0].clone());
        let external_package = external_package.ok().unwrap();
        let external_package_scope = external_package.read().unwrap().get_scope();
        target_scope = external_package_scope;
        let mut name_vec_without_package_name: Vec<String> = vec![];
        name_vec_without_package_name.push(name_vec[1].clone());
        name_vec_without_package_name.push(name_vec[2].clone());
        name_vec = name_vec_without_package_name;
    }
    else {
        unreachable!()
    }

    //resolve implement
    let streamlet_result = target_scope.read().unwrap().resolve_streamlet_from_scope(name_vec[0].clone());
    if streamlet_result.is_err() { return Err(ExpressionEvaluationFail(String::from(streamlet_result.err().unwrap()))); }
    let resolved_streamlet = streamlet_result.ok().unwrap();

    //infer streamlet
    let streamlet_result;
    match arg_exps {
        None => {
            streamlet_result = crate::evaluation_streamlet::infer_streamlet(resolved_streamlet.clone(), vec![], scope.clone(), project.clone())?;
        }
        Some(arg_exps) => {
            streamlet_result = crate::evaluation_streamlet::infer_streamlet(resolved_streamlet.clone(), arg_exps, scope.clone(), project.clone())?;
        }
    }

    let streamlet_scope = streamlet_result.read().unwrap().get_scope();
    let output_var_result = streamlet_scope.read().unwrap().resolve_variable_in_current_scope(name_vec[1].clone());
    if output_var_result.is_err() { return Err(ExpressionEvaluationFail(String::from(output_var_result.err().unwrap()))); }
    let output_var = output_var_result.ok().unwrap();
    infer_variable(output_var.clone(), target_scope.clone(), project.clone())?;
    let output_var_cloned = (*output_var.read().unwrap()).clone();
    Ok(output_var_cloned)
}

pub fn eval_term(term: Pairs<Rule>, scope: Arc<RwLock<Scope>>, project: Arc<RwLock<Project>>) -> Result<Variable, ParserErrorCode> {
    for term_inner in term.clone().into_iter() {
        match term_inner.clone().as_rule() {
            Rule::Exp => {
                return eval_exp(term_inner.into_inner(), scope.clone(), project.clone());
            }

            Rule::ExpLog => {
                return eval_explog(term_inner.into_inner(), scope.clone(), project.clone());
            }
            Rule::ExpBitWiseNot => {
                return eval_exp_bitwisenot(term_inner.into_inner(), scope.clone(), project.clone());
            }
            Rule::ExpConstInType | Rule::ExpConstInExternType => {
                return eval_exp_in_type(term_inner.into_inner(), scope.clone(), project.clone());
            }
            Rule::ExpRound => {
                let mut input_exp = Variable::new_int(String::from(""), 0);
                for exp in term_inner.into_inner().into_iter() {
                    match exp.as_rule() {
                        Rule::Exp => {
                            let result = eval_exp(exp.into_inner(), scope.clone(), project.clone());
                            if result.is_err() { return Err(result.err().unwrap()); }
                            input_exp = result.ok().unwrap();
                        }
                        _ => unreachable!()
                    }
                }
                let output_value: i32;
                match input_exp.get_var_value().get_raw_value() {
                    VariableValue::Int(v) => {
                        output_value = v;
                    }
                    VariableValue::Float(v) => {
                        output_value = v.round() as i32;
                    }
                    _ => { return Err(ExpressionEvaluationFail(format!("Round only acceptes float and int"))); }
                }
                return Ok(Variable::new_int(String::from(""), output_value));
            }
            Rule::ExpFloor => {
                let mut input_exp = Variable::new_int(String::from(""), 0);
                for exp in term_inner.into_inner().into_iter() {
                    match exp.as_rule() {
                        Rule::Exp => {
                            let result = eval_exp(exp.into_inner(), scope.clone(), project.clone());
                            if result.is_err() { return Err(result.err().unwrap()); }
                            input_exp = result.ok().unwrap();
                        }
                        _ => unreachable!()
                    }
                }
                let output_value: i32;
                match input_exp.get_var_value().get_raw_value() {
                    VariableValue::Int(v) => {
                        output_value = v;
                    }
                    VariableValue::Float(v) => {
                        output_value = v.floor() as i32;
                    }
                    _ => { return Err(ExpressionEvaluationFail(format!("Round only acceptes float and int"))); }
                }
                return Ok(Variable::new_int(String::from(""), output_value));
            }
            Rule::ExpCeil => {
                let mut input_exp = Variable::new_int(String::from(""), 0);
                for exp in term_inner.into_inner().into_iter() {
                    match exp.as_rule() {
                        Rule::Exp => {
                            let result = eval_exp(exp.into_inner(), scope.clone(), project.clone());
                            if result.is_err() { return Err(result.err().unwrap()); }
                            input_exp = result.ok().unwrap();
                        }
                        _ => unreachable!()
                    }
                }
                let output_value: i32;
                match input_exp.get_var_value().get_raw_value() {
                    VariableValue::Int(v) => {
                        output_value = v;
                    }
                    VariableValue::Float(v) => {
                        output_value = v.ceil() as i32;
                    }
                    _ => { return Err(ExpressionEvaluationFail(format!("Round only acceptes float and int"))); }
                }
                return Ok(Variable::new_int(String::from(""), output_value));
            }
            Rule::ExpIndex => {
                let mut input_exp = Variable::new_int(String::from(""), 0);
                let mut input_index = Variable::new_int(String::from(""), 0);
                for exp in term_inner.into_inner().into_iter() {
                    match exp.as_rule() {
                        Rule::ID => {
                            let var_name = exp.as_str().to_string();
                            let result = scope.read().unwrap().resolve_variable_from_scope(var_name);
                            if result.is_err() { return Err(ExpressionEvaluationFail(String::from(result.err().unwrap()))); }
                            let result = result.ok().unwrap();
                            let infer_result = infer_variable(result.clone(), scope.clone(), project.clone());
                            if infer_result.is_err() { return Err(infer_result.err().unwrap()); }
                            input_exp = (*result.read().unwrap()).clone();
                        }
                        Rule::Exp => {
                            let result = eval_exp(exp.into_inner(), scope.clone(), project.clone());
                            if result.is_err() { return Err(result.err().unwrap()); }
                            input_index = result.ok().unwrap();
                        }
                        _ => unreachable!()
                    }
                }

                let index = match input_index.get_var_value().get_raw_value() {
                    VariableValue::Int(i) => {
                        if i < 0 { return Err(ExpressionEvaluationFail(format!("the index of [] operator must be positive integer"))) }
                        i as usize
                    },
                    _ => return Err(ExpressionEvaluationFail(format!("the index of [] operator must be positive integer")))
                };

                match input_exp.get_var_value().get_raw_value() {
                    VariableValue::ArrayStr(array) => {
                        if index >= array.len() { return Err(ExpressionEvaluationFail(format!("index({}) out of array length({})", index, array.len()))); }
                        return Ok(Variable::new_str(String::from(""), array.get(index).unwrap().clone()));
                    }
                    VariableValue::ArrayInt(array) => {
                        if index >= array.len() { return Err(ExpressionEvaluationFail(format!("index({}) out of array length({})", index, array.len()))); }
                        return Ok(Variable::new_int(String::from(""), array.get(index).unwrap().clone()));
                    }
                    VariableValue::ArrayFloat(array) => {
                        if index >= array.len() { return Err(ExpressionEvaluationFail(format!("index({}) out of array length({})", index, array.len()))); }
                        return Ok(Variable::new_float(String::from(""), array.get(index).unwrap().clone()));
                    }
                    VariableValue::ArrayBool(array) => {
                        if index >= array.len() { return Err(ExpressionEvaluationFail(format!("index({}) out of array length({})", index, array.len()))); }
                        return Ok(Variable::new_bool(String::from(""), array.get(index).unwrap().clone()));
                    }
                    _ => return Err(ExpressionEvaluationFail(format!("[] operator only accepts array")))
                }
            }
            Rule::ExpConstInStreamlet | Rule::ExpExternalConstInStreamlet  => {
                return eval_exp_in_streamlet(term_inner.into_inner(), scope.clone(), project.clone());
            }
            Rule::ExpConstInImplement | Rule::ExpExternalConstInImplement => {
                return eval_exp_in_implement(term_inner.into_inner(), scope.clone(), project.clone());
            }
            Rule::UnaryExp => {
                return eval_exp_unary(term_inner.into_inner(), scope.clone(), project.clone());
            }

            Rule::Var => {
                let var_name = term_inner.as_str().to_string();
                let result = scope.read().unwrap().resolve_variable_from_scope(var_name.clone());
                if result.is_err() { return Err(ExpressionEvaluationFail(String::from(result.err().unwrap()))); }
                let target_var = result.ok().unwrap();
                let result = infer_variable(target_var.clone(), scope.clone(), project.clone());
                if result.is_err() { return Err(result.err().unwrap()); }
                let output = (*target_var.read().unwrap()).clone();
                return Ok(output);
            }
            Rule::ExternalVar => {
                let mut package_id = String::from("");
                let mut var_id = String::from("");
                let mut is_package = true;
                for id in term_inner.clone().into_inner().into_iter() {
                    match id.as_rule() {
                        Rule::ID =>{
                            if is_package {
                                package_id = id.as_str().to_string();
                                is_package = false;
                            }
                            else {
                                var_id = id.as_str().to_string();
                            }
                        }
                        _ => unreachable!()
                    }
                }

                //check import
                {
                    let package_var_result = scope.read().unwrap().resolve_variable_from_scope(format!("$package${}", package_id.clone()));
                    if package_var_result.is_err() { return Err(ExpressionEvaluationFail(format!("package {} not imported", package_id.clone()))); }
                }

                let project_unwrap = project.read().unwrap();
                let package = project_unwrap.packages.get(&package_id);
                if package.is_none() { return Err(ExpressionEvaluationFail(format!("package {} doesn't exist", package_id.clone()))); }
                let package = package.unwrap().clone();
                let package_scope = package.read().unwrap().scope.clone();
                let var = package_scope.read().unwrap().resolve_variable_from_scope(var_id.clone());
                if var.is_err() { return Err(ExpressionEvaluationFail(String::from(var.err().unwrap()))); }
                let var = var.ok().unwrap();
                let result = infer_variable(var.clone(), package_scope.clone(), project.clone());
                if result.is_err() { return Err(result.err().unwrap()); }
                let output = (*var.read().unwrap()).clone();
                return Ok(output);
            }

            Rule::IntExp => {
                return eval_exp_int(term_inner.into_inner(), scope.clone(), project.clone());
            }
            Rule::FloatExp => {
                let result = term_inner.clone().as_str().parse::<f32>();
                if result.is_err() { return Err(ExpressionEvaluationFail(format!("cannot parse {} as float", term_inner.clone().to_string()))); }
                return Ok(Variable::new_float(String::from(""), result.ok().unwrap()));
            }
            Rule::StringExp => {
                let output = term_inner.as_str().to_string();
                let output = output[1 .. output.len()-1].to_string();
                return Ok(Variable::new_str(String::from(""), output));
            }
            Rule::BoolExp => {
                let bool_exp = term_inner.as_str().to_string().to_lowercase();
                if bool_exp == "true" {
                    return Ok(Variable::new_bool(String::from(""), true));
                }
                else if bool_exp == "false" {
                    return Ok(Variable::new_bool(String::from(""), false));
                }
                else {
                    unreachable!()
                }
            }
            Rule::ClockDomainExp => {
                let mut clockdomain_exp = term_inner.as_str().to_string();
                if clockdomain_exp.ends_with("GHz") {
                    clockdomain_exp = clockdomain_exp.replace("GHz", "");
                    let freq = clockdomain_exp.parse::<f32>().expect("not a valid clockdomain");
                    return Ok(Variable::new_with_value(String::from(""), DataType::ClockDomainType, VariableValue::ClockDomain(ClockDomainValue::GHz(freq))));
                }
                else if clockdomain_exp.ends_with("MHz") {
                    clockdomain_exp = clockdomain_exp.replace("MHz", "");
                    let freq = clockdomain_exp.parse::<f32>().expect("not a valid clockdomain");
                    return Ok(Variable::new_with_value(String::from(""), DataType::ClockDomainType, VariableValue::ClockDomain(ClockDomainValue::MHz(freq))));
                }
                else if clockdomain_exp.ends_with("kHz") {
                    clockdomain_exp = clockdomain_exp.replace("kHz", "");
                    let freq = clockdomain_exp.parse::<f32>().expect("not a valid clockdomain");
                    return Ok(Variable::new_with_value(String::from(""), DataType::ClockDomainType, VariableValue::ClockDomain(ClockDomainValue::kHz(freq))));
                }
                else if clockdomain_exp.ends_with("Hz") {
                    clockdomain_exp = clockdomain_exp.replace("Hz", "");
                    let freq = clockdomain_exp.parse::<f32>().expect("not a valid clockdomain");
                    return Ok(Variable::new_with_value(String::from(""), DataType::ClockDomainType, VariableValue::ClockDomain(ClockDomainValue::Hz(freq))));
                }
                else {
                    unreachable!()
                }
            }

            Rule::ArrayRange => {
                let mut exp_vars: Vec<Variable> = vec![];
                for single_exp in term_inner.into_inner().into_iter() {
                    match single_exp.as_rule() {
                        Rule::Exp => {
                            let result = eval_exp(single_exp.into_inner(), scope.clone(), project.clone());
                            if result.is_err() { return Err(result.err().unwrap()); }
                            exp_vars.push(result.ok().unwrap());
                        }
                        _ => unreachable!()
                    }
                }
                let mut range_values: Vec<i32> = vec![];
                for index in 0 .. exp_vars.len() {
                    match exp_vars[index].get_var_value().get_raw_value() {
                        VariableValue::Int(v) => {
                            range_values.push(v);
                        }
                        _ => { return Err(ExpressionEvaluationFail(format!("ArrayRange expression can only use int values as start value")));}
                    }
                }
                let mut output_exp: Vec<i32> = vec![];
                let mut current_v = range_values[0];
                while current_v < range_values[2] {
                    output_exp.push(current_v);
                    current_v = current_v + range_values[1];
                }
                return Ok(Variable::new_int_array(String::from(""), output_exp));
            }
            Rule::ArrayExp => {
                let mut exps: Vec<Variable> = vec![];
                for single_exp in term_inner.into_inner().into_iter() {
                    match single_exp.as_rule() {
                        Rule::Exp => {
                            let result = eval_exp(single_exp.into_inner(), scope.clone(), project.clone());
                            if result.is_err() { return Err(result.err().unwrap()); }
                            exps.push(result.ok().unwrap());
                        }
                        _ => unreachable!()
                    }
                }

                //check type
                let element_type = (*exps[0].get_type().read().unwrap()).clone();
                match element_type {
                    DataType::IntType | DataType::StringType | DataType::BoolType | DataType::FloatType => {}
                    _ => { return Err(ExpressionEvaluationFail(format!("array expression only supports int/bool/float/string"))); }
                }
                for index in 1 .. exps.len() {
                    if (*exps[index].get_type().read().unwrap()).clone() != element_type {
                        return Err(ExpressionEvaluationFail(format!("array elements must have the same types")));
                    }
                }

                match element_type {
                    DataType::IntType => {
                        let mut output: Vec<i32> = vec![];
                        for exp in exps {
                            match exp.get_var_value().get_raw_value() {
                                VariableValue::Int(v) => {
                                    output.push(v);
                                }
                                _ => unreachable!()
                            }
                        }
                        return Ok(Variable::new_int_array(String::from(""), output));
                    }
                    DataType::BoolType => {
                        let mut output: Vec<bool> = vec![];
                        for exp in exps {
                            match exp.get_var_value().get_raw_value() {
                                VariableValue::Bool(v) => {
                                    output.push(v);
                                }
                                _ => unreachable!()
                            }
                        }
                        return Ok(Variable::new_bool_array(String::from(""), output));
                    }
                    DataType::StringType => {
                        let mut output: Vec<String> = vec![];
                        for exp in exps {
                            match exp.get_var_value().get_raw_value() {
                                VariableValue::Str(v) => {
                                    output.push(v);
                                }
                                _ => unreachable!()
                            }
                        }
                        return Ok(Variable::new_str_array(String::from(""), output));
                    }
                    DataType::FloatType => {
                        let mut output: Vec<f32> = vec![];
                        for exp in exps {
                            match exp.get_var_value().get_raw_value() {
                                VariableValue::Float(v) => {
                                    output.push(v);
                                }
                                _ => unreachable!()
                            }
                        }
                        return Ok(Variable::new_float_array(String::from(""), output));
                    }
                    _ => unreachable!()
                }
            }

            _ => { unreachable!() },
        }
    }
    unreachable!()
}

pub fn parse_eval_exp(expression: String, scope: Arc<RwLock<Scope>>, project: Arc<RwLock<Project>>) -> Result<Variable, ParserErrorCode> {
    if expression.is_empty() { unreachable!() }
    let parse_result = TydiParser::parse(Rule::Exp, &expression);
    if parse_result.is_err() { return Err(ExpressionEvaluationFail(format!("{}", parse_result.err().unwrap()))); }
    let parse_result = parse_result.ok().unwrap().next().unwrap();
    return eval_exp(parse_result.into_inner(), scope.clone(), project.clone());
}

pub fn eval_exp(expression: Pairs<Rule>, scope: Arc<RwLock<Scope>>, project: Arc<RwLock<Project>>) -> Result<Variable, ParserErrorCode> {
    PREC_CLIMBER.climb(expression,
        |pair: Pair<Rule>| {
            match pair.as_rule() {
                Rule::Term => {
                    return eval_term(pair.into_inner(), scope.clone(), project.clone());
                },
                Rule::Exp => {
                    return eval_exp(pair.into_inner(), scope.clone(), project.clone());
                },
                _ => { unreachable!() },
            }
        },
        |lhs_raw: Result<Variable, ParserErrorCode>, op: Pair<Rule>, rhs_raw: Result<Variable, ParserErrorCode>| {
            if lhs_raw.is_err() { return Err(lhs_raw.err().unwrap()); }
            if rhs_raw.is_err() { return Err(rhs_raw.err().unwrap()); }
            let mut lhs = lhs_raw.clone().unwrap();
            let mut rhs = rhs_raw.clone().unwrap();

            // infer lhs value
            if lhs.get_var_value().get_infer_state() == InferState::NotInferred {
                let result = parse_eval_exp(lhs.get_var_value().get_raw_exp(), scope.clone(), project.clone());
                if result.is_err() { return Err(result.err().unwrap()); }
                let new_val = result.ok().unwrap();
                lhs.set_var_value(new_val.get_var_value());
            }

            // infer rhs value
            if rhs.get_var_value().get_infer_state() == InferState::NotInferred {
                let result = parse_eval_exp(rhs.get_var_value().get_raw_exp(), scope.clone(), project.clone());
                if result.is_err() { return Err(result.err().unwrap()); }
                let new_val = result.ok().unwrap();
                rhs.set_var_value(new_val.get_var_value());
            }

            let lhs_type = (*lhs.get_type().read().unwrap()).clone();
            let rhs_type = (*rhs.get_type().read().unwrap()).clone();
            match op.as_rule() {
                Rule::OP_LogicalEq => {
                    let result = eval_op_equal(lhs.clone(), rhs.clone(), lhs_type.clone(), rhs_type.clone());
                    if result.is_err() { return Err(result.err().unwrap()); }
                    return Ok(Variable::new_bool(String::from(""), result.ok().unwrap()));
                },
                Rule::OP_LogicalNotEq => {
                    let result = eval_op_equal(lhs.clone(), rhs.clone(), lhs_type.clone(), rhs_type.clone());
                    if result.is_err() { return Err(result.err().unwrap()); }
                    return Ok(Variable::new_bool(String::from(""), !result.ok().unwrap()));
                },

                Rule::OP_Greater => {
                    let result = eval_op_greater(lhs.clone(), rhs.clone(), lhs_type.clone(), rhs_type.clone());
                    if result.is_err() { return Err(result.err().unwrap()); }
                    return Ok(Variable::new_bool(String::from(""), result.ok().unwrap()));
                },
                Rule::OP_GreaterEq => {
                    let result = eval_op_less(lhs.clone(), rhs.clone(), lhs_type.clone(), rhs_type.clone());
                    if result.is_err() { return Err(result.err().unwrap()); }
                    return Ok(Variable::new_bool(String::from(""), !result.ok().unwrap()));
                },
                Rule::OP_Less => {
                    let result = eval_op_less(lhs.clone(), rhs.clone(), lhs_type.clone(), rhs_type.clone());
                    if result.is_err() { return Err(result.err().unwrap()); }
                    return Ok(Variable::new_bool(String::from(""), result.ok().unwrap()));
                },
                Rule::OP_LessEq => {
                    let result = eval_op_greater(lhs.clone(), rhs.clone(), lhs_type.clone(), rhs_type.clone());
                    if result.is_err() { return Err(result.err().unwrap()); }
                    return Ok(Variable::new_bool(String::from(""), !result.ok().unwrap()));
                },

                Rule::OP_LeftShift => {
                    if lhs_type == DataType::IntType && rhs_type == DataType::IntType {
                        let lhs_value = lhs.get_var_value().get_raw_value();
                        let rhs_value = rhs.get_var_value().get_raw_value();
                        let lhs;
                        let rhs;
                        match lhs_value {
                            VariableValue::Int(v) => { lhs = v; }
                            _ => unreachable!()
                        }
                        match rhs_value {
                            VariableValue::Int(v) => { rhs = v; }
                            _ => unreachable!()
                        }
                        if lhs < 0 { return Err(ExpressionEvaluationFail(format!("left shift negative values: {}", lhs))); }
                        return Ok(Variable::new_int(String::from(""), lhs << rhs));
                    }
                    else {
                        return Err(ExpressionEvaluationFail(format!("<< operator does not support {} + {}", String::from(lhs_type), String::from(rhs_type))));
                    }
                },
                Rule::OP_RightShift => {
                    if lhs_type == DataType::IntType && rhs_type == DataType::IntType {
                        let lhs_value = lhs.get_var_value().get_raw_value();
                        let rhs_value = rhs.get_var_value().get_raw_value();
                        let lhs;
                        let rhs;
                        match lhs_value {
                            VariableValue::Int(v) => { lhs = v; }
                            _ => unreachable!()
                        }
                        match rhs_value {
                            VariableValue::Int(v) => { rhs = v; }
                            _ => unreachable!()
                        }
                        if lhs < 0 { return Err(ExpressionEvaluationFail(format!("left shift negative values: {}", lhs))); }
                        return Ok(Variable::new_int(String::from(""), lhs >> rhs));
                    }
                    else {
                        return Err(ExpressionEvaluationFail(format!(">> operator does not support {} + {}", String::from(lhs_type), String::from(rhs_type))));
                    }
                },
                Rule::OP_Add => {
                    if lhs_type == DataType::IntType && rhs_type == DataType::IntType {
                        let lhs_value = lhs.get_var_value().get_raw_value();
                        let rhs_value = rhs.get_var_value().get_raw_value();
                        let output:i32;
                        match lhs_value {
                            VariableValue::Int(i0) => {
                                match rhs_value {
                                    VariableValue::Int(i1) => {
                                        output = i0 + i1;
                                    }
                                    _ => unreachable!()
                                }
                            }
                            _ => unreachable!()
                        }
                        return Ok(Variable::new_int(String::from(""), output));
                    }
                    else if (lhs_type == DataType::FloatType && rhs_type == DataType::FloatType) ||
                        (lhs_type == DataType::FloatType && rhs_type == DataType::IntType) ||
                        (lhs_type == DataType::IntType && rhs_type == DataType::FloatType) {
                        let lhs_value = lhs.get_var_value().get_raw_value();
                        let rhs_value = rhs.get_var_value().get_raw_value();
                        let output:f32;
                        let lhs_f32:f32;
                        let rhs_f32:f32;
                        match lhs_value {
                            VariableValue::Float(v) => { lhs_f32 = v; }
                            VariableValue::Int(v) => { lhs_f32 = v as f32; }
                            _ => unreachable!()
                        }
                        match rhs_value {
                            VariableValue::Float(v) => { rhs_f32 = v; }
                            VariableValue::Int(v) => { rhs_f32 = v as f32; }
                            _ => unreachable!()
                        }
                        output = lhs_f32 + rhs_f32;
                        return Ok(Variable::new_float(String::from(""), output));
                    }
                    else if (lhs_type == DataType::StringType) || (rhs_type == DataType::StringType) {
                        let lhs_value = lhs.get_var_value().get_raw_value();
                        let rhs_value = rhs.get_var_value().get_raw_value();
                        let lhs_str:String;
                        let rhs_str:String;
                        let mut output = String::from("");
                        match lhs_value {
                            VariableValue::Float(v) => { lhs_str = v.to_string(); }
                            VariableValue::Int(v) => { lhs_str = v.to_string(); }
                            VariableValue::Str(v) => { lhs_str = String::from(v); }
                            VariableValue::Bool(v) => { lhs_str = v.to_string(); }
                            VariableValue::ClockDomain(v) => { lhs_str = String::from(v); }
                            _ => return Err(ExpressionEvaluationFail(format!("+ operator only supports append string with int/float/bool/string")))
                        }
                        match rhs_value {
                            VariableValue::Float(v) => { rhs_str = v.to_string(); }
                            VariableValue::Int(v) => { rhs_str = v.to_string(); }
                            VariableValue::Str(v) => { rhs_str = String::from(v); }
                            VariableValue::Bool(v) => { rhs_str = v.to_string(); }
                            VariableValue::ClockDomain(v) => { rhs_str = String::from(v); }
                            _ => return Err(ExpressionEvaluationFail(format!("+ operator only supports append string with int/float/bool/string")))
                        }
                        output.push_str(&lhs_str);
                        output.push_str(&rhs_str);
                        return Ok(Variable::new_str(String::from(""), output));
                    }
                    else {
                        //array
                        match lhs_type.clone() {
                            DataType::ArrayType(inner_type) => {
                                let inner_type = (*inner_type.read().unwrap()).clone();
                                if rhs_type == inner_type || (rhs_type == DataType::IntType && inner_type == DataType::FloatType) {
                                    let variable_value_infer = lhs.get_var_value();
                                    let variable_value = variable_value_infer.get_raw_value();
                                    let output_variable;
                                    match variable_value.clone() {
                                        VariableValue::ArrayBool(var) => {
                                            let mut var = var.clone();
                                            match rhs.get_var_value().get_raw_value() {
                                                VariableValue::Bool(new_v) => var.push(new_v),
                                                _ => return Err(ExpressionEvaluationFail(format!("+ operator does not support [{}] + {}", String::from(lhs_type), String::from(rhs_type))))
                                            }
                                            output_variable = Variable::new_bool_array(String::from(""), var);
                                        }
                                        VariableValue::ArrayFloat(var) => {
                                            let mut var = var.clone();
                                            match rhs.get_var_value().get_raw_value() {
                                                VariableValue::Float(new_v) => var.push(new_v),
                                                VariableValue::Int(new_v) => var.push(new_v as f32),
                                                _ => return Err(ExpressionEvaluationFail(format!("+ operator does not support [{}] + {}", String::from(lhs_type), String::from(rhs_type))))
                                            }
                                            output_variable = Variable::new_float_array(String::from(""), var);
                                        }
                                        VariableValue::ArrayInt(var) => {
                                            let mut var = var.clone();
                                            match rhs.get_var_value().get_raw_value() {
                                                VariableValue::Int(new_v) => var.push(new_v),
                                                _ => return Err(ExpressionEvaluationFail(format!("+ operator does not support [{}] + {}", String::from(lhs_type), String::from(rhs_type))))
                                            }
                                            output_variable = Variable::new_int_array(String::from(""), var);
                                        }
                                        VariableValue::ArrayStr(var) => {
                                            let mut var = var.clone();
                                            match rhs.get_var_value().get_raw_value() {
                                                VariableValue::Str(new_v) => var.push(new_v),
                                                _ => return Err(ExpressionEvaluationFail(format!("+ operator does not support [{}] + {}", String::from(lhs_type), String::from(rhs_type))))
                                            }
                                            output_variable = Variable::new_str_array(String::from(""), var);
                                        }
                                        _ => unreachable!()
                                    }
                                    return Ok(output_variable);
                                }
                            }
                            _ => {}
                        }
                        match rhs_type.clone() {
                            DataType::ArrayType(inner_type) => {
                                let inner_type = (*inner_type.read().unwrap()).clone();
                                if lhs_type == inner_type || (lhs_type == DataType::IntType && inner_type == DataType::FloatType) {
                                    let variable_value_infer = rhs.get_var_value();
                                    let variable_value = variable_value_infer.get_raw_value();
                                    let output_variable;
                                    match variable_value.clone() {
                                        VariableValue::ArrayBool(var) => {
                                            let mut var = var.clone();
                                            match lhs.get_var_value().get_raw_value() {
                                                VariableValue::Bool(new_v) => var.insert(0,new_v),
                                                _ => return Err(ExpressionEvaluationFail(format!("+ operator does not support {} + [{}]", String::from(lhs_type), String::from(rhs_type))))
                                            }
                                            output_variable = Variable::new_bool_array(String::from(""), var);
                                        }
                                        VariableValue::ArrayFloat(var) => {
                                            let mut var = var.clone();
                                            match lhs.get_var_value().get_raw_value() {
                                                VariableValue::Float(new_v) => var.insert(0,new_v),
                                                VariableValue::Int(new_v) => var.insert(0,new_v as f32),
                                                _ => return Err(ExpressionEvaluationFail(format!("+ operator does not support {} + [{}]", String::from(lhs_type), String::from(rhs_type))))
                                            }
                                            output_variable = Variable::new_float_array(String::from(""), var);
                                        }
                                        VariableValue::ArrayInt(var) => {
                                            let mut var = var.clone();
                                            match lhs.get_var_value().get_raw_value() {
                                                VariableValue::Int(new_v) => var.insert(0,new_v),
                                                _ => return Err(ExpressionEvaluationFail(format!("+ operator does not support {} + [{}]", String::from(lhs_type), String::from(rhs_type))))
                                            }
                                            output_variable = Variable::new_int_array(String::from(""), var);
                                        }
                                        VariableValue::ArrayStr(var) => {
                                            let mut var = var.clone();
                                            match lhs.get_var_value().get_raw_value() {
                                                VariableValue::Str(new_v) => var.insert(0,new_v),
                                                _ => return Err(ExpressionEvaluationFail(format!("+ operator does not support {} + [{}]", String::from(lhs_type), String::from(rhs_type))))
                                            }
                                            output_variable = Variable::new_str_array(String::from(""), var);
                                        }
                                        _ => unreachable!()
                                    }
                                    return Ok(output_variable);
                                }
                            }
                            _ => {}
                        }

                        return Err(ExpressionEvaluationFail(format!("+ operator does not support {} + {}", String::from(lhs_type), String::from(rhs_type))));
                    }
                },
                Rule::OP_Minus => {
                    if lhs_type == DataType::IntType && rhs_type == DataType::IntType {
                        let lhs_value = lhs.get_var_value().get_raw_value();
                        let rhs_value = rhs.get_var_value().get_raw_value();
                        let output:i32;
                        match lhs_value {
                            VariableValue::Int(i0) => {
                                match rhs_value {
                                    VariableValue::Int(i1) => {
                                        output = i0 - i1;
                                    }
                                    _ => unreachable!()
                                }
                            }
                            _ => unreachable!()
                        }
                        return Ok(Variable::new_int(String::from(""), output));
                    }
                    else if (lhs_type == DataType::FloatType && rhs_type == DataType::FloatType) ||
                        (lhs_type == DataType::FloatType && rhs_type == DataType::IntType) ||
                        (lhs_type == DataType::IntType && rhs_type == DataType::FloatType) {
                        let lhs_value = lhs.get_var_value().get_raw_value();
                        let rhs_value = rhs.get_var_value().get_raw_value();
                        let output:f32;
                        let lhs_f32:f32;
                        let rhs_f32:f32;
                        match lhs_value {
                            VariableValue::Float(v) => { lhs_f32 = v; }
                            VariableValue::Int(v) => { lhs_f32 = v as f32; }
                            _ => unreachable!()
                        }
                        match rhs_value {
                            VariableValue::Float(v) => { rhs_f32 = v; }
                            VariableValue::Int(v) => { rhs_f32 = v as f32; }
                            _ => unreachable!()
                        }
                        output = lhs_f32 - rhs_f32;
                        return Ok(Variable::new_float(String::from(""), output));
                    }
                    else {
                        return Err(ExpressionEvaluationFail(format!("- operator does not support {} + {}", String::from(lhs_type), String::from(rhs_type))));
                    }
                },
                Rule::OP_BitAnd => {
                    if lhs_type == DataType::IntType && rhs_type == DataType::IntType {
                        let lhs_value = lhs.get_var_value().get_raw_value();
                        let rhs_value = rhs.get_var_value().get_raw_value();
                        let lhs;
                        let rhs;
                        match lhs_value {
                            VariableValue::Int(v) => { lhs = v; }
                            _ => unreachable!()
                        }
                        match rhs_value {
                            VariableValue::Int(v) => { rhs = v; }
                            _ => unreachable!()
                        }
                        return Ok(Variable::new_int(String::from(""), lhs & rhs));
                    }
                    else {
                        return Err(ExpressionEvaluationFail(format!("& operator does not support {} + {}", String::from(lhs_type), String::from(rhs_type))));
                    }
                },
                Rule::OP_BitOr => {
                    if lhs_type == DataType::IntType && rhs_type == DataType::IntType {
                        let lhs_value = lhs.get_var_value().get_raw_value();
                        let rhs_value = rhs.get_var_value().get_raw_value();
                        let lhs;
                        let rhs;
                        match lhs_value {
                            VariableValue::Int(v) => { lhs = v; }
                            _ => unreachable!()
                        }
                        match rhs_value {
                            VariableValue::Int(v) => { rhs = v; }
                            _ => unreachable!()
                        }
                        return Ok(Variable::new_int(String::from(""), lhs | rhs));
                    }
                    else {
                        return Err(ExpressionEvaluationFail(format!("| operator does not support {} + {}", String::from(lhs_type), String::from(rhs_type))));
                    }
                },
                Rule::OP_LogicalAnd => {
                    if lhs_type == DataType::BoolType && rhs_type == DataType::BoolType {
                        let lhs_value = lhs.get_var_value().get_raw_value();
                        let rhs_value = rhs.get_var_value().get_raw_value();
                        let lhs;
                        let rhs;
                        match lhs_value {
                            VariableValue::Bool(v) => { lhs = v; }
                            _ => unreachable!()
                        }
                        match rhs_value {
                            VariableValue::Bool(v) => { rhs = v; }
                            _ => unreachable!()
                        }
                        return Ok(Variable::new_bool(String::from(""), lhs && rhs));
                    }
                    else {
                        return Err(ExpressionEvaluationFail(format!("&& operator does not support {} + {}", String::from(lhs_type), String::from(rhs_type))));
                    }
                },
                Rule::OP_LogicalOr => {
                    if lhs_type == DataType::BoolType && rhs_type == DataType::BoolType {
                        let lhs_value = lhs.get_var_value().get_raw_value();
                        let rhs_value = rhs.get_var_value().get_raw_value();
                        let lhs;
                        let rhs;
                        match lhs_value {
                            VariableValue::Bool(v) => { lhs = v; }
                            _ => unreachable!()
                        }
                        match rhs_value {
                            VariableValue::Bool(v) => { rhs = v; }
                            _ => unreachable!()
                        }
                        return Ok(Variable::new_bool(String::from(""), lhs || rhs));
                    }
                    else {
                        return Err(ExpressionEvaluationFail(format!("&& operator does not support {} + {}", String::from(lhs_type), String::from(rhs_type))));
                    }
                },

                Rule::OP_Multiply => {
                    if lhs_type == DataType::IntType && rhs_type == DataType::IntType {
                        let lhs_value = lhs.get_var_value().get_raw_value();
                        let rhs_value = rhs.get_var_value().get_raw_value();
                        let lhs_int: i32;
                        let rhs_int: i32;
                        match lhs_value {
                            VariableValue::Int(i0) => { lhs_int = i0; }
                            _ => unreachable!()
                        }
                        match rhs_value {
                            VariableValue::Int(i1) => { rhs_int = i1; }
                            _ => unreachable!()
                        }
                        return Ok(Variable::new_int(String::from(""), lhs_int * rhs_int));
                    }
                    else if (lhs_type == DataType::FloatType && rhs_type == DataType::FloatType) ||
                        (lhs_type == DataType::FloatType && rhs_type == DataType::IntType) ||
                        (lhs_type == DataType::IntType && rhs_type == DataType::FloatType) {
                        let lhs_value = lhs.get_var_value().get_raw_value();
                        let rhs_value = rhs.get_var_value().get_raw_value();
                        let lhs_float:f32;
                        let rhs_float:f32;
                        match lhs_value {
                            VariableValue::Int(v) => {
                                lhs_float = v as f32;
                            }
                            VariableValue::Float(v) => {
                                lhs_float = v;
                            }
                            _ => unreachable!()
                        }
                        match rhs_value {
                            VariableValue::Int(v) => {
                                rhs_float = v as f32;
                            }
                            VariableValue::Float(v) => {
                                rhs_float = v;
                            }
                            _ => unreachable!()
                        }
                        return Ok(Variable::new_float(String::from(""), lhs_float * rhs_float));
                    }
                    else {
                        return Err(ExpressionEvaluationFail(format!("* operator does not support {} + {}", String::from(lhs_type), String::from(rhs_type))));
                    }
                },
                Rule::OP_Divide => {
                    if lhs_type == DataType::IntType && rhs_type == DataType::IntType {
                        let lhs_value = lhs.get_var_value().get_raw_value();
                        let rhs_value = rhs.get_var_value().get_raw_value();
                        let lhs_int: i32;
                        let rhs_int: i32;
                        match lhs_value {
                            VariableValue::Int(i0) => { lhs_int = i0; }
                            _ => unreachable!()
                        }
                        match rhs_value {
                            VariableValue::Int(i1) => { rhs_int = i1; }
                            _ => unreachable!()
                        }
                        if rhs_int == 0 { return Err(ExpressionEvaluationFail(format!("try to divide 0"))); }
                        return Ok(Variable::new_int(String::from(""), lhs_int / rhs_int));
                    }
                    else if (lhs_type == DataType::FloatType && rhs_type == DataType::FloatType) ||
                        (lhs_type == DataType::FloatType && rhs_type == DataType::IntType) ||
                        (lhs_type == DataType::IntType && rhs_type == DataType::FloatType) {
                        let lhs_value = lhs.get_var_value().get_raw_value();
                        let rhs_value = rhs.get_var_value().get_raw_value();
                        let lhs_float:f32;
                        let rhs_float:f32;
                        match lhs_value {
                            VariableValue::Int(v) => {
                                lhs_float = v as f32;
                            }
                            VariableValue::Float(v) => {
                                lhs_float = v;
                            }
                            _ => unreachable!()
                        }
                        match rhs_value {
                            VariableValue::Int(v) => {
                                rhs_float = v as f32;
                            }
                            VariableValue::Float(v) => {
                                rhs_float = v;
                            }
                            _ => unreachable!()
                        }
                        if rhs_float == 0.0 { return Err(ExpressionEvaluationFail(format!("try to divide 0"))); }
                        return Ok(Variable::new_float(String::from(""), lhs_float / rhs_float));
                    }
                    else {
                        return Err(ExpressionEvaluationFail(format!("/ operator does not support {} + {}", String::from(lhs_type), String::from(rhs_type))));
                    }
                },
                Rule::OP_Mod => {
                    if lhs_type == DataType::IntType && rhs_type == DataType::IntType {
                        let lhs_value = lhs.get_var_value().get_raw_value();
                        let rhs_value = rhs.get_var_value().get_raw_value();
                        let lhs_int: i32;
                        let rhs_int: i32;
                        match lhs_value {
                            VariableValue::Int(i0) => { lhs_int = i0; }
                            _ => unreachable!()
                        }
                        match rhs_value {
                            VariableValue::Int(i1) => { rhs_int = i1; }
                            _ => unreachable!()
                        }
                        if rhs_int == 0 { return Err(ExpressionEvaluationFail(format!("try to divide 0"))); }
                        return Ok(Variable::new_int(String::from(""), lhs_int % rhs_int));
                    }
                    else {
                        return Err(ExpressionEvaluationFail(format!("% operator does not support {} + {}", String::from(lhs_type), String::from(rhs_type))));
                    }
                },
                Rule::OP_Power => {
                    if lhs_type == DataType::IntType && rhs_type == DataType::IntType {
                        let lhs_value = lhs.get_var_value().get_raw_value();
                        let rhs_value = rhs.get_var_value().get_raw_value();
                        let lhs_int: i32;
                        let rhs_int: i32;
                        match lhs_value {
                            VariableValue::Int(i0) => { lhs_int = i0; }
                            _ => unreachable!()
                        }
                        match rhs_value {
                            VariableValue::Int(i1) => { rhs_int = i1; }
                            _ => unreachable!()
                        }
                        if rhs_int < 0 { return Err(ExpressionEvaluationFail(format!("power operation on integers cannot have negative index"))); }
                        let rhs_int = rhs_int as u32;
                        return Ok(Variable::new_int(String::from(""),lhs_int.pow(rhs_int)));
                    }
                    else if (lhs_type == DataType::FloatType && rhs_type == DataType::FloatType) ||
                        (lhs_type == DataType::FloatType && rhs_type == DataType::IntType) ||
                        (lhs_type == DataType::IntType && rhs_type == DataType::FloatType) {
                        let lhs_value = lhs.get_var_value().get_raw_value();
                        let rhs_value = rhs.get_var_value().get_raw_value();
                        let lhs_float:f32;
                        let rhs_float:f32;
                        match lhs_value {
                            VariableValue::Int(v) => {
                                lhs_float = v as f32;
                            }
                            VariableValue::Float(v) => {
                                lhs_float = v;
                            }
                            _ => unreachable!()
                        }
                        match rhs_value {
                            VariableValue::Int(v) => {
                                rhs_float = v as f32;
                            }
                            VariableValue::Float(v) => {
                                rhs_float = v;
                            }
                            _ => unreachable!()
                        }
                        return Ok(Variable::new_float(String::from(""), lhs_float.powf(rhs_float)));
                    }
                    else {
                        return Err(ExpressionEvaluationFail(format!("^ operator does not support {} + {}", String::from(lhs_type), String::from(rhs_type))));
                    }
                },
                _ => unreachable!(),
            }
        },
    )
}

pub fn infer_variable(var: Arc<RwLock<Variable>>, scope: Arc<RwLock<Scope>>, project: Arc<RwLock<Project>>) -> Result<(), ParserErrorCode> {
    if var.read().unwrap().get_var_value().get_infer_state() == InferState::Inferred { return Ok(()); }

    //type check
    let origin_var_type = var.read().unwrap().get_type();
    if *origin_var_type.read().unwrap() == DataType::PackageType { return Ok(()); }

    //import package var
    if var.read().unwrap().get_name().contains("$package$") { return Ok(()); }

    let var_type = var.read().unwrap().get_type();
    let mut inferred_value;
    match (*var_type.read().unwrap()).clone() {
        DataType::UnknownType | DataType::IntType | DataType::StringType | DataType::BoolType | DataType::FloatType | DataType::ClockDomainType | DataType::ArrayType(_) => {
            let raw_exp = var.read().unwrap().get_var_value().get_raw_exp();
            let value_result = parse_eval_exp(raw_exp.clone(), scope.clone(), project.clone());
            if value_result.is_err() {
                return Err(value_result.err().unwrap());
            }
            inferred_value = value_result.ok().unwrap();
            let mut var_value = inferred_value.get_var_value();
            var_value.set_raw_exp(raw_exp.clone());
            inferred_value.set_var_value(var_value);
        }
        DataType::LogicalDataType(logical_data_type) => {
            let result = evaluation_type::infer_logical_type(logical_data_type.clone(), scope.clone(), project.clone());
            if result.is_err() { return Err(result.err().unwrap()); }
            inferred_value = Variable::new_with_value(String::from(""), DataType::LogicalDataType(logical_data_type.clone()), VariableValue::LogicalDataType(logical_data_type));
        }

        //following region will process the values which should be by-passed
        DataType::ProxyImplement(_,_) => {
            //do nothing
            return Ok(());
        }
        _ => unreachable!(),
    };

    //type check
    {
        if *origin_var_type.read().unwrap() != DataType::UnknownType {
            let inferred_type = inferred_value.get_type();
            let origin_type = (*origin_var_type.read().unwrap()).clone();
            let inferred_type = (*inferred_type.read().unwrap()).clone();
            if origin_type != inferred_type {
                return Err(ExpressionEvaluationFail(format!("type mismatch: {} != {}", String::from(origin_type), String::from(inferred_type))));
            }
        }
    }

    //assign value
    {
        let mut var_write = var.write().unwrap();
        var_write.set_var_value(inferred_value.get_var_value());
        var_write.set_type(inferred_value.get_type());
    }
    return Ok(());
}
