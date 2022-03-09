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

    let output_folder_path = format!("{}/{}", real_output_path.clone(), "4_vhdl");
    let output_folder = Path::new(&output_folder_path);
    if !output_folder.exists() { std::fs::create_dir(output_folder).expect("cannot create VHDl output folder"); }

    let til_folder_path = format!("{}/{}", real_output_path.clone(), "3_til");
    let til_folder = Path::new(&til_folder_path);
    for til_file_result in til_folder.read_dir().expect("cannot read til folder") {
        match til_file_result {
            Ok(file) => {
                let result = parse_to_output(source(file.path()), format!("{}", output_folder_path.clone()));
                if result.is_err() { panic!("{}", result.err().unwrap().to_string()) }
            }
            Err(err) => {
                panic!("{}", err);
            }
        }
    }

}