extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::fs;
use std::sync::{Arc, RwLock};
use pest::Parser;
use pest::iterators::{Pairs};
use tydi_lang_raw_ast::data_type::DataType;

extern crate tydi_lang_raw_ast;
extern crate proc_macro;

use tydi_lang_raw_ast::project_arch;
use tydi_lang_raw_ast::scope::Scope;

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
            parse_start(inner_rules, &mut output_package);

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
                            if result.is_err() { return Err(ParserErrorCode::AnalysisCodeStructureFail(String::from(result.err().unwrap()))) }
                        },
                        _ => { unreachable!() },
                    }
                }
            },
            Rule::StartElementAStatement => {
                let result = parse_statement(start_element.into_inner(), output_package);
                if result.is_err() { return Err(ParserErrorCode::AnalysisCodeStructureFail(String::from(result.err().unwrap()))) }
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
                if result.is_err() { return Err(ParserErrorCode::AnalysisCodeStructureFail(String::from(result.err().unwrap()))) }
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

pub fn parse_const_assign(statement: Pairs<Rule>, scope: Arc<RwLock<Scope>>) -> Result<(), ParserErrorCode> {
    let mut id = String::from("");
    let mut type_exp = String::from("");
    let mut exp = String::from("");
    for element in statement.into_iter() {
        match element.as_rule() {
            Rule::ID => { id = element.as_str().to_string() },
            Rule::TypeIndicatorBasicType => { type_exp = element.as_str().to_string() },
            Rule::TypeIndicatorBasicArrayType => { type_exp = element.as_str().to_string() },
            Rule::Exp => { exp = element.as_str().to_string() },
            _ => {
                let statement_name = element.as_str().to_string();
                println!("{}", statement_name);
                //unreachable!()
            },
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
        if result.is_err() { return Err(ParserErrorCode::AnalysisCodeStructureFail(String::from(result.err().unwrap()))) }
    }
    return Ok(());
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
        Err(e) => { println!("error: {}", String::from(e))}
    }
}