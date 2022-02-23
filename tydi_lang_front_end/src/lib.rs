mod test_tydi_ir;

extern crate chrono;
extern crate tydi_lang_parser;
extern crate tydi_lang_raw_ast;

use chrono::{Datelike, Timelike};
use std::fs;
use std::io::Write;
use std::sync::{Arc, RwLock};
use tydi_lang_raw_ast::project_arch::Project;
use tydi_lang_parser::evaluation;
use tydi_lang_raw_ast::util::PrettyPrint;

#[test]
pub fn test() {
    let base_dir = std::env::current_dir().expect("not found path");
    let paths:Vec<String>;
    if base_dir.ends_with("tydi-lang") {
        paths = vec![String::from("./tydi_lang_front_end/tydi_source/tydi_ir.td")];
    }
    else if base_dir.ends_with("tydi_lang_front_end") {
        paths = vec![String::from("./tydi_source/tydi_ir.td")];
    }
    else {
        unreachable!()
    }

    let result = tydi_frontend_compile(None, paths, Some(String::from("./output")), true, true, None);
    if result.is_err() {
        println!("{}",result.err().unwrap());
        assert!(false);
    }

}

pub fn tydi_frontend_compile(project_name: Option<String>, tydi_source_path: Vec<String>, output_path: Option<String>, flag_compile_streamlet: bool, flag_compile_implement: bool, worker: Option<usize>) -> Result<Arc<RwLock<Project>>, String> {
    //get project name
    let real_project_name;
    match project_name {
        None => {
            let now = chrono::Local::now();
            real_project_name = format!("project_{}_{:02}_{:02}_{:02}_{:02}_{:02}", now.year(), now.month(), now.day(), now.hour(), now.minute(), now.second());
        }
        Some(name) => { real_project_name = name; }
    }

    //create output path
    let real_output_path;
    match output_path {
        None => {
            real_output_path = format!("./{}", real_project_name.clone());
        }
        Some(path) => { real_output_path = path; }
    }
    let result = fs::create_dir(real_output_path.clone());
    if result.is_err() { return Err(format!("failed to create output folder({})", real_output_path.clone())); }

    //parse front end
    let result = tydi_lang_parser::parse_multi_files_mt(real_project_name.clone(), tydi_source_path.clone(), worker);
    let project_architecture;
    match result {
        Ok(project) => {
            project_architecture = project;
        }
        Err(errors) => {
            let mut err_msg = String::from("");
            for index in 0..errors.len() {
                match errors[index].1.clone() {
                    Ok(_) => { err_msg.push_str(&format!("{}: no error\n", errors[index].0)) }
                    Err(e) => { err_msg.push_str(&format!("{}: error: {}\n", errors[index].0, String::from(e))); }
                }
            }
            return Err(err_msg);
        }
    }

    {
        let parser_output_file_path = format!("{}/parser_output.txt", real_output_path.clone());
        let mut file = fs::File::create(&parser_output_file_path).unwrap();
        let content = project_architecture.read().unwrap().pretty_print(0,false);
        file.write_all(content.as_bytes()).unwrap();
    }

    //evaluation project
    let result = evaluation::evaluation_project_mt(project_architecture.clone(), flag_compile_streamlet, flag_compile_implement, worker);
    if result.is_err() {
        let mut err_msg = String::from("");
        for error_code in result.err().unwrap() {
            err_msg.push_str(&format!("{}\n", String::from(error_code)));
        }
        return Err(err_msg);
    }

    {
        let evaluation_output_file_path = format!("{}/evaluation_output.txt", real_output_path.clone());
        let mut file = fs::File::create(&evaluation_output_file_path).unwrap();
        let content = project_architecture.read().unwrap().pretty_print(0,false);
        file.write_all(content.as_bytes()).unwrap();
    }

    //generation
    if flag_compile_streamlet {

    }

    if flag_compile_implement {

    }

    return Ok(project_architecture);
}