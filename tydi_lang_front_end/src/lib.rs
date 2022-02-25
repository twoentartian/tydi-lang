mod test_tydi_ir;

extern crate chrono;
extern crate tydi_lang_parser;
extern crate tydi_lang_raw_ast;

use std::fs;
use std::fs::File;
use std::path::Path;
use std::io::Write;
use std::sync::{Arc, RwLock};
use chrono::{Datelike, Timelike};
use tydi_lang_raw_ast::project_arch::Project;
use tydi_lang_parser::evaluation;
use tydi_lang_raw_ast::util::PrettyPrint;

pub fn tydi_frontend_compile(project_name: Option<String>, tydi_source_path: Vec<String>, output_path: Option<String>, worker: Option<usize>) -> Result<Arc<RwLock<Project>>, String> {
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

    if !Path::new(&real_output_path).exists() {
        let result = fs::create_dir(real_output_path.clone());
        if result.is_err() { return Err(format!("failed to create output folder({})", real_output_path.clone())); }
    }

    //parse front end
    let result = tydi_lang_parser::parse_multi_files_mt(real_project_name.clone(), tydi_source_path.clone(), worker);
    let project_architecture;
    let ast_trees;
    match result {
        Ok((project, asts)) => {
            project_architecture = project;
            ast_trees = asts;
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
        let output_ast_path = format!("{}/0_ast", real_output_path.clone());
        if !Path::new(&output_ast_path).exists() {
            let result = fs::create_dir(output_ast_path.clone());
            if result.is_err() { return Err(format!("failed to create output ast folder({})", output_ast_path.clone())); }
        }

        for (src_file_path, ast)  in ast_trees {
            let file_name = Path::new(&src_file_path).file_name().unwrap().to_str().unwrap();
            let output_file_path = format!("{}/{}.ast.txt", output_ast_path.clone(), file_name);
            let mut file = fs::File::create(&output_file_path).unwrap();
            file.write_all(ast.as_bytes()).unwrap();
        }

        let parser_output_file_path = format!("{}/1_parser_output.txt", real_output_path.clone());
        let mut file = fs::File::create(&parser_output_file_path).unwrap();
        let content = project_architecture.read().unwrap().pretty_print(0,false);
        file.write_all(content.as_bytes()).unwrap();
    }
    {
        let parser_output_file_path = format!("{}/1_parser_output.txt", real_output_path.clone());
        let mut file = fs::File::create(&parser_output_file_path).unwrap();
        let content = project_architecture.read().unwrap().pretty_print(0,false);
        file.write_all(content.as_bytes()).unwrap();
    }

    //evaluation project
    let result = evaluation::evaluation_project_mt(project_architecture.clone(), true, true, worker);
    if result.is_err() {
        let mut err_msg = String::from("");
        for error_code in result.err().unwrap() {
            err_msg.push_str(&format!("{}\n", String::from(error_code)));
        }
        return Err(err_msg);
    }

    {
        let evaluation_output_file_path = format!("{}/2_evaluation_output.txt", real_output_path.clone());
        let mut file = fs::File::create(&evaluation_output_file_path).unwrap();
        let content = project_architecture.read().unwrap().pretty_print(0,false);
        file.write_all(content.as_bytes()).unwrap();
    }

    //IR generation
    let output_til_path = format!("{}/3_til", real_output_path.clone());
    if !Path::new(&output_til_path).exists() {
        let result = fs::create_dir(output_til_path.clone());
        if result.is_err() { return Err(format!("failed to create output til folder({})", output_til_path.clone())); }
    }

    let output_til = project_architecture.read().unwrap().to_tydi_il(real_project_name.clone());
    for (file_name, file_content) in output_til {
        let output_til_file_path = format!("{}/{}.til", output_til_path.clone(), file_name);
        let result = File::create(output_til_file_path.clone());
        if result.is_err() { return Err(result.err().unwrap().to_string()); }
        let mut file = result.ok().unwrap();
        let result = file.write_all(file_content.as_bytes());
        if result.is_err() { return Err(result.err().unwrap().to_string()); }
    }

    return Ok(project_architecture);
}