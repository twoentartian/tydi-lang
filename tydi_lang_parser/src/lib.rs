extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::fs;
use std::sync::{Arc, RwLock};
use ParserErrorCode::AnalysisCodeStructureFail;
use pest::Parser;
use pest::iterators::{Pairs};
use tydi_lang_raw_ast::bit_null_type::LogicalBit;
use tydi_lang_raw_ast::data_type::DataType;
use tydi_lang_raw_ast::inferable::{NewInferable, Inferable};

extern crate tydi_lang_raw_ast;
extern crate proc_macro;

use tydi_lang_raw_ast::{not_inferred, project_arch};
use tydi_lang_raw_ast::scope::*;
use tydi_lang_raw_ast::{inferred, infer_logical_data_type};
use tydi_lang_raw_ast::{};

mod lex_test;

#[derive(Parser)]
#[grammar = "tydi_lang_syntax.pest"]
pub struct TydiParser;

pub enum ParserErrorCode {
    ParserError(String),
    FileError(String),
    AnalysisCodeStructureFail(String),
}

impl From<ParserErrorCode> for String {
    fn from(e: ParserErrorCode) -> Self {
        return match e {
            ParserErrorCode::ParserError(s) => { format!("ParserError:{}", s) }
            ParserErrorCode::FileError(s) => { format!("FileError:{}", s) }
            ParserErrorCode::AnalysisCodeStructureFail(s) => { format!("AnalysisCodeStructureFail:{}", s) }
        }
    }
}

pub fn parse_to_memory(file_path: String) -> Result<project_arch::Package, ParserErrorCode> {
    let unparsed_file_result = fs::read_to_string(file_path.clone());
    if unparsed_file_result.is_err() { return Err(ParserErrorCode::FileError(unparsed_file_result.err().unwrap().to_string())); }
    let unparsed_file_content = unparsed_file_result.ok().unwrap();
    let tydi_ast_result = TydiParser::parse(Rule::Start, &unparsed_file_content);
    if tydi_ast_result.is_err() { return Err(ParserErrorCode::ParserError(tydi_ast_result.err().unwrap().to_string())); }
    let mut tydi_ast_result = tydi_ast_result.unwrap();

    let mut output_package = project_arch::Package::new(String::from("unknown_package"));

    let tydi_ast = tydi_ast_result.next().unwrap();
    match tydi_ast.as_rule() {
        Rule::Start => {
            let mut inner_rules = tydi_ast.into_inner();
            let result = parse_start(inner_rules, &mut output_package);
            if result.is_err() { return Err(result.err().unwrap()); }
        }
        _ => unreachable!(),
    }

    return Ok(output_package);
}


pub fn parse_start(start_elements: Pairs<Rule>, output_package: &mut project_arch::Package) -> Result<(), ParserErrorCode> {
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
                            let result = output_package.scope.write().unwrap().new_variable(format!("$package${}", element_name.clone()), DataType::PackageType, String::from(""));
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

pub fn parse_statement(statement: Pairs<Rule>, output_package: &mut project_arch::Package) -> Result<(), ParserErrorCode> {
    for element in statement.into_iter() {
        match element.as_rule() {
            Rule::StatementConstAssign => {
                let result = parse_const_assign(element.into_inner(), output_package.scope.clone());
                if result.is_err() { return result; }
            },
            Rule::StatementTypeAssign => {
                let result = parse_type_assign(element.into_inner(), output_package.scope.clone());
                if result.is_err() { return result; }
            },
            Rule::StatementDeclareLogicalDataType => {
                let result = parse_type_declare(element.into_inner(), output_package.scope.clone());
                if result.is_err() { return result; }
            },
            Rule::StatementDeclareStreamLet => {
                let result = parse_streamlet_declare(element.into_inner(), output_package.scope.clone());
                if result.is_err() { return result; }
            },
            _ => {
                let statement_name = element.as_str().to_string();
                println!("{}", statement_name);
                //unreachable!()
            },
        }
    }
    return Ok(());
}

pub fn parse_streamlet_declare(statement: Pairs<Rule>, scope: Arc<RwLock<Scope>>) -> Result<(), ParserErrorCode> {
    let mut streamlet = Streamlet::new(String::from(""), StreamletType::UnknownType);
    for element in statement.into_iter() {
        match element.as_rule() {
            Rule::ID => {
                streamlet.set_name(element.as_str().to_string());
            },
            Rule::Arg => {

            },
            Rule::StreamLetBodyStreamLetPort => {

            },
            Rule::StreamLetBodyStreamLetPortArray => {

            },
            Rule::StreamLetBodyDeclareConstInStreamlet => {

            },
            _ => { unreachable!() },
        }
    }
    scope.write().unwrap().with_streamlet(streamlet);
    return Ok(());
}

pub fn parse_const_assign(statement: Pairs<Rule>, scope: Arc<RwLock<Scope>>) -> Result<(), ParserErrorCode> {
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

    let mut data_type ;
    if type_exp == String::from("") { data_type = DataType::UnknownType; }
    else if type_exp == String::from("int") { data_type = DataType::IntType; }
    else if type_exp == String::from("float") { data_type = DataType::FloatType; }
    else if type_exp == String::from("str") { data_type = DataType::StringType; }
    else if type_exp == String::from("bool") { data_type = DataType::BoolType; }
    else if type_exp == String::from("[int]") { data_type = DataType::ArrayType(Arc::new(RwLock::new(DataType::IntType))); }
    else if type_exp == String::from("[float]") { data_type = DataType::ArrayType(Arc::new(RwLock::new(DataType::FloatType))); }
    else if type_exp == String::from("[str]") { data_type = DataType::ArrayType(Arc::new(RwLock::new(DataType::StringType))); }
    else if type_exp == String::from("[bool]") { data_type = DataType::ArrayType(Arc::new(RwLock::new(DataType::BoolType))); }
    else { unreachable!() }
    {
        let result = scope.write().unwrap().new_variable(id.clone(), data_type, exp.clone());
        if result.is_err() { return Err(ParserErrorCode::AnalysisCodeStructureFail(format!("{}", String::from(result.err().unwrap())))); }
    }
    return Ok(());
}

pub fn parse_type_assign(statement: Pairs<Rule>, scope: Arc<RwLock<Scope>>) -> Result<(), ParserErrorCode> {
    let mut id = String::from("");
    for element in statement.clone().into_iter() {
        match element.as_rule() {
            Rule::ID => { id = element.clone().as_str().to_string() },
            Rule::LogicalType => {
                let result = get_logical_type(element.clone().into_inner(), id.clone(),scope.clone());
                if result.is_err() { return Err(result.err().unwrap()); }
                let result = scope.write().unwrap().new_logical_data_type(id.clone(), result.ok().unwrap().clone());
                if result.is_err() { return Err(ParserErrorCode::AnalysisCodeStructureFail(format!("{}-{:?}", String::from(result.err().unwrap()), element.clone().as_span()))); }
            },
            _ => { unreachable!() },
        }
    }
    return Ok(());
}

pub fn parse_type_declare(statement: Pairs<Rule>, scope: Arc<RwLock<Scope>>) -> Result<(), ParserErrorCode> {
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

pub fn get_logical_type(exp: Pairs<Rule>, id: String, scope: Arc<RwLock<Scope>>) -> Result<LogicalDataType, ParserErrorCode> {
    let mut output = LogicalDataType::EmptyLogicalData;
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
                let mut logical_stream = LogicalStream::new(id.clone(), inferred!(infer_logical_data_type!(), Arc::new(RwLock::new(LogicalDataType::DataNull))));
                for e in element.into_inner() {
                    match e.clone().as_rule() {
                        Rule::LogicalType => {
                            let exp = e.as_str().to_string();
                            let result = get_logical_type(e.into_inner(), String::from(""), scope.clone());
                            if result.is_err() { return result; }
                            logical_stream.set_data_type(inferred!(infer_logical_data_type!(), Arc::new(RwLock::new(result.ok().unwrap()))));
                        },
                        Rule::StreamPropertyDimension => {
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
                            if sync == LogicalStreamSynchronicity::Unknown { return Err(AnalysisCodeStructureFail(format!("{} cannot be resolve as a valid stream synchronicity", exp.clone()))); }
                            logical_stream.set_synchronicity(sync);
                        },
                        Rule::StreamPropertyComplexity => {
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
                            if dir == LogicalStreamDirection::Unknown { return Err(AnalysisCodeStructureFail(format!("{} cannot be resolve as a valid stream direction", exp.clone()))); }
                            logical_stream.set_direction(dir);
                        },
                        Rule::StreamPropertyKeep => {
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

pub fn get_logical_group(items: Pairs<Rule>) -> Result<LogicalGroup, ParserErrorCode> {
    let mut group_type = LogicalGroup::new(String::from(""));
    for item in items.into_iter() {
        match item.as_rule() {
            Rule::ID => {
                group_type.set_name(item.as_str().to_string().clone());
            },
            Rule::SubItemItem => {
                let result =  parse_type_assign(item.into_inner(), group_type.get_scope());
                if result.is_err() { return Err(result.err().unwrap()); }
            },
            Rule::SubItemDeclareConst => {
                let result =  parse_const_assign(item.into_inner(), group_type.get_scope());
                if result.is_err() { return Err(result.err().unwrap()); }
            },
            _ => { unreachable!() },
        }
    }
    return Ok(group_type);
}

pub fn get_logical_union(items: Pairs<Rule>) -> Result<LogicalUnion, ParserErrorCode> {
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
            _ => { unreachable!() },
        }
    }
    return Ok(union_type);
}


#[test]
fn parse_test0() {
    use tydi_lang_raw_ast::util::PrettyPrint;
    use std::env;
    let base_dir = env::current_dir().expect("not found path");
    println!("The base dir: {}", base_dir.to_str().expect(""));

    let result = parse_to_memory(String::from("D:/git/tydi-lang/tydi_lang_parser/tydi_source/test0.td"));
    match result {
        Ok(package) => {
            println!("{}", package.pretty_print(0, false));
        }
        Err(e) => { println!("{}", String::from(e))}
    }
}