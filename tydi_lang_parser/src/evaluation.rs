use std::sync::{Arc, mpsc, RwLock};
use threadpool::ThreadPool;
use evaluation_var::infer_variable;

use crate::{evaluation_implement, evaluation_streamlet, ParserErrorCode};
use tydi_lang_raw_ast::implement::ImplementType;
use tydi_lang_raw_ast::project_arch::Project;
use tydi_lang_raw_ast::scope::{StreamletType, VariableValue};

pub fn evaluation_project_mt(project: Arc<RwLock<Project>>, flag_streamlet: bool, flag_implement: bool, worker: Option<usize>) -> Result<(), Vec<ParserErrorCode>> {
    let packages = project.clone().read().unwrap().packages.clone();
    let worker_u32 = match worker {
        None => { num_cpus::get() }
        Some(v) => { v }
    };
    let pool = ThreadPool::new(worker_u32);
    let (tx, rx) = mpsc::channel();

    for (_, package) in packages.clone() {
        let package_scope = package.read().unwrap().get_scope().clone();

        if flag_streamlet {
            //infer streamlet
            let package_streamlets = package_scope.read().unwrap().streamlets.clone();
            for (_, streamlet) in package_streamlets.clone() {
                let streamlet_type = streamlet.read().unwrap().get_type();
                match streamlet_type.clone() {
                    StreamletType::NormalStreamlet => {
                        let tx = mpsc::Sender::clone(&tx);
                        let package_scope = package_scope.clone();
                        let project = project.clone();
                        pool.execute(move || {
                            let result = evaluation_streamlet::infer_streamlet(streamlet.clone(), vec![], package_scope.clone(), project.clone());
                            if result.is_err() {
                                tx.send(Err(result.err().unwrap())).unwrap();
                            }
                        });
                    }
                    StreamletType::TemplateStreamlet(_) => {
                        //don't evaluate template
                    }
                    _ => unreachable!()
                }
            }
        }

        if flag_implement {
            //infer implements
            let package_implements = package_scope.read().unwrap().implements.clone();
            for (_, implement) in package_implements.clone() {
                let implement_type = implement.read().unwrap().get_type();
                match implement_type.clone() {
                    ImplementType::NormalImplement => {
                        let tx = mpsc::Sender::clone(&tx);
                        let package_scope = package_scope.clone();
                        let project = project.clone();
                        pool.execute(move || {
                            let result = evaluation_implement::infer_implement(implement, vec![], package_scope.clone(), project.clone());
                            if result.is_err() {
                                tx.send(Err(result.err().unwrap())).unwrap();
                            }
                        });
                    }
                    ImplementType::TemplateImplement(_) => {
                        //don't evaluate template
                    }
                    _ => unreachable!()
                }
            }

            //infer global implementation instance
            let package_instances = package_scope.read().unwrap().instances.clone();
            for (_, single_inst) in package_instances {
                let tx = mpsc::Sender::clone(&tx);
                let project = project.clone();
                let package_scope = package_scope.clone();
                pool.execute(move || {
                    let result = evaluation_implement::infer_instance(single_inst.clone(), package_scope.clone(), project.clone());
                    if result.is_err() { tx.send(result.clone()).unwrap(); }
                });
            }
        }
    }

    pool.join();

    let mut errors: Vec<ParserErrorCode> = vec![];
    let mut error_flag = false;
    for single_rx in rx.try_iter() {
        match single_rx {
            Ok(_) => {}
            Err(err) => {
                errors.push(err);
                error_flag = true;
            }
        }
    }

    if error_flag { return Err(errors) }

    //infer asserts
    for (_, package) in packages.clone() {
        let package_scope = package.read().unwrap().get_scope().clone();
        let package_asserts = package_scope.read().unwrap().asserts.clone();
        for (_, assert) in package_asserts.clone() {
            let assert_var = assert.read().unwrap().get_var();
            let result = infer_variable(assert_var.clone(), package_scope.clone(), project.clone());
            if result.is_err() { return Err(vec![result.err().unwrap()]); }
            let assert_var_read = assert_var.read().unwrap();
            match assert_var_read.get_var_value().get_raw_value() {
                VariableValue::Bool(v) => {
                    if !v {
                        let msg = assert.read().unwrap().get_msg();
                        match msg {
                            None => {
                                return Err(vec![ParserErrorCode::AnalysisCodeStructureFail(format!("assert fails: {}", assert.read().unwrap().get_name()))]);
                            }
                            Some(msg) => {
                                let result = infer_variable(msg.clone(), package_scope.clone(), project.clone());
                                if result.is_err() {
                                    return Err(vec![ParserErrorCode::AnalysisCodeStructureFail(format!("assert fails: {} and its error message also fails to infer", assert.read().unwrap().get_name()))]);
                                }
                                else {
                                    return Err(vec![ParserErrorCode::AnalysisCodeStructureFail(format!("assert fails: {} - {}", assert.read().unwrap().get_name(), String::from((*msg.read().unwrap()).clone())))]);
                                }
                            }
                        }
                    }
                }
                _ => { return Err(vec![ParserErrorCode::AnalysisCodeStructureFail(format!("assert function only accepts bool expression"))]); }
            }
        }
    }

    return Ok(());
}
