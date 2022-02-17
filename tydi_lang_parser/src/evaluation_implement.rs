use std::sync::{Arc, RwLock};
use evaluation_streamlet::resolve_and_infer_streamlet;
use evaluation_var::infer_variable;
use tydi_lang_raw_ast::data_type::DataType;

use tydi_lang_raw_ast::project_arch::Project;
use tydi_lang_raw_ast::scope::{Connection, Implement, Port, Scope, Streamlet, StreamletType, Variable, VariableValue};
use tydi_lang_raw_ast::{inferred, infer_implement};
use tydi_lang_raw_ast::connection::PortOwner;
use tydi_lang_raw_ast::deep_clone::DeepClone;
use tydi_lang_raw_ast::implement::ImplementType;
use tydi_lang_raw_ast::inferable::{Inferable, NewInferable, InferState};
use tydi_lang_raw_ast::instances::{Instance, InstanceArray};
use tydi_lang_raw_ast::port::PortArray;

use crate::ParserErrorCode;
use crate::ParserErrorCode::ImplementEvaluationFail;
use crate::{evaluation_streamlet, evaluation_var};

pub fn infer_implement(implement: Arc<RwLock<Implement>>, implement_template_exps: Vec<Arc<RwLock<Variable>>>, scope: Arc<RwLock<Scope>>, project: Arc<RwLock<Project>>) -> Result<Arc<RwLock<Implement>>, ParserErrorCode> {
    let implement_scope = implement.read().unwrap().get_scope();

    match implement.read().unwrap().get_derived_streamlet() {
        None => {}
        Some(_) => {
            //already inferred
            return Ok(implement.clone());
        }
    }

    let implement_type = implement.read().unwrap().get_type();
    match implement_type {
        ImplementType::NormalImplement => {
            //check implement_template_exps
            if implement_template_exps.len() != 0 { return Err(ImplementEvaluationFail(format!("normal implement cannot have template expressions"))); }

            //infer derived streamlet
            let streamlet_var = implement.read().unwrap().get_derived_streamlet_var();
            let streamlet_var_type = streamlet_var.read().unwrap().get_type();
            match (*streamlet_var_type.read().unwrap()).clone() {
                DataType::ProxyStreamlet(streamlet_name, template_exps) => {
                    let result = evaluation_streamlet::resolve_and_infer_streamlet(streamlet_name.clone(), None, template_exps.clone(), scope.clone(), project.clone());
                    if result.is_err() { return Err(result.err().unwrap()); }
                    {
                        implement.write().unwrap().set_derived_streamlet(Some(result.ok().unwrap().clone()));
                    }
                }
                DataType::ExternalProxyStreamlet(package_name, streamlet_name, template_exps) => {
                    let result = evaluation_streamlet::resolve_and_infer_streamlet(streamlet_name.clone(), Some(package_name.clone()), template_exps.clone(), scope.clone(), project.clone());
                    if result.is_err() { return Err(result.err().unwrap()); }
                    {
                        implement.write().unwrap().set_derived_streamlet(Some(result.ok().unwrap().clone()));
                    }
                }
                _ => { return Err(ImplementEvaluationFail(format!("implement must derive from a streamlet"))); }
            }

            //infer instances
            let instances_in_implement = implement_scope.read().unwrap().instances.clone();
            for (_, instance) in instances_in_implement {
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
                        crate::util::check_import_package(package_name.clone(), scope.clone())?;
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
                let evaluated_implement = result.ok().unwrap();

                //set derived implement
                {
                    instance.write().unwrap().set_implement_type(inferred!(infer_implement!(), evaluated_implement));
                }

                //perform array expansion?
                let instance_array_type = instance.read().unwrap().get_array_type().clone();
                match instance_array_type {
                    InstanceArray::SingleInstance => { /*nothing to do*/ }
                    InstanceArray::ArrayInstance(array_var) => {
                        let result = evaluation_var::infer_variable(array_var.clone(), implement_scope.clone(), project.clone());
                        if result.is_err() { return Err(result.err().unwrap()); }
                        match array_var.read().unwrap().get_var_value().get_raw_value() {
                            VariableValue::Int(array_value) => {
                                if array_value <= 0 { return Err(ImplementEvaluationFail(format!("the length of implement port array must be a positive number"))); }
                                //generate instance array
                                let instance_read = instance.read().unwrap();
                                for i in 0 .. array_value {
                                    let result = implement_scope.write().unwrap().new_instance(format!("{}@{}", instance_read.get_name(), i.to_string()), instance_read.get_package(), instance_read.get_implement_type(), instance_read.get_implement_argexp());
                                    if result.is_err() { return Err(ImplementEvaluationFail(String::from(result.err().unwrap()))); }
                                }
                                //remove the array instance in scope
                                {
                                    implement_scope.write().unwrap().instances.remove(&instance_read.get_name()).unwrap();
                                }
                            }
                            _ => { return Err(ImplementEvaluationFail(format!("the length of an instance array must be an integer"))); }
                        }
                    }
                    _ => unreachable!()
                }
            }

            //infer connection
            for (_, connection) in implement_scope.read().unwrap().connections.clone() {
                infer_connection(connection.clone(), implement.clone(), implement_scope.clone(), project.clone())?;
            }

            return Ok(implement);
        }
        ImplementType::TemplateImplement(template_args) => {
            //get instantiate template name
            let implement_instance_name = crate::util::generate_template_instance_name(implement.read().unwrap().get_name(), &implement_template_exps);

            //infer template expressions
            for template_exp in &implement_template_exps {
                infer_variable(template_exp.clone(), scope.clone(), project.clone())?;
            }

            //clone / instantiate implement
            let mut cloned_implement = implement.read().unwrap().deep_clone();
            cloned_implement.set_name(implement_instance_name);
            cloned_implement.set_type(ImplementType::NormalImplement);
            let cloned_implement = Arc::new(RwLock::new(cloned_implement));
            {
                let result = scope.write().unwrap().with_implement(cloned_implement.clone());
                if result.is_err() { /*that implement might have already exists, so we don't check result*/ }
            }

            //remove the template var in scope
            let cloned_implement_scope = cloned_implement.read().unwrap().get_scope();
            for i in 0 .. template_args.len() {
                let name = template_args[i].read().unwrap().get_name();
                let index = name.find(&*crate::built_in_ids::ARG_PREFIX).unwrap();
                let name = (&name[index+5 ..]).to_string();
                let result = cloned_implement_scope.write().unwrap().vars.remove(&name);
                match result {
                    None => { unreachable!() }
                    Some(_) => {}
                }
            }

            //create corresponding linking var
            if implement_template_exps.len() != template_args.len() { return Err(ImplementEvaluationFail(format!("template expressions mismatch"))); }
            for i in 0 .. implement_template_exps.len() {
                let template_exp = &implement_template_exps[i];
                let template_arg = &template_args[i];
                let template_exp_type_arc = template_exp.read().unwrap().get_type();
                let template_arg_type_arc = template_arg.read().unwrap().get_type();
                let template_exp_type = (*template_exp_type_arc.read().unwrap()).clone();
                let template_arg_type = (*template_arg_type_arc.read().unwrap()).clone();
                if template_arg_type != template_exp_type.clone() {
                    return Err(ImplementEvaluationFail(format!("template expressions mismatch, template type({}) != exp type({})", String::from(template_arg_type.clone()), String::from(template_exp_type.clone()))));
                }
                let linking_var_name = template_arg.read().unwrap().get_name();
                let linking_var_name_index = linking_var_name.find(&*crate::built_in_ids::ARG_PREFIX).unwrap();
                let linking_var_name = (&linking_var_name[linking_var_name_index+5 ..]).to_string();
                match template_arg_type.clone() {
                    DataType::IntType | DataType::StringType | DataType::BoolType | DataType::FloatType | DataType::ArrayType(_) => {
                        let mut linking_var = Arc::new(RwLock::new(Variable::new_with_value(linking_var_name.clone(), template_arg_type.clone(), template_exp.read().unwrap().get_var_value().get_raw_value())));
                        let result = cloned_implement_scope.write().unwrap().with_variable(linking_var);
                        if result.is_err() { return Err(ImplementEvaluationFail(format!("failed to create linking variable({}): {}", linking_var_name.clone(), String::from(result.err().unwrap())))); }
                    }
                    DataType::LogicalDataType(_) => {
                        match template_exp_type {
                            DataType::LogicalDataType(logical_data_type) => {
                                let result = cloned_implement_scope.write().unwrap().new_logical_data_type(linking_var_name.clone(), (*logical_data_type.read().unwrap()).clone());
                                if result.is_err() { return Err(ImplementEvaluationFail(format!("failed to create linking type({}): {}", linking_var_name.clone(), String::from(result.err().unwrap())))); }
                            },
                            _ => return unreachable!(),
                        }
                    }
                    DataType::ProxyImplementOfStreamlet(streamlet_name, streamlet_template_exps) => {
                        let result = resolve_and_infer_streamlet(streamlet_name, None, streamlet_template_exps, implement_scope.clone(), project.clone());
                        if result.is_err() { return Err(result.err().unwrap()); }
                        let streamlet = result.ok().unwrap();

                        //resolve implement
                        let arg_implement;
                        match template_exp_type {
                            DataType::ProxyImplement(name, proxy_implement_template_exps) => {
                                arg_implement = resolve_and_infer_implement(name.clone(), None, proxy_implement_template_exps.clone(), implement_scope.clone(), project.clone())?;
                            }
                            DataType::ExternalProxyImplement(package, name, proxy_implement_template_exps) => {
                                arg_implement = resolve_and_infer_implement(name.clone(), Some(package), proxy_implement_template_exps.clone(), implement_scope.clone(), project.clone())?;
                            }
                            _ => unreachable!()
                        }

                        //check implement is derived from the streamlet
                        let streamlet_of_arg_implement = arg_implement.read().unwrap().get_derived_streamlet().unwrap();
                        let expected_name = streamlet.read().unwrap().get_name();
                        let provided_name = streamlet_of_arg_implement.read().unwrap().get_name();
                        if expected_name != provided_name {
                            return Err(ImplementEvaluationFail(format!("invalid implement, derived from streamlet {} but expects {}", provided_name, provided_name)));
                        }

                        //deep clone
                        let mut cloned_implement = arg_implement.read().unwrap().deep_clone();
                        cloned_implement.set_name(linking_var_name.clone());
                        let result = cloned_implement_scope.write().unwrap().with_implement(Arc::new(RwLock::new(cloned_implement)));
                        if result.is_err() { return Err(ImplementEvaluationFail(String::from(result.err().unwrap()))); }
                    }
                    DataType::ExternalProxyImplementOfStreamlet(package_name, streamlet_name, streamlet_template_exps) => {
                        let result = resolve_and_infer_streamlet(streamlet_name, Some(package_name), streamlet_template_exps, implement_scope.clone(), project.clone());
                        if result.is_err() { return Err(result.err().unwrap()); }
                        let streamlet = result.ok().unwrap();

                        //resolve implement
                        let arg_implement;
                        match template_exp_type {
                            DataType::ProxyImplement(name, proxy_implement_template_exps) => {
                                arg_implement = resolve_and_infer_implement(name.clone(), None, proxy_implement_template_exps.clone(), implement_scope.clone(), project.clone())?;
                            }
                            DataType::ExternalProxyImplement(package, name, proxy_implement_template_exps) => {
                                arg_implement = resolve_and_infer_implement(name.clone(), Some(package), proxy_implement_template_exps.clone(), implement_scope.clone(), project.clone())?;
                            }
                            _ => unreachable!()
                        }

                        //check implement is derived from the streamlet
                        let streamlet_of_arg_implement = arg_implement.read().unwrap().get_derived_streamlet().unwrap();
                        let expected_name = streamlet.read().unwrap().get_name();
                        let provided_name = streamlet_of_arg_implement.read().unwrap().get_name();
                        if expected_name != provided_name {
                            return Err(ImplementEvaluationFail(format!("invalid implement, derived from streamlet {} but expects {}", provided_name, provided_name)));
                        }

                        //deep clone
                        let mut cloned_implement = arg_implement.read().unwrap().deep_clone();
                        cloned_implement.set_name(linking_var_name.clone());
                        let result = cloned_implement_scope.write().unwrap().with_implement(Arc::new(RwLock::new(cloned_implement)));
                        if result.is_err() { return Err(ImplementEvaluationFail(String::from(result.err().unwrap()))); }
                    }
                    _ => { unreachable!() }
                };
            }

            //evaluation the new generated implement
            infer_implement(cloned_implement.clone(), vec![], scope.clone(), project.clone())?;

            return Ok(cloned_implement);
        }
        _ => unreachable!()
    }


}

pub fn infer_connection(connection: Arc<RwLock<Connection>>, implement: Arc<RwLock<Implement>>, scope: Arc<RwLock<Scope>>, project: Arc<RwLock<Project>>) -> Result<(), ParserErrorCode> {
    //infer port owner
    let lhs_owner_result;
    let rhs_owner_result;
    {
        let lhs_port_owner;
        let rhs_port_owner;
        {
            let connection_read = connection.read().unwrap();
            lhs_port_owner = connection_read.get_lhs_port_owner();
            rhs_port_owner = connection_read.get_rhs_port_owner();
        }
        let lhs_owner_result_ = infer_port_owner(lhs_port_owner.clone(), scope.clone(), project.clone());
        if lhs_owner_result_.is_err() { return Err(lhs_owner_result_.err().unwrap()); }
        lhs_owner_result = lhs_owner_result_.ok().unwrap();
        let rhs_owner_result_ = infer_port_owner(rhs_port_owner.clone(), scope.clone(), project.clone());
        if rhs_owner_result_.is_err() { return Err(rhs_owner_result_.err().unwrap()); }
        rhs_owner_result = rhs_owner_result_.ok().unwrap();
        {
            let mut connection_write = connection.write().unwrap();
            connection_write.set_lhs_port_owner(lhs_owner_result.clone());
            connection_write.set_rhs_port_owner(rhs_owner_result.clone());
        }
    }

    //infer port
    {
        let lhs_array = connection.read().unwrap().get_lhs_port_array_type();
        let lhs_port = connection.read().unwrap().get_lhs_port();
        let result = infer_port(lhs_port.clone(), &lhs_owner_result, lhs_array, implement.clone(), scope.clone(), project.clone());
        if result.is_err() { return Err(result.err().unwrap()); }
        {
            connection.write().unwrap().set_lhs_port(result.ok().unwrap());
        }
        let rhs_array = connection.read().unwrap().get_rhs_port_array_type();
        let rhs_port = connection.read().unwrap().get_rhs_port();
        let result = infer_port(rhs_port.clone(), &rhs_owner_result, rhs_array, implement.clone(), scope.clone(), project.clone());
        if result.is_err() { return Err(result.err().unwrap()); }
        {
            connection.write().unwrap().set_rhs_port(result.ok().unwrap());
        }
    }

    //infer delay
    {
        let delay_var = connection.read().unwrap().get_delay();
        let delay_var_result = evaluation_var::infer_variable(delay_var.clone(), scope.clone(), project.clone());
        if delay_var_result.is_err() { return Err(delay_var_result.err().unwrap()); }
        match delay_var.read().unwrap().get_var_value().get_raw_value() {
            VariableValue::Int(i) => {
                if i < 0 { return Err(ImplementEvaluationFail(format!("delay of {} must >= 0", connection.read().unwrap().get_name()))); }
            }
            _ => return {
                let datatype = delay_var.read().unwrap().get_type();
                Err(ImplementEvaluationFail(format!("delay of {} must be an integer, but it's a {}", connection.read().unwrap().get_name(), String::from((*datatype.read().unwrap()).clone()))))
            }
        };
    }

    return Ok(());
}

pub fn infer_port(port_to_infer: Inferable<Arc<RwLock<Port>>>, port_to_infer_owner: &PortOwner, array_exp: PortArray, implement: Arc<RwLock<Implement>>, scope: Arc<RwLock<Scope>>, project: Arc<RwLock<Project>>) -> Result<Inferable<Arc<RwLock<Port>>>, ParserErrorCode> {
    //infer array var
    let index: Option<i32>;
    match array_exp {
        PortArray::UnknownPortArray => { unreachable!() }
        PortArray::SinglePort => { index = None }
        PortArray::ArrayPort(var) => {
            infer_variable(var.clone(), scope.clone(), project.clone())?;
            match var.read().unwrap().get_var_value().get_raw_value() {
                VariableValue::Int(i) => {
                    if i < 0 { return Err(ImplementEvaluationFail(format!("the index of port array must >= 0"))); }
                    index = Some(i);
                }
                _ => return Err(ImplementEvaluationFail(format!("the index of port array must be an integer")))
            }
        }
    }

    let mut infer_output = port_to_infer.clone();
    match port_to_infer_owner.clone() {
        PortOwner::UnknownPortOwner => unreachable!(),
        PortOwner::SelfOwner => {
            let mut port = port_to_infer;
            let streamlet = implement.read().unwrap().get_derived_streamlet().unwrap();
            let streamlet_scope = streamlet.read().unwrap().get_scope();
            let resolve_port_result = streamlet_scope.read().unwrap().resolve_port_in_current_scope(match index {
                None => {
                    port.get_raw_exp()
                }
                Some(i) => {
                    format!("{}@{}", port.get_raw_exp(), i.to_string())
                }
            });
            if resolve_port_result.is_err() { return Err(ImplementEvaluationFail(String::from(resolve_port_result.err().unwrap()))); }
            let resolve_port_result = resolve_port_result.ok().unwrap();
            port.set_raw_value(resolve_port_result.clone());
            port.set_infer_state(InferState::Inferred);
            infer_output = port;
        }
        PortOwner::ExternalOwner(_, streamlet, _) => {
            let mut port;
            match streamlet {
                None => { unreachable!()/*infer_port_owner() should have inferred it*/ }
                Some(streamlet) => {
                    port = port_to_infer;
                    let streamlet_scope = streamlet.read().unwrap().get_scope();
                    let resolve_port_result = streamlet_scope.read().unwrap().resolve_port_in_current_scope(match index {
                        None => {
                            port.get_raw_exp()
                        }
                        Some(i) => {
                            format!("{}@{}", port.get_raw_exp(), i.to_string())
                        }
                    });
                    if resolve_port_result.is_err() { return Err(ImplementEvaluationFail(format!("{}, or it's an array and index out of range", String::from(resolve_port_result.err().unwrap())))); }
                    let resolve_port_result = resolve_port_result.ok().unwrap();
                    port.set_raw_value(resolve_port_result.clone());
                    port.set_infer_state(InferState::Inferred);
                }
            }
            infer_output = port;
        }
    }
    return Ok(infer_output);
}

pub fn infer_port_owner(port_owner: PortOwner, scope: Arc<RwLock<Scope>>, project: Arc<RwLock<Project>>) -> Result<PortOwner, ParserErrorCode> {
    let mut output_port_owner = port_owner.clone();
    match port_owner.clone() {
        PortOwner::SelfOwner => { /*nothing to do*/ }
        PortOwner::ExternalOwner(instance_name, streamlet_of_instance, array_var) => {
            let mut instance_name = instance_name.clone();
            match array_var {
                None => { /*do nothing*/ }
                Some(array_var) => {
                    let result = evaluation_var::infer_variable(array_var.clone(), scope.clone(), project.clone());
                    if result.is_err() { return Err(result.err().unwrap()); }
                    match array_var.read().unwrap().get_var_value().get_raw_value() {
                        VariableValue::Int(i) => {
                            if i < 0 { return Err(ImplementEvaluationFail(format!("instance index must >= 0"))); }
                            instance_name = format!("{}@{}", instance_name, i.to_string());
                        }
                        _ => return Err(ImplementEvaluationFail(format!("instance index must be a positive integer")))
                    }
                }
            }
            //resolve instance
            let instance_result = scope.read().unwrap().resolve_instance_from_scope(instance_name);
            if instance_result.is_err() { return Err(ImplementEvaluationFail(format!("{}, or it's an array and index out of range", String::from(instance_result.err().unwrap())))); }
            //get implement
            let instance_result = instance_result.ok().unwrap();
            let implement_result = instance_result.read().unwrap().get_implement_type(); // all instances in the implement should have already been inferred.
            assert_eq!(implement_result.get_infer_state(), InferState::Inferred);
            let implement_result = implement_result.get_raw_value();
            let streamlet = implement_result.read().unwrap().get_derived_streamlet();
            match streamlet {
                None => { unreachable!() /* the derived streamlet should have already been inferred in the inferring instance stage */ }
                Some(streamlet) => {
                    output_port_owner = match output_port_owner {
                        PortOwner::ExternalOwner(v1,_,v3) => {
                            PortOwner::ExternalOwner(v1, Some(streamlet), v3)
                        }
                        _ => unreachable!()
                    };
                }
            }
        }
        _ => unreachable!()
    }

    return Ok(output_port_owner);
}

pub fn resolve_and_infer_implement(implement_name: String, package_name: Option<String>, implement_template_exps: Vec<Arc<RwLock<Variable>>>, scope: Arc<RwLock<Scope>>, project: Arc<RwLock<Project>>) -> Result<Arc<RwLock<Implement>>, ParserErrorCode> {
    match package_name {
        None => {
            let result = scope.read().unwrap().resolve_implement_from_scope(implement_name.clone());
            if result.is_err() { return Err(ImplementEvaluationFail(String::from(result.err().unwrap()))); }
            let implement = result.ok().unwrap();
            return infer_implement(implement.clone(), implement_template_exps.clone(), scope.clone(), project.clone())
        }
        Some(package_name) => {
            crate::util::check_import_package(package_name.clone(), scope.clone())?;
            let project_read = project.read().unwrap();
            let external_package = project_read.packages.get(&package_name);
            if external_package.is_none() { return Err(ImplementEvaluationFail(format!("package {} not found", package_name.clone()))); }
            let external_package = external_package.unwrap();
            let external_scope = external_package.read().unwrap().get_scope();

            let result = external_scope.read().unwrap().resolve_implement_in_current_scope(implement_name.clone());
            if result.is_err() { return Err(ImplementEvaluationFail(String::from(result.err().unwrap()))); }
            let implement = result.ok().unwrap();
            return infer_implement(implement.clone(), implement_template_exps.clone(), scope.clone(), project.clone())
        }
    }
}
