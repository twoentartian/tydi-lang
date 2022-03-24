use std::sync::{Arc, RwLock};
use std::collections::BTreeMap;
use tydi_lang_raw_ast::project_arch::Project;

use err::SimulatorError;

use circuit_connection;
use circuit_port;
use circuit_implement;
use tydi_lang_raw_ast;

#[derive(Clone, Debug)]
pub struct CircuitGraph {
    implements: BTreeMap<String, Arc<RwLock<circuit_implement::Implement>>>,
}

impl CircuitGraph {
    pub fn new() -> Self {
        return Self {
            implements: BTreeMap::new(),
        }
    }
}

fn get_hierarchy_name(hierarchy: &Vec<String>) -> String {
    let mut output = String::from("");
    let mut first = true;
    for h in hierarchy {
        if first {
            first = false;
        }
        else {
            output.push_str("::");
        }
        output.push_str(h);
    }
    return output;
}

fn add_implement_to_circuit_graph(implement: Arc<RwLock<tydi_lang_raw_ast::implement::Implement>>, circuit_graph: Arc<RwLock<CircuitGraph>>, hierarchy: &Vec<String>) -> Result<(), SimulatorError> {
    let implement_name = implement.read().unwrap().get_name();
    let mut implement_simu = circuit_implement::Implement::new();

    let implement_name_hierarchy = get_hierarchy_name(&hierarchy);
    implement_simu.set_name(implement_name_hierarchy.clone());

    let implement_scope = implement.read().unwrap().get_scope();

    //add sub implement
    {
        let sub_implements = implement_scope.read().unwrap().instances.clone();
        for (name, instance) in sub_implements {
            let implement_of_instance = instance.read().unwrap().get_implement_type().get_raw_value();
            let mut new_hierarchy = hierarchy.clone();
            new_hierarchy.push(name.clone());
            add_implement_to_circuit_graph(implement_of_instance.clone(), circuit_graph.clone(), &new_hierarchy)?;
        }
    }

    //add ports
    {
        let ports = implement_scope.read().unwrap().ports.clone();
        for (name, port) in ports {

        }
    }

    let insert_result = circuit_graph.write().unwrap().implements.insert(implement_simu.get_name(), Arc::new(RwLock::new(implement_simu)));
    if insert_result.is_some() { panic!("insert on an existing key, consider naming bug in circuit graph") }

    return Ok(());
}


pub fn tydi_raw_ast_to_circuit(project: Arc<RwLock<Project>>, top_level_implement_name: String, top_level_implement_package: String) -> Result<Arc<RwLock<CircuitGraph>>, SimulatorError> {
    let packages = project.read().unwrap().packages.clone();
    let target_package = packages.get(&top_level_implement_package);
    if target_package.is_none() { return Err(SimulatorError::ConfigError(format!("package {} doesn't exist", top_level_implement_package))); }
    let target_package = target_package.unwrap();
    let target_package_scope = target_package.read().unwrap().scope.clone();
    let all_implements = target_package_scope.read().unwrap().implements.clone();
    let target_implement = all_implements.get(&top_level_implement_name.clone());
    if target_implement.is_none() { return Err(SimulatorError::ConfigError(format!("cannot find implement {} in package {}", top_level_implement_name, top_level_implement_package))); }
    let target_implement = target_implement.unwrap();

    //create output circuit graph
    let mut output = Arc::new(RwLock::new(CircuitGraph::new()));

    let mut hierarchy = vec![top_level_implement_name.clone()];
    let result = add_implement_to_circuit_graph(target_implement.clone(), output.clone(), &hierarchy);
    if result.is_err() { return Err(result.err().unwrap()); }

    return Ok(output);
}

