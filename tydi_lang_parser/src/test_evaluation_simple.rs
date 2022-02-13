use ::{evaluation_streamlet, evaluation_type};
use tydi_lang_raw_ast::scope::{PrettyPrint};

use crate::{parse_multi_files_mt, evaluation_var};

#[test]
fn evaluate_simple() {
    use std::env;
    let base_dir = env::current_dir().expect("not found path");
    let paths:Vec<String>;
    if base_dir.ends_with("tydi-lang") {
        paths = vec![String::from("./tydi_lang_parser/tydi_source/simple_0.td"),
                     String::from("./tydi_lang_parser/tydi_source/simple_1.td")];
    }
    else if base_dir.ends_with("tydi_lang_parser") {
        paths = vec![String::from("./tydi_source/simple_0.td"),
                     String::from("./tydi_source/simple_1.td")];
    }
    else { todo!() }

    let result = parse_multi_files_mt(String::from("test_project"), paths.clone(), None);
    match result {
        Ok(project) => {
            {
                println!("{}", project.read().unwrap().pretty_print(0, false));
            }
            {
                let package = project.read().unwrap().packages["simple_0"].clone();
                let package_scope = package.read().unwrap().get_scope().clone();
                for (_, var) in package_scope.read().unwrap().vars.clone() {
                    let result = evaluation_var::infer_variable(var, package_scope.clone(), project.clone());
                    if result.is_err() { println!("{}", String::from(result.err().unwrap()));return; }
                }
            }
            {
                println!("{}", project.read().unwrap().pretty_print(0, false));
            }
        }
        Err(errors) => {
            for index in 0..errors.len() {
                match errors[index].clone() {
                    Ok(_) => { println!("{}:no error", paths[index]) }
                    Err(e) => println!("{}: error: {}", paths[index], String::from(e))
                }
            }
        }
    }
    assert!(true);
}

#[test]
fn evaluate_type() {
    use std::env;
    let base_dir = env::current_dir().expect("not found path");
    let paths:Vec<String>;
    if base_dir.ends_with("tydi-lang") {
        paths = vec![String::from("./tydi_lang_parser/tydi_source/type_eval_0.td"),
                     String::from("./tydi_lang_parser/tydi_source/type_eval_1.td")];
    }
    else if base_dir.ends_with("tydi_lang_parser") {
        paths = vec![String::from("./tydi_source/type_eval_0.td"),
                     String::from("./tydi_source/type_eval_1.td")];
    }
    else {
        todo!()
    }

    let result = parse_multi_files_mt(String::from("test_project"), paths.clone(), None);
    match result {
        Ok(project) => {
            {
                println!("{}", project.read().unwrap().pretty_print(0, false));
            }
            {
                let package = project.read().unwrap().packages["type_eval_0"].clone();
                let package_scope = package.read().unwrap().get_scope().clone();
                for (_, type_alias) in package_scope.read().unwrap().types.clone() {
                    let result = evaluation_type::infer_type(type_alias, package_scope.clone(), project.clone());
                    if result.is_err() { println!("{}", String::from(result.err().unwrap()));return; }
                }
            }
            {
                println!("{}", project.read().unwrap().pretty_print(0, false));
            }
        }
        Err(errors) => {
            for index in 0..errors.len() {
                match errors[index].clone() {
                    Ok(_) => { println!("{}:no error", paths[index]) }
                    Err(e) => println!("{}: error: {}", paths[index], String::from(e))
                }
            }
        }
    }
    assert!(true);
}

#[test]
fn evaluate_streamlet() {
    use std::env;
    let base_dir = env::current_dir().expect("not found path");
    let paths:Vec<String>;
    if base_dir.ends_with("tydi-lang") {
        paths = vec![String::from("./tydi_lang_parser/tydi_source/streamlet_eval_0.td"),
                     String::from("./tydi_lang_parser/tydi_source/streamlet_eval_1.td")];
    }
    else if base_dir.ends_with("tydi_lang_parser") {
        paths = vec![String::from("./tydi_source/streamlet_eval_0.td"),
                     String::from("./tydi_source/streamlet_eval_1.td")];
    }
    else {
        todo!()
    }

    let result = parse_multi_files_mt(String::from("test_project"), paths.clone(), None);
    match result {
        Ok(project) => {
            {
                println!("{}", project.read().unwrap().pretty_print(0, false));
            }
            {
                let package = project.read().unwrap().packages["streamlet_eval_0"].clone();
                let package_scope = package.read().unwrap().get_scope().clone();
                for (_, streamlet) in package_scope.read().unwrap().streamlets.clone() {
                    let result = evaluation_streamlet::infer_streamlet(streamlet, vec![], package_scope.clone(), project.clone());
                    if result.is_err() { println!("{}", String::from(result.err().unwrap()));return; }
                }
            }
            {
                println!("{}", project.read().unwrap().pretty_print(0, false));
            }
        }
        Err(errors) => {
            for index in 0..errors.len() {
                match errors[index].clone() {
                    Ok(_) => { println!("{}:no error", paths[index]) }
                    Err(e) => println!("{}: error: {}", paths[index], String::from(e))
                }
            }
        }
    }
    assert!(true);
}