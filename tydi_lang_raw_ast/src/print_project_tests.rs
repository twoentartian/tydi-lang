
#[cfg(test)]
mod tests {
    pub use std::collections::HashMap;
    use std::sync::{Arc, RwLock};

    use infer_logical_data_type;
    use crate::implement::ImplementType;
    use crate::inferable::{Inferable, NewInferable};
    use crate::{infer_port, inferred, not_inferred, infer_streamlet};
    use crate::project_arch::Project;
    use crate::scope::*;

    #[test]
    fn var_scope() {
        let mut project0 = Project::new(String::from("project0"));
        let package_name = String::from("package0");
        project0.new_package(package_name.clone()).unwrap();
        let result = project0.find_package(package_name.clone()).unwrap();
        let package = result.write().unwrap();
        let mut package_scope = package.scope.write().unwrap();
        match package_scope.new_variable(String::from("var1"), DataType::IntType, String::from("")) {
            Ok(()) => {}
            Err(err_code) => {
                println!("error: {:?}", err_code);
                assert!(false);
            }
        }
        match package_scope.new_variable(String::from("var1"), DataType::IntType, String::from("")) {
            Ok(()) => {}
            Err(err_code) => {
                match err_code {
                    ErrorCode::IdRedefined(_) => {assert!(true)}
                    _ => {assert!(false)}
                }
            }
        }
        package_scope.new_variable(String::from("var2"), DataType::StringType, String::from("")).unwrap();

        println!("{}", package_scope.pretty_print(0, false));

    }

    #[test]
    fn print_project() {
        let mut project0 = Project::new(String::from("project0"));

        //set default stream parameter
        {
            let mut default_stream = DEFAULT_LOGICAL_STREAM.write().unwrap();
            default_stream.set_dimension(Variable::new_int(String::from(""), 2));
            default_stream.set_complexity(Variable::new_int(String::from(""), 6));
        }

        //generate project
        {
            let package_name = String::from("package0");
            let package_scope_l = project0.new_package(package_name.clone()).unwrap();
            let mut package_scope = package_scope_l.write().unwrap();
            package_scope.new_variable(String::from("var1"), DataType::IntType, String::from("")).unwrap();
            package_scope.new_variable(String::from("var2"), DataType::StringType, String::from("")).unwrap();
            package_scope.new_variable(String::from("f0"), DataType::FloatType, String::from("")).unwrap();
            let new_group = package_scope.new_logical_group(String::from("group0")).unwrap();
            {
                let mut group_scope = new_group.write().unwrap();
                group_scope.new_variable(String::from("var3"), DataType::StringType, String::from("")).unwrap();
                group_scope.new_logical_bit(String::from("bit16"), String::from("16")).unwrap();
                group_scope.new_logical_bit_with_definite(String::from("bit16_"), 16).unwrap();
            }
            package_scope.new_logical_union(String::from("union0")).unwrap();
            package_scope.new_logical_null(String::from("null")).unwrap();
            package_scope.new_logical_bit(String::from("bit8"), String::from("8")).unwrap();
            package_scope.new_external_type(String::from("external"), String::from("pack"), String::from("t1"));
            let temp_type = package_scope.resolve_type_in_current_scope(String::from("group0")).unwrap();

            let streamlet_new = package_scope.new_streamlet(String::from("streamlet0"), StreamletType::NormalStreamlet).unwrap();
            {
                let mut streamlet_scope = streamlet_new.write().unwrap();
                streamlet_scope.new_variable(String::from("var4"), DataType::StringType, String::from("")).unwrap();
            }

            {
                let type_alias = temp_type.read().unwrap();
                let t = type_alias.get_type_infer().get_raw_value();

                package_scope.new_logical_stream(String::from("stream0"), inferred!(infer_logical_data_type!(), t.clone()));

                match package_scope.resolve_streamlet_from_scope(String::from("streamlet0")) {
                    Ok(streamlet) => {
                        //streamlet.read().unwrap().new_port(String::from("port0"), <Inferable<Arc<RwLock<LogicalDataType>>> as NewInferable<Arc<RwLock<LogicalDataType>>>>::_new_inferred(String::from(""), t.clone()) , PortDirection::Input);
                        streamlet.read().unwrap().new_port(String::from("port0"), inferred!(infer_logical_data_type!(), t.clone()) , PortDirection::Input).unwrap();
                        streamlet.read().unwrap().new_port(String::from("port1"), not_inferred!(infer_logical_data_type!(), String::from("port1_type")) , PortDirection::Input).unwrap();
                    }
                    Err(_) => { assert!(false) }
                }
            }

            let implement_scope = package_scope.new_implement(String::from("impl0"), ImplementType::NormalImplement).unwrap();

            {
                let mut impl_scope = implement_scope.write().unwrap();
                impl_scope.new_instance(String::from("instance"), Some(String::from("external_package")), not_inferred!(infer_streamlet!(), String::from("streamlet_unknown")), vec![]).unwrap();
                impl_scope.new_connection(String::from("connection0"),
                                          not_inferred!(infer_port!(), String::from("a.b")),
                                          not_inferred!(infer_port!(), String::from("a.b")),
                                          Variable::new(String::from("temp"), DataType::IntType, String::from("1"))).unwrap();

                let if_scope = impl_scope.new_if_block(String::from("if_block0"), Arc::new(RwLock::new(Variable::new(String::from(""), DataType::BoolType, String::from("true")))), String::from("parent")).unwrap();
                {
                    if_scope.write().unwrap().new_for_block(String::from("for_block0"), Arc::new(RwLock::new(Variable::new(String::from(""), DataType::IntType, String::from("i")))),
                                                            Arc::new(RwLock::new(Variable::new(String::from(""), DataType::ArrayType(Arc::new(RwLock::new(DataType::BoolType))), String::from("i_array"))))).unwrap();
                }
            }

        }
        println!("{}", project0.pretty_print(0, false));

        //access
        {
            let package_container = project0.find_package(String::from("package0")).unwrap();
            let package = package_container.read().unwrap();
            let group_type = package.scope.read().unwrap().resolve_type_in_current_scope(String::from("group0")).unwrap();
            let group_type_alias = group_type.read().unwrap();

            match &*(group_type_alias.get_type_infer().get_raw_value().read().unwrap()) {
                LogicalDataType::DataGroupType(_, group_scope) => {
                    let result = group_scope.read().unwrap().get_scope().read().unwrap().resolve_variable_from_scope(String::from("var1"));
                    let _ = result.unwrap();

                    let result = group_scope.read().unwrap().get_scope().read().unwrap().resolve_variable_in_current_scope(String::from("var1"));
                    match result {
                        Ok(_) => { assert!(false) }
                        Err(_) => { assert!(true) }
                    }

                    let result = group_scope.read().unwrap().get_scope().read().unwrap().resolve_type_from_scope(String::from("bit8"));
                    let _ = result.unwrap();

                    let result = group_scope.read().unwrap().get_scope().read().unwrap().resolve_type_in_current_scope(String::from("bit8"));
                    match result {
                        Ok(_) => { assert!(false) }
                        Err(_) => { assert!(true) }
                    }
                }
                _ => {}
            }
            // let output_str = group_type.read().unwrap().pretty_print(0, false);
            // println!("{}", output_str);
        }
    }
}