use ::{evaluation_streamlet, evaluation_type};
use evaluation_implement;
use tydi_lang_raw_ast::implement::ImplementType;
use tydi_lang_raw_ast::scope::{PrettyPrint, StreamletType};

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
    else { unreachable!() }

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
                match errors[index].1.clone() {
                    Ok(_) => { println!("{}:no error", errors[index].0) }
                    Err(e) => println!("{}: error: {}", errors[index].0, String::from(e))
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
        unreachable!()
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
                    let result = evaluation_type::infer_type_alias(type_alias, package_scope.clone(), project.clone());
                    if result.is_err() { println!("{}", String::from(result.err().unwrap()));return; }
                }
            }
            {
                println!("{}", project.read().unwrap().pretty_print(0, false));
            }
        }
        Err(errors) => {
            for index in 0..errors.len() {
                match errors[index].1.clone() {
                    Ok(_) => { println!("{}:no error", errors[index].0) }
                    Err(e) => println!("{}: error: {}", errors[index].0, String::from(e))
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
        unreachable!()
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
                    let streamlet_type = streamlet.read().unwrap().get_type();
                    match streamlet_type {
                        StreamletType::NormalStreamlet => {
                            let result = evaluation_streamlet::infer_streamlet(streamlet, vec![], package_scope.clone(), project.clone());
                            if result.is_err() { println!("{}", String::from(result.err().unwrap()));return; }
                        },
                        StreamletType::TemplateStreamlet(_) => {
                            //don't evaluate template
                        },
                        _ => unreachable!()
                    }

                }
            }
            {
                println!("{}", project.read().unwrap().pretty_print(0, false));
            }
        }
        Err(errors) => {
            for index in 0..errors.len() {
                match errors[index].1.clone() {
                    Ok(_) => { println!("{}:no error", errors[index].0) }
                    Err(e) => println!("{}: error: {}", errors[index].0, String::from(e))
                }
            }
        }
    }
    assert!(true);
}

#[test]
fn evaluate_implement() {
    use std::env;
    let base_dir = env::current_dir().expect("not found path");
    let paths:Vec<String>;
    if base_dir.ends_with("tydi-lang") {
        paths = vec![String::from("./tydi_lang_parser/tydi_source/implement_eval_0.td"),
                     String::from("./tydi_lang_parser/tydi_source/implement_eval_1.td")];
    }
    else if base_dir.ends_with("tydi_lang_parser") {
        paths = vec![String::from("./tydi_source/implement_eval_0.td"),
                     String::from("./tydi_source/implement_eval_1.td")];
    }
    else {
        unreachable!()
    }

    let result = parse_multi_files_mt(String::from("test_project"), paths.clone(), None);
    match result {
        Ok(project) => {
            {
                println!("{}", project.read().unwrap().pretty_print(0, false));
            }
            {
                let package = project.read().unwrap().packages["implement_eval_0"].clone();
                let package_scope = package.read().unwrap().get_scope().clone();
                let package_implements = package_scope.read().unwrap().implements.clone();
                for (_, implement) in package_implements {
                    let implement_type = implement.read().unwrap().get_type();
                    match implement_type.clone() {
                        ImplementType::NormalImplement => {
                            let result = evaluation_implement::infer_implement(implement, vec![], package_scope.clone(), project.clone());
                            if result.is_err() { println!("{}", String::from(result.err().unwrap()));return; }
                        }
                        ImplementType::TemplateImplement(_) => {
                            //don't evaluate template
                        }
                        _ => unreachable!()
                    }
                }
            }
            {
                println!("{}", project.read().unwrap().pretty_print(0, false));
            }
        }
        Err(errors) => {
            for index in 0..errors.len() {
                match errors[index].1.clone() {
                    Ok(_) => { println!("{}:no error", errors[index].0) }
                    Err(e) => println!("{}: error: {}", errors[index].0, String::from(e))
                }
            }
        }
    }
    assert!(true);
}


#[test]
fn evaluate_implement_mt() {
    use std::env;
    let base_dir = env::current_dir().expect("not found path");
    let paths:Vec<String>;
    if base_dir.ends_with("tydi-lang") {
        paths = vec![String::from("./tydi_lang_parser/tydi_source/implement_eval_0.td"),
                     String::from("./tydi_lang_parser/tydi_source/implement_eval_1.td")];
    }
    else if base_dir.ends_with("tydi_lang_parser") {
        paths = vec![String::from("./tydi_source/implement_eval_0.td"),
                     String::from("./tydi_source/implement_eval_1.td")];
    }
    else {
        unreachable!()
    }

    let result = parse_multi_files_mt(String::from("test_project"), paths.clone(), None);
    match result {
        Ok(project) => {
            {
                println!("{}", project.read().unwrap().pretty_print(0, false));
            }
            let result = crate::evaluation::evaluation_project_mt(project.clone(), None);
            if result.is_err() {
                for single_err in result.err().unwrap() {
                    println!("{}", String::from(single_err));
                }
                return;
            }
            {
                println!("{}", project.read().unwrap().pretty_print(0, false));
            }
        }
        Err(errors) => {
            for index in 0..errors.len() {
                match errors[index].1.clone() {
                    Ok(_) => { println!("{}:no error", errors[index].0) }
                    Err(e) => println!("{}: error: {}", errors[index].0, String::from(e))
                }
            }
        }
    }
    assert!(true);
}

#[test]
fn evaluate_implement_mt_benchmark() {
    use std::env;
    let base_dir = env::current_dir().expect("not found path");
    let paths:Vec<String>;
    if base_dir.ends_with("tydi-lang") {
        paths = vec![String::from("./tydi_lang_parser/tydi_source/implement_eval_0.td"),
                     String::from("./tydi_lang_parser/tydi_source/implement_eval_1.td")];
    }
    else if base_dir.ends_with("tydi_lang_parser") {
        paths = vec![String::from("./tydi_source/implement_eval_0.td"),
                     String::from("./tydi_source/implement_eval_1.td")];
    }
    else {
        unreachable!()
    }

    for worker in 1..num_cpus::get() {

        use std::time::Instant;
        let start = Instant::now();
        let result = parse_multi_files_mt(String::from("test_project"), paths.clone(), Some(worker));
        let stop = Instant::now();
        let parser_time = stop - start;
        let evaluation_time;
            match result {
            Ok(project) => {
                let start = Instant::now();
                let result = crate::evaluation::evaluation_project_mt(project.clone(), Some(worker));
                let stop = Instant::now();
                evaluation_time = stop - start;
                if result.is_err() {
                    for single_err in result.err().unwrap() {
                        println!("{}", String::from(single_err));
                    }
                    return;
                }
            }
            Err(errors) => {
                for index in 0..errors.len() {
                    match errors[index].1.clone() {
                        Ok(_) => { println!("{}:no error", errors[index].0) }
                        Err(e) => println!("{}: error: {}", errors[index].0, String::from(e))
                    }
                }
                return;
            }
        }
        println!("worker = {}: {}(parser) + {}(evaluation_time) = {}(total)", worker, parser_time.as_micros(), evaluation_time.as_micros(), (parser_time+evaluation_time).as_micros());
    }

    assert!(true);
}
