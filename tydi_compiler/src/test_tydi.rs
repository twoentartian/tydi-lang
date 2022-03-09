#[test]
pub fn test() {
    let base_dir = std::env::current_dir().expect("not found path");
    let paths:Vec<String>;
    if base_dir.ends_with("tydi-lang") {
        // paths = vec![String::from("./tydi_compiler/tydi_source/tydi_ir.td"),
        //              String::from("./tydi_compiler/tydi_source/tydi_ir2.td")];
        paths = vec![String::from("./tydi_compiler/tydi_source/tydi_ir.td")];
    }
    else if base_dir.ends_with("tydi_compiler") {
        // paths = vec![String::from("./tydi_source/tydi_ir.td"),
        //              String::from("./tydi_source/tydi_ir2.td")];
        paths = vec![String::from("./tydi_source/tydi_ir.td")];
    }
    else {
        unreachable!()
    }

    crate::tydi_compile(String::from("test_project"), paths, String::from("./output"),None);
}