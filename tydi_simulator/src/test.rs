mod simulator_test {
    use circuit_frequency::SimulationClockDomainMapping;
    use circuit_representation;
    use simulator_config_file;
    use simulator_config_file::ConfigItem_SimulatorConfig;
    use to_dot::ToDot;
    use tydi_lang_front_end::tydi_frontend_compile;

    #[test]
    pub fn test_generate_default_config_json() {
        let default_config_json = simulator_config_file::generate_default_config_json();
        println!("{}", default_config_json);
        let mut config = ConfigItem_SimulatorConfig::new();
        let result = config.load_config_str(default_config_json);
        println!("{:?}", result);
        assert!(result.is_ok());
    }

    #[test]
    pub fn test_process_sample_code() {
        let base_dir = std::env::current_dir().expect("not found path");
        let paths:Vec<String>;
        let config_file_path;
            if base_dir.ends_with("tydi-lang") {
            paths = vec![String::from("./tydi_simulator/td_sample/1.td")];
            config_file_path = String::from("./tydi_simulator/td_sample/config_1.json");
        }
        else if base_dir.ends_with("tydi_simulator") {
            paths = vec![String::from("./td_sample/1.td")];
            config_file_path = String::from("./td_sample/config_1.json");
        }
        else {
            unreachable!()
        }

        let mut config = simulator_config_file::ConfigItem_SimulatorConfig::new();
        config.load_config_file(config_file_path).expect("failed to load config file");
        let mut clockdomain_mapping = SimulationClockDomainMapping::new();
        clockdomain_mapping.convert_from_configure_file(&config);

        let project = tydi_frontend_compile(Some(format!("project1")), paths, Some(format!("./output")), None);
        assert!(project.is_ok(), "{}", project.err().unwrap().1);
        let project = project.ok().unwrap();

        let circuit_graph = circuit_representation::tydi_raw_ast_to_circuit(project.clone(), config.top_level_implement.clone(), config.top_level_implement_package.clone(), &clockdomain_mapping);
        if circuit_graph.is_err() { panic!("failed to convert circuit graph") }
        let circuit_graph = circuit_graph.unwrap();
        let dot_content = circuit_graph.read().unwrap().to_dot();
        std::fs::write("output/circuit.dot", dot_content);
    }

}