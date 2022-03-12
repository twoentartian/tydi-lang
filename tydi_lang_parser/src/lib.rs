extern crate pest;
#[macro_use]
extern crate pest_derive;
#[macro_use]
extern crate lazy_static;
extern crate num_cpus;
extern crate threadpool;
extern crate rand;

use std::fs;
use std::string::ParseError;
use std::sync::{Arc, RwLock};

use ParserErrorCode::{AnalysisCodeStructureFail, FileError, ImplementEvaluationFail};
use pest::{Parser};
use pest::iterators::{Pairs};
use tydi_lang_raw_ast::bit_null_type::LogicalBit;
use tydi_lang_raw_ast::data_type::DataType;
use tydi_lang_raw_ast::inferable::{NewInferable, Inferable};

extern crate tydi_lang_raw_ast;

use tydi_lang_raw_ast::{project_arch};
use tydi_lang_raw_ast::scope::*;
use tydi_lang_raw_ast::{not_inferred, inferred, infer_logical_data_type, infer_port, infer_implement};
use tydi_lang_raw_ast::implement::ImplementType;

mod test_lex;
mod test_parse_project;
mod test_evaluation_simple;
pub mod evaluation_var;
pub mod evaluation_type;
pub mod evaluation_streamlet;
pub mod evaluation_implement;
pub mod evaluation;
pub mod built_in_ids;
mod util;

#[derive(Parser)]
#[grammar = "tydi_lang_syntax.pest"]
pub struct TydiParser;

#[derive(Clone, Debug)]
pub enum ParserErrorCode {
    Unknown,
    ParserError(String),
    FileError(String),
    AnalysisCodeStructureFail(String),
    ExpressionEvaluationFail(String),
    TypeEvaluationFail(String),
    StreamletEvaluationFail(String),
    ImplementEvaluationFail(String),
    PackageNotImported(String),
}

impl From<ParserErrorCode> for String {
    fn from(e: ParserErrorCode) -> Self {
        return match e {
            ParserErrorCode::Unknown => { format!("UnknownError") }
            ParserErrorCode::ParserError(s) => { format!("ParserError:{}", s) }
            ParserErrorCode::FileError(s) => { format!("FileError:{}", s) }
            ParserErrorCode::AnalysisCodeStructureFail(s) => { format!("AnalysisCodeStructureFail:{}", s) }
            ParserErrorCode::ExpressionEvaluationFail(s) => { format!("ExpressionEvaluationFail:{}", s) }
            ParserErrorCode::TypeEvaluationFail(s) => { format!("TypeEvaluationFail:{}", s) }
            ParserErrorCode::StreamletEvaluationFail(s) => { format!("StreamletEvaluationFail:{}", s) }
            ParserErrorCode::ImplementEvaluationFail(s) => { format!("ImplementEvaluationFail:{}", s) }
            ParserErrorCode::PackageNotImported(s) => { format!("PackageNotImported:{}", s) }
        }
    }
}

pub fn parse_multi_files_st(project_name: String, file_paths: Vec<String>) -> Result<Arc<RwLock<project_arch::Project>>, Vec<Result<(),ParserErrorCode>>> {
    use std::path::Path;
    use std::ffi::OsStr;

    let mut error_flag = false;
    let output_project = Arc::new(RwLock::new(project_arch::Project::new(project_name.clone())));
    let mut errors:Vec<Result<(),ParserErrorCode>>  = vec![];
    for file_path in file_paths {
        let result = parse_to_memory(file_path.clone());
        if result.is_err() {
            error_flag = true;
            errors.push(Err(result.err().unwrap()));
            continue;
        }
        let (package, _) = result.ok().unwrap();
        let file_path_fs= Path::new(&file_path);
        if Some(OsStr::new(&format!("{}.td", package.get_name()))) != file_path_fs.file_name() {
            error_flag = true;
            errors.push(Err(FileError(format!("{} has a different package name", file_path.clone()))));
        }
        let result = output_project.write().unwrap().with_package(package);
        if result.is_err() {
            error_flag = true;
            errors.push(Err(AnalysisCodeStructureFail(String::from(result.err().unwrap()))));
        }
        else { errors.push(Ok(())); }
    }

    if !error_flag {
        return Ok(output_project);
    }
    else {
        return Err(errors);
    }
}

pub fn parse_multi_files_mt(project_name: String, file_paths: Vec<String>, worker: Option<usize>) -> Result<(Arc<RwLock<project_arch::Project>>, HashMap<String, String>), Vec<(String, Result<(),ParserErrorCode>)>> {
    use threadpool::ThreadPool;
    use std::sync::mpsc;
    use std::path::Path;
    use std::ffi::OsStr;

    let output_project = Arc::new(RwLock::new(project_arch::Project::new(project_name.clone())));
    let worker_u32 = match worker {
        None => { num_cpus::get() }
        Some(v) => { v }
    };

    let pool = ThreadPool::new(worker_u32);
    let (tx, rx) = mpsc::channel();

    let ast_trees: Arc<RwLock<HashMap<String, String>>> = Arc::new(RwLock::new(HashMap::new()));
    for file_path_index in 0..file_paths.len() {
        let output_project = output_project.clone();
        let tx_temp = mpsc::Sender::clone(&tx);
        let file_path = file_paths[file_path_index].clone();
        let ast_tree = ast_trees.clone();
        pool.execute(move|| {
            let result = parse_to_memory(file_path.clone());
            if result.is_err() {
                let result = tx_temp.send((file_path.clone(), Err(AnalysisCodeStructureFail(String::from(result.err().unwrap())))));
                result.unwrap();
                return;
            }
            let (package, ast) = result.ok().unwrap();
            {
                ast_tree.write().unwrap().insert(file_path.clone(), ast);
            }
            let file_path_fs= Path::new(&file_path);
            if Some(OsStr::new(&format!("{}.td", package.get_name()))) != file_path_fs.file_name() {
                let result = tx_temp.send((file_path.clone(), Err(FileError(format!("{} has a different package name", file_path.clone())))));
                result.unwrap();
                return;
            }
            {
                let result = output_project.write().unwrap().with_package(package);
                if result.is_err() {
                    let result = tx_temp.send((file_path.clone(), Err(AnalysisCodeStructureFail(String::from(result.err().unwrap())))));
                    result.unwrap();
                    return;
                }
            }
            let result = tx_temp.send((file_path.clone(), Ok(())));
            result.unwrap();
            return;
        });
    }
    pool.join();

    let mut errors: Vec<(String,Result<(),ParserErrorCode>)> = vec![];
    let mut error_flag = false;
    for single_rx in rx.try_iter() {
        errors.push(single_rx.clone());
        match single_rx.1.clone() {
            Ok(_) => {}
            Err(_) => { error_flag = true }
        }
    }

    if !error_flag {
        return Ok((output_project, ast_trees.read().unwrap().clone()));
    }
    else {
        return Err(errors);
    }
}


fn parse_to_memory(file_path: String) -> Result<(project_arch::Package, String), ParserErrorCode> {
    let unparsed_file_result = fs::read_to_string(file_path.clone());
    if unparsed_file_result.is_err() { return Err(ParserErrorCode::FileError(unparsed_file_result.err().unwrap().to_string())); }
    let unparsed_file_content = unparsed_file_result.ok().unwrap();
    let tydi_ast_result = TydiParser::parse(Rule::Start, &unparsed_file_content);
    if tydi_ast_result.is_err() { return Err(ParserErrorCode::ParserError(tydi_ast_result.err().unwrap().to_string())); }
    let mut tydi_ast_result = tydi_ast_result.unwrap();
    let output_ast = format!("{}", tydi_ast_result.clone());

    let mut output_package = project_arch::Package::new(String::from("unknown_package"));

    let tydi_ast = tydi_ast_result.next().unwrap();
    match tydi_ast.clone().as_rule() {
        Rule::Start => {
            let inner_rules = tydi_ast.clone().into_inner();
            let result = parse_start(inner_rules, &mut output_package);
            if result.is_err() { return Err(result.err().unwrap()); }
        }
        _ => unreachable!(),
    }

    return Ok((output_package, output_ast));
}

fn parse_start(start_elements: Pairs<Rule>, output_package: &mut project_arch::Package) -> Result<(), ParserErrorCode> {
    for start_element in start_elements.into_iter() {
        match start_element.as_rule() {
            Rule::ID => {
                output_package.set_name(start_element.as_str().to_string());
            },
            Rule::StartElementAImport => {
                let inner_import_elements = start_element.into_inner();
                for single_import_element in inner_import_elements.into_iter() {
                    match single_import_element.as_rule() {
                        Rule::ID => {
                            let element_name = single_import_element.as_str().to_string();
                            let result = output_package.scope.write().unwrap().new_variable(format!("{}{}", *built_in_ids::PACKAGE_PREFIX, element_name.clone()), DataType::PackageType, String::from(""));
                            if result.is_err() { return Err(ParserErrorCode::AnalysisCodeStructureFail(String::from(result.err().unwrap()))); }
                        },
                        _ => { unreachable!() },
                    }
                }
            },
            Rule::StartElementAStatement => {
                let result = parse_statement(start_element.into_inner(), output_package);
                if result.is_err() { return Err(result.err().unwrap()); }
            },
            Rule::EOI => {},
            _ => { unreachable!() },
        }
    }

    return Ok(());
}

fn parse_function(statement: Pairs<Rule>, scope: Arc<RwLock<Scope>>) -> Result<(), ParserErrorCode> {
    let mut function_name = String::from("");
    let mut exps: Vec<Arc<RwLock<Variable>>> = vec![];
    let mut start_pos = 0;
    let mut end_pos = 0;
    for item in statement.into_iter() {
        match item.as_rule() {
            Rule::ID => {
                function_name = item.as_str().to_string();
            }
            Rule::Exp => {
                exps.push(Arc::new(RwLock::new(Variable::new(String::from(""), DataType::UnknownType, item.as_str().to_string()))));
                start_pos = item.as_span().start_pos().pos();
                end_pos = item.as_span().end_pos().pos();
            }
            _ => { unreachable!() },
        }
    }

    //assert function
    if function_name == "assert" {
        if exps.len() == 1 {
            let exp = exps[0].clone();
            let mut assert = Assert::new(format!("assert_{}_{}", start_pos, end_pos));
            assert.set_var(exp.clone());
            let result = scope.write().unwrap().with_assert(Arc::new(RwLock::new(assert)));
            if result.is_err() {
                return Err(ParserErrorCode::AnalysisCodeStructureFail(String::from(result.err().unwrap())));
            }
        }
        else if exps.len() == 2 {
            let exp = exps[0].clone();
            let msg = exps[1].clone();
            let mut assert = Assert::new(format!("assert_{}_{}", start_pos, end_pos));
            assert.set_var(exp.clone());
            assert.set_msg(Some(msg.clone()));

            let result = scope.write().unwrap().with_assert(Arc::new(RwLock::new(assert)));
            if result.is_err() {
                return Err(ParserErrorCode::AnalysisCodeStructureFail(String::from(result.err().unwrap())));
            }
        }
        else {
            return Err(ParserErrorCode::AnalysisCodeStructureFail(format!("assert function only accepts one or two arguments")));
        }
    }
    else {
        return Err(ParserErrorCode::AnalysisCodeStructureFail(format!("unknown function: {}", function_name.clone())));
    }

    return Ok(());
}

fn parse_statement(statement: Pairs<Rule>, output_package: &mut project_arch::Package) -> Result<(), ParserErrorCode> {
    for element in statement.into_iter() {
        match element.as_rule() {
            Rule::StatementConstAssign => {
                parse_const_assign(element.into_inner(), output_package.scope.clone())?;
            },
            Rule::StatementTypeAssign => {
                parse_type_assign(element.into_inner(), output_package.scope.clone())?;
            },
            Rule::StatementDeclareLogicalDataType => {
                parse_type_declare(element.into_inner(), output_package.scope.clone())?;
            },
            Rule::StatementDeclareStreamLet => {
                parse_streamlet_declare(element.into_inner(), output_package.scope.clone())?;
            },
            Rule::StatementDeclareImplementation => {
                parse_implement_declare(element.into_inner(), output_package.scope.clone())?;
            },
            Rule::StatementDeclareImplInst => {
                parse_global_instance(element.into_inner(), output_package.scope.clone())?;
            },
            Rule::Function => {
                parse_function(element.into_inner(), output_package.scope.clone())?;
            },
            Rule::StatementConstDeclare => {
                parse_const_declare(element.into_inner(), output_package.scope.clone())?;
            },
            _ => { unreachable!() },
        }
    }
    return Ok(());
}

fn parse_global_instance(statement: Pairs<Rule>, scope: Arc<RwLock<Scope>>) -> Result<(), ParserErrorCode> {
    let mut instance_name = String::from("");
    let mut derived_implement_package: Option<String> = None;
    let mut derived_implement_name = String::from("");
    let mut derived_implement_argexps = vec![];
    for element in statement.into_iter() {
        match element.as_rule() {
            Rule::ID => {
                instance_name = element.as_str().to_string();
            },
            Rule::Extern_Intern_Id => {
                let result = parse_internal_external_id(element.into_inner(), scope.clone());
                if result.is_err() { return Err(result.err().unwrap()); }
                let (package_name, streamlet_name) = result.ok().unwrap();
                derived_implement_package = package_name;
                derived_implement_name = streamlet_name;
            },
            Rule::ArgExps => {
                let result = parse_argexps(element.into_inner(), scope.clone());
                if result.is_err() { return Err(result.err().unwrap()); }
                let result = result.ok().unwrap();
                derived_implement_argexps = result;
            },
            _ => { unreachable!() },
        }
    }

    let result = scope.write().unwrap().new_instance(instance_name, derived_implement_package, not_inferred!(infer_implement!(), derived_implement_name), derived_implement_argexps);
    if result.is_err() { return Err(AnalysisCodeStructureFail(String::from(result.err().unwrap()))); }

    return Ok(());
}

fn parse_implement_body_declare_instance(statement: Pairs<Rule>, scope: Arc<RwLock<Scope>>) -> Result<(), ParserErrorCode> {
    let mut instance_array_type = InstanceArray::SingleInstance;
    let mut instance_name = String::from("");
    let mut derived_implement_package: Option<String> = None;
    let mut derived_implement_name = String::from("");
    let mut derived_implement_argexps = vec![];
    let mut instance_docu: Option<String> = None;
    for element in statement.into_iter() {
        match element.as_rule() {
            Rule::ID => {
                instance_name = element.as_str().to_string();
            },
            Rule::Extern_Intern_Id => {
                let result = parse_internal_external_id(element.into_inner(), scope.clone());
                if result.is_err() { return Err(result.err().unwrap()); }
                let (package_name, streamlet_name) = result.ok().unwrap();
                derived_implement_package = package_name;
                derived_implement_name = streamlet_name;
            },
            Rule::ArgExps => {
                let result = parse_argexps(element.into_inner(), scope.clone());
                if result.is_err() { return Err(result.err().unwrap()); }
                let result = result.ok().unwrap();
                derived_implement_argexps = result;
            },
            Rule::Exp => {
                let var = Variable::new(String::from(""), DataType::IntType, element.as_str().to_string());
                instance_array_type = InstanceArray::ArrayInstance(Arc::new(RwLock::new(var)));
            },
            Rule::DOCUMENT => {
                instance_docu = Some(element.as_str().to_string());
            },
            _ => { unreachable!() },
        }
    }

    match instance_array_type {
        InstanceArray::ArrayInstance(array_) => {
            let result = scope.write().unwrap().new_instance_array(instance_name.clone(), derived_implement_package, not_inferred!(infer_implement!(), derived_implement_name), derived_implement_argexps, array_.clone());
            if result.is_err() { return Err(AnalysisCodeStructureFail(String::from(result.err().unwrap()))); }
        },
        InstanceArray::SingleInstance => {
            let result = scope.write().unwrap().new_instance(instance_name.clone(), derived_implement_package, not_inferred!(infer_implement!(), derived_implement_name), derived_implement_argexps);
            if result.is_err() { return Err(AnalysisCodeStructureFail(String::from(result.err().unwrap()))); }
        },
        _ => { unreachable!() }
    }

    //document
    let inst = scope.read().unwrap().resolve_instance_in_current_scope(instance_name.clone()).unwrap();
    {
        inst.write().unwrap().set_document(instance_docu);
    }

    return Ok(());
}

fn parse_logical_type_slice(slice: Pairs<Rule>, _: Arc<RwLock<Scope>>) -> Result<(Inferable<Arc<RwLock<Port>>>, PortOwner, PortArray), ParserErrorCode> {
    for single_slice in slice.into_iter() {
        match single_slice.as_rule() {
            Rule::LogicalTypeSliceCompound => {
                let mut port_exp = not_inferred!(infer_port!(), String::from(""));
                let mut port_owner = PortOwner::SelfOwner;
                let mut port_array = PortArray::SinglePort;
                let mut index = -1;
                for element in single_slice.into_inner().into_iter() {
                    match element.as_rule() {
                        Rule::ID => {
                            index = index + 1;
                            match index {
                                0 => {
                                    let owner_name = element.as_str().to_string();
                                    if owner_name.clone().to_lowercase() == "self" { port_owner = PortOwner::SelfOwner; }
                                    else { port_owner = PortOwner::ExternalOwner(owner_name.clone(), None, None); }
                                },
                                1 => {
                                    port_exp = not_inferred!(infer_port!(), element.as_str().to_string());
                                },
                                _ => { unreachable!() }
                            }
                        },
                        Rule::Exp => {
                            match index {
                                0 => {
                                    match port_owner.clone() {
                                        PortOwner::ExternalOwner(owner_name, s,_) => {
                                            port_owner = PortOwner::ExternalOwner(owner_name, s, Some(Arc::new(RwLock::new(Variable::new(String::from(""), DataType::IntType, element.as_str().to_string())))));
                                        },
                                        PortOwner::SelfOwner => {
                                            return Err(AnalysisCodeStructureFail(format!("{} is not allowed after a self slice", element.as_str().to_string())));
                                        },
                                        _ => { unreachable!() }
                                    }
                                },
                                1 => {
                                    port_array = PortArray::ArrayPort(Arc::new(RwLock::new(Variable::new(String::from(""), DataType::IntType, element.as_str().to_string()))));
                                },
                                _ => { unreachable!() }
                            }
                        },
                        _ => { unreachable!() },
                    }
                }
                return Ok((port_exp, port_owner, port_array));
            },
            Rule::LogicalTypeSliceSelf => {
                let mut port_exp = not_inferred!(infer_port!(), String::from(""));
                let port_owner = PortOwner::SelfOwner;
                let mut port_array = PortArray::SinglePort;
                for element in single_slice.into_inner().into_iter() {
                    match element.as_rule() {
                        Rule::ID => {
                            port_exp = not_inferred!(infer_port!(), element.as_str().to_string());
                        },
                        Rule::Exp => {
                            port_array = PortArray::ArrayPort(Arc::new(RwLock::new(Variable::new(String::from(""), DataType::IntType, element.as_str().to_string()))));
                        },
                        _ => { unreachable!() },
                    }
                }
                return Ok((port_exp, port_owner, port_array));
            },
            _ => { unreachable!() },
        }
    }
    unreachable!();
}

fn parse_else_block(block: Pairs<Rule>, scope: Arc<RwLock<Scope>>) -> Result<(), ParserErrorCode> {
    for item in block.into_iter() {
        match item.clone().as_rule() {
            Rule::ImplementationBody => {
                let process = parse_implement_body(item.into_inner(), scope.clone())?;
                if process.is_some() { return Err(ImplementEvaluationFail(format!("cannot define simulation process in else block"))); }
            }
            _ => { unreachable!() }
        }
    }
    return Ok(());
}

fn parse_elif_block(block: Pairs<Rule>, scope: Arc<RwLock<Scope>>, elif_block: & mut ElifScope) -> Result<(), ParserErrorCode> {
    for item in block.into_iter() {
        match item.clone().as_rule() {
            Rule::Exp => {
                elif_block.set_elif_exp(Arc::new(RwLock::new(Variable::new(String::from(""), DataType::BoolType, item.as_str().to_string()))));
            }
            Rule::ImplementationBody => {
                let process = parse_implement_body(item.into_inner(), scope.clone())?;
                if process.is_some() { return Err(ImplementEvaluationFail(format!("cannot define simulation process in elif block"))); }
            }
            _ => { unreachable!() }
        }
    }
    return Ok(());
}

fn parse_implement_body(statement: Pairs<Rule>, scope: Arc<RwLock<Scope>>) -> Result</*process:*/Option<String>, ParserErrorCode> {
    let mut output_process: Option<String> = None;
    for single_stat in statement.into_iter() {
        match single_stat.clone().as_rule() {
            Rule::ImplementationBodyDeclareInstance => {
                let result = parse_implement_body_declare_instance(single_stat.clone().into_inner(), scope.clone());
                if result.is_err() { return Err(result.err().unwrap()); }
            },
            Rule::ImplementationBodyDeclareInstanceArray => {
                let result = parse_implement_body_declare_instance(single_stat.clone().into_inner(), scope.clone());
                if result.is_err() { return Err(result.err().unwrap()); }
            },
            Rule::ImplementationBodyDeclareConstInImpl => {
                let result = parse_const_assign(single_stat.clone().into_inner(), scope.clone());
                if result.is_err() { return Err(result.err().unwrap()); }
            },
            Rule::ImplementationBodyIfBlock => {
                let name = format!("if_{}_{}", single_stat.clone().as_span().start(),single_stat.clone().as_span().end());
                let mut if_block: IfScope = IfScope::new(name.clone(), Arc::new(RwLock::new(Variable::new_bool(String::from(""), true))));
                for item in single_stat.clone().into_inner().into_iter() {
                    match item.as_rule() {
                        Rule::Exp => {
                            {
                                if_block.set_if_exp(Arc::new(RwLock::new(Variable::new(String::from(""), DataType::BoolType, item.as_str().to_string()))));
                            }
                        },
                        Rule::ImplementationBody => {
                            let process = parse_implement_body(item.into_inner(), if_block.get_scope().clone())?;
                            if process.is_some() { return Err(ImplementEvaluationFail(format!("cannot define simulation process in if block"))); }
                        },
                        Rule::ElifBlock => {
                            let name = format!("elif_{}_{}", item.clone().as_span().start(), item.clone().as_span().end());
                            let mut elif_block = ElifScope::new(name);
                            let result = parse_elif_block(item.into_inner(), elif_block.get_scope(), &mut elif_block);
                            if result.is_err() { return Err(result.err().unwrap()); }
                            let mut previous_elifs = if_block.get_elif();
                            previous_elifs.push(elif_block);
                            if_block.set_elif(previous_elifs);
                        },
                        Rule::ElseBlock => {
                            let name = format!("else_{}_{}", item.clone().as_span().start(), item.clone().as_span().end());
                            let else_block = ElseScope::new(name);
                            let result = parse_else_block(item.into_inner(), else_block.get_scope().clone());
                            if result.is_err() { return Err(result.err().unwrap()); }
                            if_block.set_else(Some(else_block));
                        },
                        _ => { unreachable!() }
                    }
                }
                let parent_scope_name = scope.read().unwrap().get_name();
                {
                    let result = scope.write().unwrap().with_if_block(Arc::new(RwLock::new( if_block)), parent_scope_name);
                    if result.is_err() { return Err(AnalysisCodeStructureFail(String::from(result.err().unwrap()))); }
                }
            },
            Rule::ImplementationBodyForBlock => {
                let name = format!("for_{}_{}", single_stat.clone().as_span().start(),single_stat.clone().as_span().end());
                let temp_var = Arc::new(RwLock::new(Variable::new_bool(String::from(""), true)));
                let mut for_block: ForScope = ForScope::new(name.clone(), temp_var.clone(), temp_var.clone());
                for item in single_stat.clone().into_inner().into_iter() {
                    match item.as_rule() {
                        Rule::ID => {
                            let var_name = item.as_str().to_string();
                            let for_scope = for_block.get_scope();
                            let mut for_scope = for_scope.write().unwrap();
                            let result = for_scope.new_variable(var_name.clone(), DataType::UnknownType, format!("{}{}", *built_in_ids::ARG_PREFIX, var_name.clone()));
                            if result.is_err() { return Err(AnalysisCodeStructureFail(String::from(result.err().unwrap()))); }
                            for_block.set_var_exp(Arc::new(RwLock::new(Variable::new(var_name.clone(), DataType::UnknownType, var_name.clone()))));
                        }
                        Rule::Exp => {
                            for_block.set_array_exp(Arc::new(RwLock::new(Variable::new(String::from(""), DataType::UnknownType, item.as_str().to_string()))));
                        }
                        Rule::ImplementationBody => {
                            let process = parse_implement_body(item.into_inner(), for_block.get_scope().clone())?;
                            if process.is_some() { return Err(ImplementEvaluationFail(format!("cannot define simulation process in for block"))); }
                        }
                        _ => { unreachable!() }
                    }
                }
                let parent_scope_name = scope.read().unwrap().get_name();
                {
                    let result = scope.write().unwrap().with_for_block(Arc::new(RwLock::new( for_block)), parent_scope_name);
                    if result.is_err() { return Err(AnalysisCodeStructureFail(String::from(result.err().unwrap()))); }
                }
            },
            Rule::ImplementationBodyDeclareProcess => {
                output_process = Some(single_stat.as_str().to_string());
            },
            Rule::ImplementationBodyDeclareNet | Rule::ImplementationBodyDeclareDelayedNet => {
                let mut connection = Connection::new(String::from(""), not_inferred!(infer_port!(), String::from("")), not_inferred!(infer_port!(), String::from("")), Variable::new_int(String::from(""), 0));
                let mut lhs_rhs_counter = 0;
                for element in single_stat.clone().into_inner().into_iter() {
                    match element.as_rule() {
                        Rule::DOCUMENT => {
                            connection.set_document(Some(element.as_str().to_string()));
                        },
                        Rule::LogicalTypeSlice => {
                            let result = parse_logical_type_slice(element.into_inner(), scope.clone());
                            if result.is_err() { return Err(result.err().unwrap()); }
                            let (infer_slice_exp, owner, array) = result.ok().unwrap();
                            match lhs_rhs_counter {
                                0 => {
                                    connection.set_lhs_port(infer_slice_exp);
                                    connection.set_lhs_port_owner(owner);
                                    connection.set_lhs_port_array_type(array);
                                },
                                1 =>{
                                    connection.set_rhs_port(infer_slice_exp);
                                    connection.set_rhs_port_owner(owner);
                                    connection.set_rhs_port_array_type(array);
                                },
                                _ => { unreachable!() },
                            }
                            lhs_rhs_counter = lhs_rhs_counter + 1;
                        },
                        Rule::NetName => {
                            for item in element.into_inner().into_iter() {
                                match item.as_rule() {
                                    Rule::ID => {
                                        connection.set_name(item.as_str().to_string());
                                    },
                                    _ => { unreachable!() }
                                }
                            }
                        },
                        Rule::Exp => {
                            connection.set_delay(Arc::new(RwLock::new(Variable::new(String::from(""), DataType::UnknownType, element.as_str().to_string()))));
                        },
                        _ => { unreachable!() }
                    }
                }

                // add connection
                connection.set_name(format!("connection_{}-{}", single_stat.clone().as_span().start(), single_stat.clone().as_span().end()));
                {
                    let result = scope.write().unwrap().with_connection(Arc::new(RwLock::new(connection)));
                    if result.is_err() { return Err(AnalysisCodeStructureFail(String::from(result.err().unwrap()))); }
                }
            },
            Rule::Function => {
                parse_function(single_stat.into_inner(), scope.clone())?;
            },
            _ => { unreachable!() },
        }
    }

    return Ok(output_process);
}

fn parse_implement_declare(statement: Pairs<Rule>, scope: Arc<RwLock<Scope>>) -> Result<(), ParserErrorCode> {
    let mut implement = Implement::new(String::from(""), ImplementType::UnknownType);
    let mut implement_args : Vec<Variable> = vec![];
    let mut derived_streamlet_type = DataType::UnknownType;
    for element in statement.into_iter() {
        match element.as_rule() {
            Rule::ID => {
                implement.set_name(element.as_str().to_string());
            },
            Rule::Arg => {
                let result = parse_arg_to_var(element.into_inner(), implement.get_scope());
                if result.is_err() { return Err(result.err().unwrap()); }
                let result = result.ok().unwrap();
                match (*result.get_type().read().unwrap()).clone() {
                    DataType::IntType => {}
                    DataType::StringType => {}
                    DataType::BoolType => {}
                    DataType::FloatType => {}
                    DataType::ClockDomainType => {}
                    DataType::ArrayType(_) => {}
                    DataType::LogicalDataType(_) => {}
                    DataType::ProxyImplementOfStreamlet(_,_) => {}
                    DataType::ExternalProxyImplementOfStreamlet(_,_,_) => {}

                    _ => return Err(AnalysisCodeStructureFail(format!("unaccepted implement template arg")))
                }
                implement_args.push(result);
            },
            Rule::Extern_Intern_Id => {
                let result = parse_internal_external_id(element.into_inner(), scope.clone());
                if result.is_err() { return Err(result.err().unwrap()); }
                let (pack_name,id) = result.ok().unwrap();
                match pack_name {
                    None => { derived_streamlet_type = DataType::ProxyStreamlet(id, vec![]); }
                    Some(pack_name) => { derived_streamlet_type = DataType::ExternalProxyStreamlet(pack_name, id, vec![]); }
                }
            },
            Rule::ArgExps => {
                let result = parse_argexps(element.into_inner(), implement.get_scope());
                if result.is_err() { return Err(result.err().unwrap()); }
                let result = result.ok().unwrap();
                match derived_streamlet_type {
                    DataType::ProxyStreamlet(id, _) => { derived_streamlet_type = DataType::ProxyStreamlet(id, result); }
                    DataType::ExternalProxyStreamlet(package_name, id, _) => { derived_streamlet_type = DataType::ExternalProxyStreamlet(package_name, id, result); }
                    _ => { unreachable!() }
                }
            },
            Rule::ImplementationBody => {
                let process = parse_implement_body(element.into_inner(), implement.get_scope())?;
                implement.set_simulation_process(process);
            },
            Rule::DOCUMENT => {
                implement.set_document(Some(element.as_str().to_string()));
            },
            Rule::ImplementationExternalFlag => {
                implement.set_external_implement_flag(true);
            }
            _ => { unreachable!() },
        }
    }

    // set implement type
    if implement_args.is_empty() {
        implement.set_type(ImplementType::NormalImplement);
    }
    else {
        let mut templates_args = vec![];
        for implement_arg in implement_args {
            templates_args.push(Arc::new(RwLock::new(implement_arg)));
        }
        implement.set_type(ImplementType::TemplateImplement(templates_args));
    }

    // set implement scope
    {
        implement.get_scope().write().unwrap().new_relationship(scope.read().unwrap().self_ref.clone().unwrap(), ScopeRelationType::ImplementScopeRela);
    }

    // set derived streamlet
    let derived_streamlet = Variable::new(String::from(""), derived_streamlet_type, String::from(""));
    implement.set_derived_streamlet_var(Arc::new(RwLock::new(derived_streamlet)));

    // arrach implement
    {
        let result = scope.write().unwrap().with_implement(Arc::new(RwLock::new(implement)));
        if result.is_err() { return Err(ParserErrorCode::AnalysisCodeStructureFail(String::from(result.err().unwrap()))); }
    }

    return Ok(());
}

fn parse_internal_external_id(ids: Pairs<Rule>, _: Arc<RwLock<Scope>>) -> Result<(Option<String>, String),ParserErrorCode> {
    for id in ids.into_iter() {
        match id.as_rule() {
            Rule::ExternalId => {
                let exp = id.as_str().to_string();
                let results: Vec<&str> = exp.split(".").collect();
                return Ok((Some(String::from(results[0])), String::from(results[1])));
            },
            Rule::InternalId => {
                return Ok((None, id.as_str().to_string()));
            },
            _ => { unreachable!() }
        }
    }
    unreachable!()
}

fn parse_argexp_implement(exps: Pairs<Rule>, scope: Arc<RwLock<Scope>>) -> Result<Arc<RwLock<Variable>>,ParserErrorCode> {
    let mut data_type = DataType::UnknownType;
    for exp in exps.clone().into_iter() {
        match exp.as_rule() {
            Rule::Extern_Intern_Id =>{
                let result = parse_internal_external_id(exp.into_inner(), scope.clone());
                if result.is_err() { return Err(result.err().unwrap()); }
                let (package_name, implement_name) = result.ok().unwrap();
                match package_name.clone() {
                    None => { data_type = DataType::ProxyImplement(implement_name, vec![]); }
                    Some(pack_name) => { data_type = DataType::ExternalProxyImplement(pack_name, implement_name, vec![]); }
                }
            },
            Rule::ArgExps =>{
                let result = parse_argexps(exp.into_inner(), scope.clone());
                if result.is_err() { return Err(result.err().unwrap()); }
                let result = result.ok().unwrap();
                match data_type.clone() {
                    DataType::ProxyImplement(id, _) => { data_type = DataType::ProxyImplement(id, result); }
                    DataType::ExternalProxyImplement(pack,id,_) => { data_type = DataType::ExternalProxyImplement(pack,id, result); }
                    _ => { unreachable!() }
                }
            },
            _ => { unreachable!() },
        }
    }

    let output_var = Variable::new(String::from(""), data_type, exps.as_str().to_string());
    return Ok(Arc::new(RwLock::new(output_var)));
}

fn parse_argexps(exps: Pairs<Rule>, scope: Arc<RwLock<Scope>>) -> Result<Vec<Arc<RwLock<Variable>>>,ParserErrorCode> {
    let mut output = vec![];
    for exp in exps.into_iter() {
        match exp.clone().as_rule() {
            Rule::ArgExpTypeExp => {
                let logical_type_exp = exp.clone().into_inner();
                for exp in logical_type_exp.into_iter() {
                    match exp.clone().as_rule() {
                        Rule::LogicalType => {
                            let logical_type_result = get_logical_type(exp.clone().into_inner(), String::from(""),scope.clone());
                            if logical_type_result.is_err() { return Err(logical_type_result.err().unwrap()); }
                            let logical_type_result = logical_type_result.ok().unwrap();
                            let output_var = Variable::new(String::from(""), DataType::LogicalDataType(Arc::new(RwLock::new(logical_type_result))), exp.clone().as_str().to_string());
                            output.push(Arc::new(RwLock::new(output_var)));
                        },
                        _ => { unreachable!() },
                    }
                }
            },
            Rule::ArgExpExternalTypeExp => {
                let mut package_id = String::from("");
                let mut logical_type = LogicalDataType::UnknownLogicalDataType;
                for item in exp.clone().into_inner().into_iter() {
                    match item.as_rule() {
                        Rule::ID => {
                            package_id = item.as_str().to_string();
                        },
                        Rule::LogicalType => {
                            let result = get_logical_type(item.into_inner(), String::from(""), scope.clone());
                            if result.is_err() { return Err(result.err().unwrap()); }
                            logical_type = result.ok().unwrap();
                        },
                        _ => { unreachable!() },
                    }
                }
                let output_var = Variable::new(String::from(""), DataType::ExternalProxyType(package_id.clone(), Arc::new(RwLock::new(logical_type))), exp.clone().as_str().to_string());
                output.push(Arc::new(RwLock::new(output_var)));
            },
            Rule::ArgExpImplementExp => {
                let result = parse_argexp_implement(exp.into_inner(), scope.clone());
                if result.is_err() { return Err(result.err().unwrap()); }
                let result = result.ok().unwrap();
                output.push(result);
            },
            Rule::ArgExpConstExp => {
                let output_var = Variable::new(String::from(""), DataType::UnknownType, exp.clone().as_str().to_string());
                output.push(Arc::new(RwLock::new(output_var)));
            },
            _ => { unreachable!() }
        }
    }

    return Ok(output);
}

fn parse_arg_to_var(arg_exp: Pairs<Rule>, scope: Arc<RwLock<Scope>>) -> Result<Variable, ParserErrorCode> {
    let mut output = Variable::new(String::from(""), DataType::UnknownType, String::from(""));
    for arg in arg_exp.into_iter() {
        match arg.clone().as_rule() {
            Rule::ArgLogicalType => {
                let mut var_name = String::from("");
                for item in arg.clone().into_inner() {
                    match item.as_rule() {
                        Rule::ID => {
                            var_name = item.as_str().to_string();

                        },
                        _ => { unreachable!() },
                    }
                }

                // shadow var
                let shadow_var_name = format!("{}{}", *built_in_ids::ARG_PREFIX , var_name.clone());
                let var_type = DataType::LogicalDataType(Arc::new(RwLock::new(LogicalDataType::DummyLogicalData)));
                {
                    let result = scope.write().unwrap().new_variable(var_name.clone(), var_type.clone(), shadow_var_name.clone());
                    if result.is_err() { return Err(AnalysisCodeStructureFail(String::from(result.err().unwrap()))); }
                }
                output = Variable::new(shadow_var_name, var_type.clone(), String::from(""));
            },
            Rule::ArgValue => {
                let mut id_exp= String::from("");
                let mut type_exp= String::from("");
                for item in arg.clone().into_inner() {
                    match item.as_rule() {
                        Rule::ID => { id_exp = item.as_str().to_string(); },
                        Rule::TypeIndicatorBasicArrayType => { type_exp = item.as_str().to_string(); },
                        Rule::TypeIndicatorBasicType => { type_exp = item.as_str().to_string(); },
                        _ => { unreachable!() },
                    }
                }
                let result = convert_type_str_to_type(type_exp);
                if result.is_err() { return Err(result.err().unwrap()); }

                // shadow var
                let var_name = id_exp;
                let shadow_var_name = format!("{}{}", *built_in_ids::ARG_PREFIX, var_name.clone());
                let var_type = result.ok().unwrap();
                {
                    let result = scope.write().unwrap().new_variable(var_name.clone(), var_type.clone(), shadow_var_name.clone());
                    if result.is_err() { return Err(AnalysisCodeStructureFail(String::from(result.err().unwrap()))); }
                }
                output = Variable::new(shadow_var_name, var_type.clone(), String::from(""));
            },
            Rule::ArgImplementationType => {
                let mut id_exp= String::from("");
                let mut data_type = DataType::UnknownType;
                for item in arg.clone().into_inner() {
                    match item.as_rule() {
                        Rule::ID => {
                            id_exp = item.as_str().to_string();
                        },
                        Rule::Extern_Intern_Id => {
                            let result = parse_internal_external_id(item.into_inner(), scope.clone());
                            if result.is_err() { return Err(result.err().unwrap()); }
                            let (package, streamlet_id) = result.ok().unwrap();
                            match package {
                                None => { data_type = DataType::ProxyImplementOfStreamlet(streamlet_id, vec![]); }
                                Some(package_name) => { data_type = DataType::ExternalProxyImplementOfStreamlet(package_name, streamlet_id, vec![]); }
                            }
                        },
                        Rule::ArgExps => {
                            let result = parse_argexps(item.into_inner(), scope.clone());
                            if result.is_err() { return Err(result.err().unwrap()); }
                            let result = result.ok().unwrap();
                            match data_type.clone() {
                                DataType::ProxyImplementOfStreamlet(id,_) => { data_type = DataType::ProxyImplementOfStreamlet(id, result); }
                                DataType::ExternalProxyImplementOfStreamlet(package_name, id,_) => { data_type = DataType::ExternalProxyImplementOfStreamlet(package_name, id, result); }
                                _ => {unreachable!()}
                            }
                        },
                        _ => { unreachable!() }
                    }
                }

                // shadow var
                let var_name = id_exp.clone();
                let shadow_var_name = format!("{}{}", *built_in_ids::ARG_PREFIX , var_name.clone());
                let var_type = data_type.clone();
                {
                    let result = scope.write().unwrap().new_variable(var_name.clone(), var_type.clone(), shadow_var_name.clone());
                    if result.is_err() { return Err(AnalysisCodeStructureFail(String::from(result.err().unwrap()))); }
                }

                output = Variable::new(shadow_var_name, data_type.clone(), String::from(""));
            },
            _ => { unreachable!() },
        }
    }

    return Ok(output);
}

fn parse_direction(dir: Pairs<Rule>, _: Arc<RwLock<Scope>>) -> Result<PortDirection, ParserErrorCode> {
    let mut output_dir = PortDirection::Unknown;
    for d in dir.into_iter() {
        match d.clone().as_rule() {
            Rule::DirectionDirIn => {
                output_dir = PortDirection::Input;
            },
            Rule::DirectionDirOut => {
                output_dir = PortDirection::Output;
            },
            _ => { return Err(ParserErrorCode::AnalysisCodeStructureFail(format!("unknown port direction - {:?}", d.clone().as_span()))) }
        }
    }
    return Ok(output_dir);
}

fn parse_streamlet_declare(statement: Pairs<Rule>, scope: Arc<RwLock<Scope>>) -> Result<(), ParserErrorCode> {
    let mut streamlet = Streamlet::new(String::from(""), StreamletType::UnknownType);
    let mut streamlet_args : Vec<Variable> = vec![];
    for element in statement.into_iter() {
        match element.as_rule() {
            Rule::ID => {
                streamlet.set_name(element.as_str().to_string());
            },
            Rule::DOCUMENT => {
                streamlet.set_document(Some(element.as_str().to_string()));
            },
            Rule::Arg => {
                let result = parse_arg_to_var(element.into_inner(), streamlet.get_scope());
                if result.is_err() { return Err(result.err().unwrap()); }
                let result = result.ok().unwrap();
                match (*result.get_type().read().unwrap()).clone() {
                    DataType::IntType => {}
                    DataType::StringType => {}
                    DataType::BoolType => {}
                    DataType::FloatType => {}
                    DataType::ClockDomainType => {}
                    DataType::ArrayType(_) => {}
                    DataType::LogicalDataType(_) => {}
                    _ => return Err(AnalysisCodeStructureFail(format!("unaccepted streamlet template arg")))
                }
                streamlet_args.push(result.clone());
            },
            Rule::StreamLetBodyStreamLetPort => {
                let mut port_dir = PortDirection::Unknown;
                let mut exp = String::from("");
                let mut logical_type= LogicalDataType::UnknownLogicalDataType;
                let mut clock_domain: Option<Variable> = None;
                let mut docu: Option<String> = None;
                for item in element.clone().into_inner().into_iter() {
                    match item.as_rule() {
                        Rule::ID => {
                            exp = item.as_str().to_string();
                        },
                        Rule::LogicalType => {
                            let result = get_logical_type(item.clone().into_inner(), format!("{}Type{}_{}", &*crate::built_in_ids::GENERATED_ID_PREFIX, item.as_span().start_pos().pos(), item.as_span().end_pos().pos()),streamlet.get_scope());
                            if result.is_err() { return Err(result.err().unwrap()); }
                            logical_type = result.ok().unwrap();
                        },
                        Rule::Direction => {
                            let result = parse_direction(item.clone().into_inner(), streamlet.get_scope());
                            if result.is_err() { return Err(result.err().unwrap()); }
                            port_dir = result.ok().unwrap();
                        },
                        Rule::DOCUMENT => {
                            docu = Some(item.as_str().to_string());
                        },
                        Rule::Exp => {
                            clock_domain = Some(Variable::new(String::from(""), DataType::ClockDomainType, item.as_str().to_string()));
                        }
                        _ => { unreachable!() }
                    }
                }

                let result = streamlet.new_port(exp.clone(), inferred!(infer_logical_data_type!(), Arc::new(RwLock::new(logical_type))), port_dir);
                if result.is_err() { return Err(AnalysisCodeStructureFail(String::from(result.err().unwrap()))); }

                //set document
                let streamlet_scope = streamlet.get_scope();
                let port = streamlet_scope.read().unwrap().resolve_port_in_current_scope(exp.clone()).unwrap();
                match docu {
                    None => {}
                    Some(docu) => {
                        port.write().unwrap().set_document(Some(docu));
                    }
                }

                //set clockdomain
                match clock_domain {
                    None => {
                        port.write().unwrap().set_clock_domain(Arc::new(RwLock::new(Variable::new_with_value(String::from(""), DataType::ClockDomainType, VariableValue::ClockDomain(ClockDomainValue::Default)))));
                    }
                    Some(var) => {
                        port.write().unwrap().set_clock_domain(Arc::new(RwLock::new(var)) );
                    }
                }
            },
            Rule::StreamLetBodyStreamLetPortArray => {
                let mut port_dir = PortDirection::Unknown;
                let mut exp = String::from("");
                let mut logical_type= LogicalDataType::UnknownLogicalDataType;
                let mut array_var = Variable::new_int(String::from(""), 0);
                let mut array_var_flag = true;
                let mut clock_domain: Option<Variable> = None;
                let mut docu: Option<String> = None;
                for item in element.clone().into_inner().into_iter() {
                    match item.as_rule() {
                        Rule::ID => {
                            exp = item.as_str().to_string();
                        },
                        Rule::LogicalType => {
                            let result = get_logical_type(item.clone().into_inner(), format!("{}Type{}_{}", &*crate::built_in_ids::GENERATED_ID_PREFIX, item.as_span().start_pos().pos(), item.as_span().end_pos().pos()),streamlet.get_scope());
                            if result.is_err() { return Err(result.err().unwrap()); }
                            logical_type = result.ok().unwrap();
                        },
                        Rule::Direction => {
                            let result = parse_direction(item.clone().into_inner(), streamlet.get_scope());
                            if result.is_err() { return Err(result.err().unwrap()); }
                            port_dir = result.ok().unwrap();
                        },
                        Rule::Exp => {
                            if array_var_flag {
                                array_var_flag = false;
                                array_var = Variable::new(String::from(""), DataType::IntType, item.as_str().to_string());
                            }
                            else {
                                clock_domain = Some(Variable::new(String::from(""), DataType::ClockDomainType, item.as_str().to_string()));
                            }
                        },
                        Rule::DOCUMENT => {
                            docu = Some(item.as_str().to_string());
                        },
                        _ => { unreachable!() }
                    }
                }
                let result = streamlet.new_port_array(exp.clone(), inferred!(infer_logical_data_type!(), Arc::new(RwLock::new(logical_type))), port_dir, Arc::new(RwLock::new(array_var)));
                if result.is_err() { return Err(AnalysisCodeStructureFail(String::from(result.err().unwrap()))); }

                //set document
                let streamlet_scope = streamlet.get_scope();
                let port = streamlet_scope.read().unwrap().resolve_port_in_current_scope(exp.clone()).unwrap();
                match docu {
                    None => {}
                    Some(docu) => {
                        port.write().unwrap().set_document(Some(docu));
                    }
                }

                //set clockdomain
                match clock_domain {
                    None => {
                        port.write().unwrap().set_clock_domain(Arc::new(RwLock::new(Variable::new_with_value(String::from(""), DataType::ClockDomainType, VariableValue::ClockDomain(ClockDomainValue::Default)))));
                    }
                    Some(var) => {
                        port.write().unwrap().set_clock_domain(Arc::new(RwLock::new(var)));
                    }
                }
            },
            Rule::StreamLetBodyDeclareConstInStreamlet => {
                parse_const_assign(element.into_inner(), streamlet.get_scope())?;
            },
            Rule::Function => {
                parse_function(element.into_inner(), streamlet.get_scope())?;
            }
            _ => { unreachable!() },
        }
    }

    // set streamlet type
    if streamlet_args.is_empty() {
        streamlet.set_type(StreamletType::NormalStreamlet);
    }
    else {
        let mut templates_args = vec![];
        for streamlet_arg in streamlet_args {
            templates_args.push(Arc::new(RwLock::new(streamlet_arg)));
        }
        streamlet.set_type(StreamletType::TemplateStreamlet(templates_args));
    }

    // set streamlet scope
    {
        streamlet.get_scope().write().unwrap().new_relationship(scope.read().unwrap().self_ref.clone().unwrap(), ScopeRelationType::StreamletScopeRela);
    }
    {
        let result = scope.write().unwrap().with_streamlet(Arc::new(RwLock::new(streamlet)));
        if result.is_err() { return Err(ParserErrorCode::AnalysisCodeStructureFail(String::from(result.err().unwrap()))); }
    }

    return Ok(());
}

fn convert_type_str_to_type(type_exp: String) -> Result<DataType, ParserErrorCode> {
    let data_type ;
    if type_exp == String::from("") { data_type = DataType::UnknownType; }
    else if type_exp == String::from("int") { data_type = DataType::IntType; }
    else if type_exp == String::from("float") { data_type = DataType::FloatType; }
    else if type_exp == String::from("str") { data_type = DataType::StringType; }
    else if type_exp == String::from("bool") { data_type = DataType::BoolType; }
    else if type_exp == String::from("[int]") { data_type = DataType::ArrayType(Arc::new(RwLock::new(DataType::IntType))); }
    else if type_exp == String::from("[float]") { data_type = DataType::ArrayType(Arc::new(RwLock::new(DataType::FloatType))); }
    else if type_exp == String::from("[str]") { data_type = DataType::ArrayType(Arc::new(RwLock::new(DataType::StringType))); }
    else if type_exp == String::from("[bool]") { data_type = DataType::ArrayType(Arc::new(RwLock::new(DataType::BoolType))); }
    else if type_exp == String::from("clockdomain") { data_type = DataType::ClockDomainType; }
    else { unreachable!() }

    return Ok(data_type);
}

fn parse_const_assign(statement: Pairs<Rule>, scope: Arc<RwLock<Scope>>) -> Result<(), ParserErrorCode> {
    let mut id = String::from("");
    let mut type_exp = String::from("");
    let mut exp = String::from("");
    for element in statement.clone().into_iter() {
        match element.as_rule() {
            Rule::ID => { id = element.as_str().to_string() },
            Rule::TypeIndicatorBasicType => { type_exp = element.as_str().to_string() },
            Rule::TypeIndicatorBasicArrayType => { type_exp = element.as_str().to_string() },
            Rule::Exp => { exp = element.as_str().to_string() },
            _ => { unreachable!() },
        }
    }
    {
        let result = convert_type_str_to_type(type_exp);
        if result.is_err() { return Err(result.err().unwrap()); }
        let type_result = result.ok().unwrap();
        let result = scope.write().unwrap().new_variable(id.clone(), type_result.clone(), exp.clone());
        if result.is_err() { return Err(AnalysisCodeStructureFail(format!("{}", String::from(result.err().unwrap())))); }
    }
    return Ok(());
}

fn parse_const_declare(statement: Pairs<Rule>, scope: Arc<RwLock<Scope>>) -> Result<(), ParserErrorCode> {
    let mut id = String::from("");
    let mut type_exp = String::from("");
    for element in statement.clone().into_iter() {
        match element.as_rule() {
            Rule::ID => { id = element.as_str().to_string() },
            Rule::TypeIndicatorBasicType => { type_exp = element.as_str().to_string() },
            Rule::TypeIndicatorBasicArrayType => { type_exp = element.as_str().to_string() },
            _ => { unreachable!() },
        }
    }
    {
        let result = convert_type_str_to_type(type_exp);
        if result.is_err() { return Err(result.err().unwrap()); }
        let result = result.ok().unwrap();
        if result == DataType::UnknownType {
            return Err(AnalysisCodeStructureFail(format!("missing type indicator in the declaring const statement")));
        }

        use rand::{thread_rng, Rng};
        use rand::distributions::Alphanumeric;
        let rand_string: String = thread_rng().sample_iter(&Alphanumeric).take(30).map(char::from).collect();
        let clock_domain_name = format!("clockdomain_{}", rand_string);
        let cd_var = Variable::new_with_value(id.clone(), DataType::ClockDomainType, VariableValue::ClockDomain(ClockDomainValue::ClockDomain(clock_domain_name)));
        let result = scope.write().unwrap().with_variable(Arc::new(RwLock::new(cd_var)));
        if result.is_err() { return Err(AnalysisCodeStructureFail(format!("{}", String::from(result.err().unwrap())))); }
    }
    return Ok(());
}

fn parse_type_assign(statement: Pairs<Rule>, scope: Arc<RwLock<Scope>>) -> Result<(), ParserErrorCode> {
    let mut id = String::from("");
    let mut docu: Option<String> = None;
    for element in statement.clone().into_iter() {
        match element.as_rule() {
            Rule::DOCUMENT => {
                //currently we don't support document on type
                docu = Some(element.as_str().to_string());
                todo!("currently we don't support document on type");
            }
            Rule::ID => { id = element.clone().as_str().to_string() },
            Rule::LogicalType => {
                let result = get_logical_type(element.clone().into_inner(), id.clone(),scope.clone());
                if result.is_err() { return Err(result.err().unwrap()); }
                let mut logical_data_type = TypeAlias::new(id.clone(), inferred!(infer_logical_data_type!(), Arc::new(RwLock::new(result.ok().unwrap().clone()))));
                logical_data_type.set_document(docu.clone());
                let result = scope.write().unwrap().with_type_alias(Arc::new(RwLock::new(logical_data_type)));
                if result.is_err() { return Err(ParserErrorCode::AnalysisCodeStructureFail(format!("{}-{:?}", String::from(result.err().unwrap()), element.clone().as_span()))); }
                //let result = scope.write().unwrap().new_logical_data_type(id.clone(), result.ok().unwrap().clone());
                //if result.is_err() { return Err(ParserErrorCode::AnalysisCodeStructureFail(format!("{}-{:?}", String::from(result.err().unwrap()), element.clone().as_span()))); }
            },
            _ => { unreachable!() },
        }
    }
    return Ok(());
}

fn parse_type_declare(statement: Pairs<Rule>, scope: Arc<RwLock<Scope>>) -> Result<(), ParserErrorCode> {
    for element in statement.clone().into_iter() {
        match element.as_rule() {
            Rule::LogicalType => {
                let mut output = LogicalDataType::DataNull;
                let mut name = String::from("");
                for e in element.clone().into_inner() {
                    match e.clone().as_rule() {
                        Rule::LogicalGroupType => {
                            let group_type_result = get_logical_group(e.into_inner());
                            if group_type_result.is_err() { return Err(group_type_result.err().unwrap()); }
                            let group_type_result = group_type_result.ok().unwrap();
                            name = group_type_result.get_name();
                            output = LogicalDataType::DataGroupType(name.clone(), Arc::new(RwLock::new(group_type_result.clone())));
                        },
                        Rule::LogicalUnionType => {
                            let union_type_result = get_logical_union(e.into_inner());
                            if union_type_result.is_err() { return Err(union_type_result.err().unwrap()); }
                            let union_type_result = union_type_result.ok().unwrap();
                            name = union_type_result.get_name();
                            output = LogicalDataType::DataUnionType(name.clone(), Arc::new(RwLock::new(union_type_result.clone())));
                        },
                        _ => { return Err(ParserErrorCode::AnalysisCodeStructureFail(format!("only Group and Union are allowed here. -{:?}", e.clone().as_span()))) },
                    }
                }
                let result = scope.write().unwrap().new_logical_data_type(name.clone(), output);
                if result.is_err() { return Err(ParserErrorCode::AnalysisCodeStructureFail(String::from(result.err().unwrap()))); }
            },
            _ => { unreachable!() },
        }
    }
    return Ok(());
}

fn get_logical_type(exp: Pairs<Rule>, id: String, scope: Arc<RwLock<Scope>>) -> Result<LogicalDataType, ParserErrorCode> {
    let mut output = LogicalDataType::DummyLogicalData;
    for element in exp.into_iter() {
        match element.as_rule() {
            Rule::LogicalExternalType => {
                let mut exps = vec![String::from(""), String::from("")];
                let mut count = 0;
                for e in element.into_inner() {
                    match e.as_rule() {
                        Rule::ID => {
                            exps[count] = e.as_str().to_string();
                            count = count + 1;
                        },
                        _ => { unreachable!() },
                    }
                }
                output = LogicalDataType::ExternalLogicalDataType(exps[0].clone(), exps[1].clone());
            },
            Rule::LogicalNullType => {
                output = LogicalDataType::DataNull;
            },
            Rule::LogicalBitType => {
                let mut exp = String::from("");
                for e in element.into_inner() {
                    match e.as_rule() {
                        Rule::Exp => {
                            exp = e.as_str().to_string();
                        },
                        _ => { unreachable!() },
                    }
                }
                let logical_bit = LogicalBit::new(id.clone(), exp);
                output = LogicalDataType::DataBitType(logical_bit);
            },
            Rule::LogicalStreamType => {
                let mut flag_logical_type = false;
                let mut flag_dimension = false;
                let mut flag_user = false;
                let mut flag_throughput = false;
                let mut flag_sync = false;
                let mut flag_complexity = false;
                let mut flag_direction = false;
                let mut flag_keep = false;

                let mut logical_stream = LogicalStream::new(id.clone(), inferred!(infer_logical_data_type!(), Arc::new(RwLock::new(LogicalDataType::DataNull))));
                for e in element.into_inner() {
                    match e.clone().as_rule() {
                        Rule::LogicalType => {
                            if !flag_logical_type { flag_logical_type = true; }
                            else { return Err(ParserErrorCode::ParserError(format!("stream logical type override"))); }

                            let result = get_logical_type(e.into_inner(), String::from(""), scope.clone());
                            if result.is_err() { return result; }
                            logical_stream.set_data_type(inferred!(infer_logical_data_type!(), Arc::new(RwLock::new(result.ok().unwrap()))));
                        },
                        Rule::StreamPropertyDimension => {
                            if !flag_dimension { flag_dimension = true; }
                            else { return Err(ParserErrorCode::ParserError(format!("stream dimension override"))); }

                            let mut exp = String::from("");
                            for e in e.clone().into_inner() {
                                match e.as_rule() {
                                    Rule::Exp => {
                                        exp = e.as_str().to_string();
                                    },
                                    _ => { unreachable!() },
                                }
                            }
                            logical_stream.set_dimension(Variable::new(String::from("$dimension$"), DataType::IntType, exp));
                        },
                        Rule::StreamPropertyUserType => {
                            if !flag_user { flag_user = true; }
                            else { return Err(ParserErrorCode::ParserError(format!("stream user type override"))); }

                            for e in e.clone().into_inner() {
                                match e.as_rule() {
                                    Rule::LogicalType => {
                                        let result = get_logical_type(e.into_inner(), String::from(""), scope.clone());
                                        if result.is_err() { return result; }
                                        logical_stream.set_user_type(inferred!(infer_logical_data_type!(), Arc::new(RwLock::new(result.ok().unwrap()))));
                                    },
                                    _ => { unreachable!() },
                                }
                            }
                        },
                        Rule::StreamPropertyThroughput => {
                            if !flag_throughput { flag_throughput = true; }
                            else { return Err(ParserErrorCode::ParserError(format!("stream throughput override"))); }

                            let mut exp = String::from("");
                            for e in e.clone().into_inner() {
                                match e.as_rule() {
                                    Rule::Exp => {
                                        exp = e.as_str().to_string();
                                    },
                                    _ => { unreachable!() },
                                }
                            }
                            logical_stream.set_throughput(Variable::new(String::from("$throughput$"), DataType::FloatType, exp));
                        },
                        Rule::StreamPropertySynchronicity => {
                            if !flag_sync { flag_sync = true; }
                            else { return Err(ParserErrorCode::ParserError(format!("stream synchronicity override"))); }

                            let mut exp = String::from("");
                            for e in e.clone().into_inner() {
                                match e.as_rule() {
                                    Rule::Exp => {
                                        exp = e.as_str().to_string();
                                    },
                                    _ => { unreachable!() },
                                }
                            }
                            let sync = LogicalStreamSynchronicity::from(exp.clone());
                            logical_stream.set_synchronicity(sync);
                        },
                        Rule::StreamPropertyComplexity => {
                            if !flag_complexity { flag_complexity = true; }
                            else { return Err(ParserErrorCode::ParserError(format!("stream complexity override"))); }

                            let mut exp = String::from("");
                            for e in e.clone().into_inner() {
                                match e.as_rule() {
                                    Rule::Exp => {
                                        exp = e.as_str().to_string();
                                    },
                                    _ => { unreachable!() },
                                }
                            }
                            logical_stream.set_complexity(Variable::new(String::from("$complexity$"), DataType::IntType, exp));
                        },
                        Rule::StreamPropertyDirection => {
                            if !flag_direction { flag_direction = true; }
                            else { return Err(ParserErrorCode::ParserError(format!("stream direction override"))); }

                            let mut exp = String::from("");
                            for e in e.clone().into_inner() {
                                match e.as_rule() {
                                    Rule::Exp => {
                                        exp = e.as_str().to_string();
                                    },
                                    _ => { unreachable!() },
                                }
                            }
                            let dir = LogicalStreamDirection::from(exp.clone());
                            logical_stream.set_direction(dir);
                        },
                        Rule::StreamPropertyKeep => {
                            if !flag_keep { flag_keep = true; }
                            else { return Err(ParserErrorCode::ParserError(format!("stream keep override"))); }

                            let mut exp = String::from("");
                            for e in e.clone().into_inner() {
                                match e.as_rule() {
                                    Rule::Exp => {
                                        exp = e.as_str().to_string();
                                    },
                                    _ => { unreachable!() },
                                }
                            }
                            logical_stream.set_keep(Variable::new(String::from("$keep$"), DataType::BoolType, exp));
                        },
                        _ => { unreachable!() },
                    }
                }
                output = LogicalDataType::DataStreamType(id.clone(), Arc::new(RwLock::new(logical_stream)));
            },
            Rule::LogicalGroupType => {
                let group_type_result = get_logical_group(element.into_inner());
                if group_type_result.is_err() { return Err(group_type_result.err().unwrap()); }
                output = LogicalDataType::DataGroupType(id.clone(), Arc::new(RwLock::new(group_type_result.ok().unwrap())));
            },
            Rule::LogicalUnionType => {
                let union_type_result = get_logical_union(element.into_inner());
                if union_type_result.is_err() { return Err(union_type_result.err().unwrap()); }
                output = LogicalDataType::DataUnionType(id.clone(), Arc::new(RwLock::new(union_type_result.ok().unwrap())));
            },
            Rule::LogicalUserDefinedType => {
                let mut exp = String::from("");
                for e in element.into_inner() {
                    match e.as_rule() {
                        Rule::ID => {
                            exp = e.as_str().to_string();
                        },
                        _ => { unreachable!() },
                    }
                }
                output = LogicalDataType::DataUserDefinedVarType(exp);
            },
            _ => { unreachable!() },
        }
    }

    return Ok(output);
}

fn get_logical_group(items: Pairs<Rule>) -> Result<LogicalGroup, ParserErrorCode> {
    let mut group_type = LogicalGroup::new(String::from(""));
    for item in items.into_iter() {
        match item.as_rule() {
            Rule::ID => {
                group_type.set_name(item.as_str().to_string().clone());
            },
            Rule::SubItemItem => {
                let result = parse_type_assign(item.into_inner(), group_type.get_scope());
                if result.is_err() { return Err(result.err().unwrap()); }
            },
            Rule::SubItemDeclareConst => {
                let result = parse_const_assign(item.into_inner(), group_type.get_scope());
                if result.is_err() { return Err(result.err().unwrap()); }
            },
            Rule::Function => {
                let result = parse_function(item.into_inner(), group_type.get_scope());
                if result.is_err() { return Err(result.err().unwrap()); }
            },
            _ => { unreachable!() },
        }
    }
    return Ok(group_type);
}

fn get_logical_union(items: Pairs<Rule>) -> Result<LogicalUnion, ParserErrorCode> {
    let mut union_type = LogicalUnion::new(String::from(""));
    for item in items.into_iter() {
        match item.as_rule() {
            Rule::ID => {
                union_type.set_name(item.as_str().to_string().clone());
            },
            Rule::SubItemItem => {
                let result =  parse_type_assign(item.into_inner(), union_type.get_scope());
                if result.is_err() { return Err(result.err().unwrap()); }
            },
            Rule::SubItemDeclareConst => {
                let result =  parse_const_assign(item.into_inner(), union_type.get_scope());
                if result.is_err() { return Err(result.err().unwrap()); }
            },
            Rule::Function => {
                let result = parse_function(item.into_inner(), union_type.get_scope());
                if result.is_err() { return Err(result.err().unwrap()); }
            },
            _ => { unreachable!() },
        }
    }
    return Ok(union_type);
}
