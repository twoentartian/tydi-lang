use std::collections::HashMap;
use std::sync::{RwLock, Arc};
use tydi_lang_raw_ast::deep_clone::DeepClone;
use tydi_lang_raw_ast::implement::ImplementType;
use tydi_lang_raw_ast::inferable::{NewInferable, Inferable};
use tydi_lang_raw_ast::{inferred, infer_implement, infer_port, not_inferred};
use tydi_lang_raw_ast::evaluated_flag::{EvaluatedFlag, EvaluatedState};
use tydi_lang_raw_ast::project_arch::Project;
use tydi_lang_raw_ast::scope::{DataType, Instance, Port, PortDirection, PortOwner, Variable, Implement, Connection, PortArray};
use crate::std_lib_config;

#[derive(Debug, Clone)]
pub enum SugaringError {
    StdImplementNotFound(String),
    StdLibConfigError(String),

}

impl From<SugaringError> for String {
    fn from(err: SugaringError) -> Self {
        return match err {
            SugaringError::StdImplementNotFound(msg) => {
                format!("StdImplementNotFound: {}", msg)
            }
            SugaringError::StdLibConfigError(msg) => {
                format!("StdLibConfigError: {}", msg)
            }
        }
    }
}

pub fn sugaring_connection(project: Arc<RwLock<Project>>) -> Result<(), SugaringError> {
    let packages = project.read().unwrap().packages.clone();

    //find the std implements
    let std_package = match &std_lib_config::STD_LIB_PACKAGE_NAME {
        None => {
            if packages.len() != 1 { return Err(SugaringError::StdLibConfigError(format!("STD_LIB_PACKAGE_NAME=None cannot be applied for projects with over 1 package"))); }
            let keys: Vec<String> = packages.clone().into_keys().collect();
            packages.get(&keys[0]).unwrap().clone()
        }
        Some(package_name) => {
            let std_package = packages.get(package_name);
            if std_package.is_none() { return Err(SugaringError::StdLibConfigError(format!("std lib {} not found", package_name.clone()))); }
            std_package.unwrap().clone()
        }
    };
    let std_void_impl;
    let std_duplicator_impl;
    {
        let std_package_scope = std_package.read().unwrap().scope.clone();

        let void_impl_name = std_lib_config::STD_VOID_IMPL_NAME.clone();
        let std_package_impls = std_package_scope.read().unwrap().implements.clone();

        let std_void_impl_result = std_package_impls.get(void_impl_name);
        if std_void_impl_result.is_none() { return Err(SugaringError::StdImplementNotFound(format!("impl {} not found in std package {}", void_impl_name.clone(), std_package.read().unwrap().get_name()))); }
        std_void_impl = std_void_impl_result.unwrap().clone();

        let duplicator_impl_name = std_lib_config::STD_DUPLICATOR_IMPL_NAME.clone();
        let std_duplicator_impl_result = std_package_impls.get(duplicator_impl_name);
        if std_duplicator_impl_result.is_none() { return Err(SugaringError::StdImplementNotFound(format!("impl {} not found in std package {}", duplicator_impl_name.clone(), std_package.read().unwrap().get_name()))); }
        std_duplicator_impl = std_duplicator_impl_result.unwrap().clone();
    }

    //do sugaring
    for (_, package) in packages {
        let package_scope = package.read().unwrap().get_scope();
        let package_implements = package_scope.read().unwrap().implements.clone();
        for (_, implement) in package_implements {
            let mut implement_modified = false;
            if implement.read().unwrap().get_external_implement_flag() { continue; } // don't do sugaring on an external implement
            let implement_type = implement.read().unwrap().get_type();
            match implement_type {
                ImplementType::NormalImplement => {}
                _ => { continue; } // don't do sugaring if it's not a normal implement
            }

            //do sugaring
            let mut src_port_mapping = HashMap::<String, Arc<RwLock<Port>>>::new();
            let mut src_to_sink_port_mapping = HashMap::<String, Vec<Arc<RwLock<Port>>>>::new();
            let mut src_to_sink_port_owner_and_array_exp = HashMap::<String, Vec<(PortOwner, PortArray)>>::new();
            let mut src_port_connections = HashMap::<String, Vec<Arc<RwLock<Connection>>>>::new();

            let streamlet = implement.read().unwrap().get_derived_streamlet().unwrap();
            let streamlet_scope = streamlet.read().unwrap().get_scope();
            let ports = streamlet_scope.read().unwrap().ports.clone();
            let implement_scope = implement.read().unwrap().get_scope();
            let connections = implement_scope.read().unwrap().connections.clone();
            let instances = implement_scope.read().unwrap().instances.clone();

            //add all output ports
            for (port_name, port) in ports.clone() {
                if port.read().unwrap().get_direction() == PortDirection::Input { //we need to reverse the port direction when we access a port on itself
                    let src_port_name = port_name;
                    src_port_mapping.insert(src_port_name.clone(), port.clone());
                    src_to_sink_port_mapping.insert(src_port_name.clone(), vec![]);
                    src_to_sink_port_owner_and_array_exp.insert(src_port_name.clone(), vec![]);
                    src_port_connections.insert(src_port_name.clone(), vec![]);
                }
            }
            for (instance_name, instance) in instances.clone() {
                let instance_implement = instance.read().unwrap().get_implement_type().get_raw_value();
                let instance_streamlet = instance_implement.read().unwrap().get_derived_streamlet().unwrap();
                let instance_streamlet_scope = instance_streamlet.read().unwrap().get_scope();
                let instance_ports = instance_streamlet_scope.read().unwrap().ports.clone();
                for (instance_port_name, instance_port) in instance_ports {
                    if instance_port.read().unwrap().get_direction() == PortDirection::Output {
                        let src_port_name = format!("{}::{}", instance_name.clone(), instance_port_name.clone());
                        src_port_mapping.insert(src_port_name.clone(), instance_port.clone());
                        src_to_sink_port_mapping.insert(src_port_name.clone(), vec![]);
                        src_to_sink_port_owner_and_array_exp.insert(src_port_name.clone(), vec![]);
                        src_port_connections.insert(src_port_name.clone(), vec![]);
                    }
                }
            }

            //count the use of each output port
            for (_, connection) in connections.clone() {
                let src_port = connection.read().unwrap().get_lhs_port().get_raw_value();
                let src_port_name = src_port.read().unwrap().get_name();
                let src_port_name = match connection.read().unwrap().get_lhs_port_owner() {
                    PortOwner::UnknownPortOwner => { unreachable!() }
                    PortOwner::SelfOwner => { format!("{}", src_port_name) }
                    PortOwner::ExternalOwner(owner_name, _, _) => { format!("{}::{}", owner_name, src_port_name) }
                };
                let port_mapping_result = src_port_mapping.get(&src_port_name);
                match port_mapping_result {
                    None => { unreachable!() }
                    Some(_) => {}
                }

                let sink_port = connection.read().unwrap().get_rhs_port().get_raw_value();
                let mut current_src_to_sink_ports = src_to_sink_port_mapping.get(&src_port_name).unwrap().clone();
                current_src_to_sink_ports.push(sink_port.clone());
                src_to_sink_port_mapping.insert(src_port_name.clone(), current_src_to_sink_ports);

                let mut current_src_to_sink_port_owners = src_to_sink_port_owner_and_array_exp.get(&src_port_name).unwrap().clone();
                let sink_port_owner = connection.read().unwrap().get_rhs_port_owner();
                let sink_port_array_exp = connection.read().unwrap().get_rhs_port_array_type();
                current_src_to_sink_port_owners.push((sink_port_owner,sink_port_array_exp));
                src_to_sink_port_owner_and_array_exp.insert(src_port_name.clone(), current_src_to_sink_port_owners);

                let mut current_src_port_connections = src_port_connections.get(&src_port_name).unwrap().clone();
                current_src_port_connections.push(connection.clone());
                src_port_connections.insert(src_port_name.clone(), current_src_port_connections);
            }

            //add missing void & duplicator
            for (src_port_name, sink_ports) in &src_to_sink_port_mapping {
                if sink_ports.len() == 0 {
                    implement_modified = true;

                    //add void instance
                    let src_port = src_port_mapping.get(src_port_name).unwrap().clone();
                    let src_port_type = src_port.read().unwrap().get_type().get_raw_value();
                    let mut void_args = vec![];
                    void_args.push(Arc::new(RwLock::new(Variable::new(String::from(""), DataType::LogicalDataType(src_port_type.clone()), String::from("")))));
                    let formal_src_port_name = src_port_name.replace("::" , "_");
                    let void_instance = Instance::new(format!("void_{}", formal_src_port_name.clone()), Some(std_package.read().unwrap().get_name()), not_inferred!(infer_implement!(), std_void_impl.read().unwrap().get_name()), void_args);
                    let void_instance = Arc::new(RwLock::new(void_instance));
                    implement_scope.write().unwrap().instances.insert(void_instance.read().unwrap().get_name(), void_instance.clone());

                    //add connection to void
                    let _src_port_owner;
                    let _src_port_name;
                    {
                        let src_port_name_items: Vec<&str> = src_port_name.split("::").collect();
                        if src_port_name_items.len() == 1 {
                            _src_port_owner = PortOwner::SelfOwner;
                            _src_port_name = String::from(src_port_name_items[0]);
                        }
                        else if src_port_name_items.len() == 2 {
                            _src_port_owner = PortOwner::ExternalOwner(String::from(src_port_name_items[0]), None, None);
                            _src_port_name = String::from(src_port_name_items[1]);
                        }
                        else {
                            unreachable!()
                        }
                    }

                    let mut void_connection = Connection::new(format!("connect_{}", void_instance.read().unwrap().get_name()), not_inferred!(infer_port!(), _src_port_name.clone()), not_inferred!(infer_port!(), String::from("input")), Variable::new_int(String::from(""), 0));
                    void_connection.set_lhs_port_owner(_src_port_owner);
                    let rhs_owner = PortOwner::ExternalOwner(void_instance.read().unwrap().get_name(), None, None);
                    void_connection.set_rhs_port_owner(rhs_owner);
                    void_connection.set_lhs_port_array_type(PortArray::SinglePort);
                    void_connection.set_rhs_port_array_type(PortArray::SinglePort);
                    let void_connection = Arc::new(RwLock::new(void_connection));
                    implement_scope.write().unwrap().connections.insert(void_connection.read().unwrap().get_name(), void_connection.clone());
                }
                else if sink_ports.len() >= 2 {
                    implement_modified = true;

                    //add duplicator instance
                    let src_port = src_port_mapping.get(src_port_name).unwrap().clone();
                    let src_port_type = src_port.read().unwrap().get_type().get_raw_value();
                    let mut duplicator_args = vec![];
                    duplicator_args.push(Arc::new(RwLock::new(Variable::new(String::from(""), DataType::LogicalDataType(src_port_type.clone()), String::from("")))));
                    duplicator_args.push(Arc::new(RwLock::new(Variable::new_int(String::from(""), sink_ports.len() as i64))));
                    let formal_src_port_name = src_port_name.replace("::" , "_");
                    let duplicate_instance = Instance::new(format!("duplicate_{}", formal_src_port_name.clone()), Some(std_package.read().unwrap().get_name()), not_inferred!(infer_implement!(), std_duplicator_impl.read().unwrap().get_name()), duplicator_args);
                    let duplicate_instance = Arc::new(RwLock::new(duplicate_instance));
                    implement_scope.write().unwrap().instances.insert(duplicate_instance.read().unwrap().get_name(), duplicate_instance.clone());

                    //add connections to duplicator
                    let _src_port_owner;
                    let _src_port_name;
                    {
                        let src_port_name_items: Vec<&str> = src_port_name.split("::").collect();
                        if src_port_name_items.len() == 1 {
                            _src_port_owner = PortOwner::SelfOwner;
                            _src_port_name = String::from(src_port_name_items[0]);
                        }
                        else if src_port_name_items.len() == 2 {
                            _src_port_owner = PortOwner::ExternalOwner(String::from(src_port_name_items[0]), None, None);
                            _src_port_name = String::from(src_port_name_items[1]);
                        }
                        else {
                            unreachable!()
                        }
                    }

                    //add connection from external input to duplicator
                    {
                        let mut duplicate_input_connection = Connection::new(format!("connect_{}_input", duplicate_instance.read().unwrap().get_name()), not_inferred!(infer_port!(), _src_port_name.clone()), not_inferred!(infer_port!(), format!("input")), Variable::new_int(String::from(""), 0));
                        duplicate_input_connection.set_lhs_port_owner(_src_port_owner.clone());
                        let rhs_owner = PortOwner::ExternalOwner(duplicate_instance.read().unwrap().get_name(), None, None);
                        duplicate_input_connection.set_rhs_port_owner(rhs_owner);
                        duplicate_input_connection.set_lhs_port_array_type(PortArray::SinglePort);
                        duplicate_input_connection.set_rhs_port_array_type(PortArray::SinglePort);
                        let duplicate_input_connection = Arc::new(RwLock::new(duplicate_input_connection));
                        implement_scope.write().unwrap().connections.insert(duplicate_input_connection.read().unwrap().get_name(), duplicate_input_connection.clone());
                    }

                    //remove the original connections
                    {
                        let connections_to_remove = src_port_connections.get(src_port_name).unwrap();
                        for connection in connections_to_remove {
                            implement_scope.write().unwrap().connections.remove(&connection.read().unwrap().get_name());
                        }
                    }

                    //add connections from duplicator to other outputs
                    let rhs_owners_and_array_exps = src_to_sink_port_owner_and_array_exp.get(src_port_name).unwrap();
                    for (port_index, _) in sink_ports.iter().enumerate() {
                        let rhs_port = sink_ports[port_index].clone();
                        let mut rhs_port_name = rhs_port.read().unwrap().get_name();
                        let find_at_symbol = rhs_port_name.find("@");
                        match find_at_symbol {
                            None => {  }
                            Some(loc) => { rhs_port_name = String::from(&rhs_port_name[0..loc]); }
                        }
                        let mut duplicate_output_connection = Connection::new(format!("connect_{}_output_{}", duplicate_instance.read().unwrap().get_name(), port_index), not_inferred!(infer_port!(), format!("output")), not_inferred!(infer_port!(), rhs_port_name), Variable::new_int(String::from(""), 0));
                        let lhs_owner = PortOwner::ExternalOwner(duplicate_instance.read().unwrap().get_name(), None, None);
                        duplicate_output_connection.set_lhs_port_owner(lhs_owner);
                        duplicate_output_connection.set_lhs_port_array_type(PortArray::ArrayPort(Arc::new(RwLock::new(Variable::new_int(String::from(""), port_index as i64)))));

                        let (rhs_owner, rhs_array_exp) = rhs_owners_and_array_exps[port_index].clone();
                        duplicate_output_connection.set_rhs_port_owner(rhs_owner);
                        duplicate_output_connection.set_rhs_port_array_type(rhs_array_exp);
                        duplicate_output_connection.set_check_restrict_type_same(false);
                        let duplicate_output_connection = Arc::new(RwLock::new(duplicate_output_connection));
                        implement_scope.write().unwrap().connections.insert(duplicate_output_connection.read().unwrap().get_name(), duplicate_output_connection.clone());
                    }
                }
                else {
                    //do nothing
                }
            }
            if implement_modified { implement.write().unwrap().set_evaluate_flag(EvaluatedState::NotEvaluate); }
        }
    }

    return Ok(());
}



