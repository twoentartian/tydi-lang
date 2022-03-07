extern crate tydi_lang_front_end;
extern crate chrono;
extern crate til_vhdl;
extern crate tydi_common;
extern crate til_parser;

use std::ffi::OsStr;
use std::path::Path;
use chrono::{Datelike, Timelike};

fn source(path: impl AsRef<Path>) -> String {
    std::fs::read_to_string(path).unwrap()
}

fn parse_to_output(src: impl Into<String>, dst: String) -> tydi_common::error::Result<()> {
    let db = til_parser::query::into_query_storage(src)?;

    til_vhdl::canonical(&db, dst)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut src_file_path: Vec<String> = vec![];
    let mut project_name: Option<String> = None;
    let mut output_path: Option<String> = None;
    let mut worker: Option<usize> = None;

    //parse args
    let mut arg_index = 1; // ignore the first arg because it's the env root
    while arg_index < args.len() {
        let arg = &args[arg_index];
        if arg == "-n" {
            arg_index = arg_index + 1;
            let arg = &args[arg_index];
            if project_name == None { project_name = Some(arg.clone()); }
            else { panic!("project name override"); }
        }
        else if arg == "-o" {
            arg_index = arg_index + 1;
            let arg = &args[arg_index];
            if output_path == None { output_path = Some(arg.clone()); }
            else { panic!("output path override"); }
        }
        else if arg == "-j" {
            arg_index = arg_index + 1;
            let arg = &args[arg_index];
            if worker == None { worker = Some(arg.parse().unwrap()); }
            else { panic!("output path override"); }
        }
        else {
            let src_file = Path::new(arg);
            if !src_file.exists() { panic!("path ({}) doesn't exist", arg); }
            if src_file.is_file() {
                if src_file.extension() == Some(OsStr::new("td")){
                    src_file_path.push(arg.clone());
                }
                else {
                    panic!("file ({}) should have extension \"td\"", arg);
                }
            }
            if src_file.is_dir() {
                let inner_srcs = src_file.read_dir();
                if inner_srcs.is_err() { panic!("unable to read dir ({:?})", inner_srcs); }
                let inner_srcs = inner_srcs.unwrap();
                for inner_src in inner_srcs.into_iter() {
                    let inner_src = inner_src.unwrap().path();
                    if inner_src.extension() == Some(OsStr::new("td")){
                        src_file_path.push(inner_src.to_str().unwrap().to_string().clone());
                    }
                }
            }
        }
        arg_index = arg_index + 1;
    }

    //set default project names and output path
    let real_project_name = match project_name {
        None => {
            let now = chrono::Local::now();
            format!("project_{}_{:02}_{:02}_{:02}_{:02}_{:02}", now.year(), now.month(), now.day(), now.hour(), now.minute(), now.second())
        }
        Some(name) => { name }
    };

    let real_output_path = match output_path {
        None => { format!("./{}", real_project_name.clone()) }
        Some(path) => { path }
    };

    if src_file_path.len() == 0 { panic!("no given source file"); }

    //ready for compiling
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    println!("Tydi compiler version: {}", VERSION);
    println!("env root path: {:?}", std::env::current_dir());
    println!("project name: {}", real_project_name.clone());
    println!("project output path: {}", real_output_path.clone());
    println!("src files: {:?}", src_file_path.clone());
    println!("worker: {:?}", worker.clone());

    let compile_result = tydi_lang_front_end::tydi_frontend_compile(Some(real_project_name.clone()), src_file_path, Some(real_output_path.clone()), worker);
    if compile_result.is_err() {
        let (project_arch, err_msg) = compile_result.err().unwrap();
        match project_arch {
            Some(project_arch) => {
                use tydi_lang_raw_ast::util::PrettyPrint;
                std::fs::write(format!("{}/{}", real_output_path.clone(), "err_arch.txt"), project_arch.read().unwrap().pretty_print(0, false)).expect("error to write the err_arch.txt");
            }
            None => {}
        }

        panic!("{}", err_msg);
    }

    println!("generating Tydi IR - done");

    let project_arch = compile_result.ok().unwrap();

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

    println!("generating VHDL - done");

}
