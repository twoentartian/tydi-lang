#[allow(unused_imports)]
use tydi_lang_parser::evaluation;
#[allow(unused_imports)]
use tydi_lang_raw_ast::util::PrettyPrint;
#[allow(unused_imports)]
use crate::tydi_frontend_compile;
use crate::sugaring_connection;

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
                //println!("{}", project.read().unwrap().pretty_print(0, false));
            }
        }
        assert!(false);
        return;
    }


    //drc check
    let project_arch = result.ok().unwrap();
    //println!("{}", project_arch.read().unwrap().pretty_print(0, false));
    let drc_msg = crate::drc::tydi_design_rule_check(project_arch.clone());
    println!("drc messages: {}", drc_msg.len());

    for msg in drc_msg {
        println!("{:?}", msg);
    }

    //print project arch
    //println!("{}", project_arch.read().unwrap().pretty_print(0, false));

}