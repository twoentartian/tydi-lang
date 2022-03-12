use std::path::Path;
use tydi_lang_raw_ast::util::PrettyPrint;
#[allow(unused_imports)]
use crate::tydi_frontend_compile;

#[test]
pub fn test() {
    let base_dir = std::env::current_dir().expect("not found path");
    let paths:Vec<String>;
    if base_dir.ends_with("tydi-lang") {
        // paths = vec![String::from("./tydi_lang_front_end/tydi_source/tydi_ir.td"),
        //              String::from("./tydi_lang_front_end/tydi_source/tydi_ir2.td")];
        paths = vec![String::from("./tydi_lang_front_end/tydi_source/tydi_ir.td")];
    }
    else if base_dir.ends_with("tydi_lang_front_end") {
        // paths = vec![String::from("./tydi_source/tydi_ir.td"),
        //              String::from("./tydi_source/tydi_ir2.td")];
        paths = vec![String::from("./tydi_source/tydi_ir.td")];
    }
    else {
        unreachable!()
    }

    let result = tydi_frontend_compile(Some(String::from("test_project")), paths, Some(String::from("./output")),None);
    if result.is_err() {
        let (project,msg) = result.err().unwrap();
        println!("{}", msg);
        match project {
            None => {}
            Some(project) => {
                println!("{}", project.read().unwrap().pretty_print(0, false));
            }
        }
        assert!(false);
    }

}