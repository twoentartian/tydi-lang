

mod simulator_test {
    use circuit_representation;
    use simulator_config_file::{parse_config, generate_default_config_json};
    use tydi_lang_front_end::tydi_frontend_compile;

    #[test]
    pub fn test_generate_default_config_json() {
        let default_config_json = generate_default_config_json();
        println!("{}", default_config_json);
        let result = parse_config(default_config_json);
        println!("{:?}", result);
        assert!(result.is_ok());
    }

    #[test]
    pub fn test_process_sample_code() {
        let base_dir = std::env::current_dir().expect("not found path");
        let paths:Vec<String>;
        if base_dir.ends_with("tydi-lang") {
            paths = vec![String::from("./tydi_simulator/td_sample/1.td")];
        }
        else if base_dir.ends_with("tydi_simulator") {
            paths = vec![String::from("./td_sample/1.td")];
        }
        else {
            unreachable!()
        }

        let project = tydi_frontend_compile(Some(format!("project1")), paths, Some(format!("./output")), None);
        assert!(project.is_ok(), "{}", project.err().unwrap().1);
        let project = project.ok().unwrap();

        let circuit_graph = circuit_representation::tydi_raw_ast_to_circuit(project.clone(), format!("byte_bypass2"), format!("test"));

        println!("{:?}", circuit_graph);

    }

}