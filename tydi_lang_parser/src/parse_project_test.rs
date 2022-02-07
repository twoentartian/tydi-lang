use crate::{parse_to_memory,parse_multi_files_mt};


#[test]
fn parse_test0() {
    use tydi_lang_raw_ast::util::PrettyPrint;
    use std::env;
    let base_dir = env::current_dir().expect("not found path");
    println!("The base dir: {}", base_dir.to_str().expect(""));

    let result = parse_to_memory(String::from("./tydi_source/test0.td"));
    match result {
        Ok(package) => {
            println!("{}", package.pretty_print(0, false));
        }
        Err(e) => { println!("{}", String::from(e))}
    }
}

#[test]
fn parse_test0_mt() {
    use tydi_lang_raw_ast::util::PrettyPrint;
    use std::env;
    let base_dir = env::current_dir().expect("not found path");
    println!("The base dir: {}", base_dir.to_str().expect(""));

    let paths = vec![String::from("./tydi_source/test0.td"), String::from("./tydi_source/test1.td")];
    let result = parse_multi_files_mt(String::from("test_project"), paths.clone(), None);

    match result {
        Ok(project) => {
            println!("{}", project.read().unwrap().pretty_print(0, false));
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
}