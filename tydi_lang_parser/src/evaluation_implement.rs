use std::sync::{Arc, RwLock};
use tydi_lang_raw_ast::data_type::DataType;

use tydi_lang_raw_ast::project_arch::Project;
use tydi_lang_raw_ast::scope::{Implement, Scope, StreamletType, Variable};
use tydi_lang_raw_ast::{inferred, infer_implement};
use tydi_lang_raw_ast::inferable::{Inferable, NewInferable};

use crate::ParserErrorCode;
use crate::ParserErrorCode::ImplementEvaluationFail;
use crate::evaluation_streamlet;

pub fn infer_implement(implement: Arc<RwLock<Implement>>, implement_template_exps: Vec<Arc<RwLock<Variable>>>, scope: Arc<RwLock<Scope>>, project: Arc<RwLock<Project>>) -> Result<(), ParserErrorCode> {
    let implement_scope = implement.read().unwrap().get_scope();

    match implement.read().unwrap().get_derived_streamlet() {
        None => {}
        Some(_) => {
            //already inferred
            return Ok(());
        }
    }

    //infer derived streamlet
    let streamlet_var = implement.read().unwrap().get_derived_streamlet_var();
    let streamlet_var_type = streamlet_var.read().unwrap().get_type();
    match (*streamlet_var_type.read().unwrap()).clone() {
        DataType::ProxyStreamlet(streamlet_name, template_exps) => {
            let result = implement_scope.read().unwrap().resolve_streamlet_from_scope(streamlet_name.clone());
            if result.is_err() { return Err(ImplementEvaluationFail(String::from(result.err().unwrap()))); }
            let streamlet = result.ok().unwrap();
            let result = evaluation_streamlet::infer_streamlet(streamlet.clone(), template_exps.clone(), scope.clone(), project.clone());
            if result.is_err() { return Err(result.err().unwrap()); }
            {
                implement.write().unwrap().set_derived_streamlet(Some(result.ok().unwrap().clone()));
            }
        }
        DataType::ExternalProxyStreamlet(package_name, streamlet_name, template_exps) => {
            //check import
            {
                let package_var_result = scope.read().unwrap().resolve_variable_from_scope(format!("{}{}", *crate::built_in_ids::PACKAGE_PREFIX, package_name.clone()));
                if package_var_result.is_err() { return Err(ImplementEvaluationFail(format!("package {} not imported", package_name.clone()))); }
            }
            let project_read = project.read().unwrap();
            let external_package = project_read.packages.get(&package_name);
            if external_package.is_none() { return Err(ImplementEvaluationFail(format!("package {} not found", package_name.clone()))); }
            let external_package = external_package.unwrap();
            let external_scope = external_package.read().unwrap().get_scope();

            let result = external_scope.read().unwrap().resolve_streamlet_in_current_scope(streamlet_name.clone());
            if result.is_err() { return Err(ImplementEvaluationFail(String::from(result.err().unwrap()))); }
            let streamlet = result.ok().unwrap();
            let result = evaluation_streamlet::infer_streamlet(streamlet.clone(), template_exps.clone(), scope.clone(), project.clone());
            if result.is_err() { return Err(result.err().unwrap()); }
            {
                implement.write().unwrap().set_derived_streamlet(Some(result.ok().unwrap().clone()));
            }
        }
        _ => { return Err(ImplementEvaluationFail(format!("implement must derive from a streamlet"))); }
    }

    //infer instances
    for (_, instance) in implement_scope.read().unwrap().instances.clone() {
        let package;
        let derived_implement_template_exps;
        let instance_array_type;
        let derived_implement_name;
        {
            let instance_read = instance.read().unwrap();
            package = instance_read.get_package();
            derived_implement_template_exps = instance_read.get_implement_argexp();
            instance_array_type = instance_read.get_array_type();
            derived_implement_name = instance_read.get_implement_type().get_raw_exp();
        }

        let mut resolve_implement_result;
        match package {
            None => {
                //local streamlet
                let find_implement_result = implement_scope.read().unwrap().resolve_implement_from_scope(derived_implement_name.clone());
                if find_implement_result.is_err() { return Err(ImplementEvaluationFail(String::from(find_implement_result.err().unwrap()))); }
                resolve_implement_result = find_implement_result.ok().unwrap();
            }
            Some(package_name) => {
                //external streamlet
                //check import
                {
                    let package_var_result = scope.read().unwrap().resolve_variable_from_scope(format!("{}{}", *crate::built_in_ids::PACKAGE_PREFIX, package_name.clone()));
                    if package_var_result.is_err() { return Err(ImplementEvaluationFail(format!("package {} not imported", package_name.clone()))); }
                }
                let project_read = project.read().unwrap();
                let external_package = project_read.packages.get(&package_name);
                if external_package.is_none() { return Err(ImplementEvaluationFail(format!("package {} not found", package_name.clone()))); }
                let external_package = external_package.unwrap();
                let external_scope = external_package.read().unwrap().get_scope();
                let find_streamlet_result = external_scope.read().unwrap().resolve_implement_in_current_scope(derived_implement_name.clone());
                if find_streamlet_result.is_err() { return Err(ImplementEvaluationFail(String::from(find_streamlet_result.err().unwrap()))); }
                resolve_implement_result = find_streamlet_result.ok().unwrap();
            }
        }

        //evaluation implement
        let result = infer_implement(resolve_implement_result.clone(), derived_implement_template_exps.clone(), scope.clone(), project.clone());
        if result.is_err() { return Err(result.err().unwrap()); }

        //set derived implement
        {
            instance.write().unwrap().set_implement_type(inferred!(infer_implement!(), resolve_implement_result));
        }
    }

    //infer connection
    for (_, connection) in implement_scope.read().unwrap().connections.clone() {
        //todo!()
    }


    return Ok(());
}