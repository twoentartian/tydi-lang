use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

use crate::circuit_port::Port;
use crate::{generate_set, generate_get, generate_access};

use tydi_lang_raw_ast;

#[derive(Clone)]
pub struct Implement {
    name: String,

    ports: BTreeMap<String, Arc<RwLock<Port>>>,

    simulation_process: Option<SimulationProcess>,
    inner_implements: Option<Vec<SimulationInstruction>>,
    ast_node_implement: Option<Arc<RwLock<tydi_lang_raw_ast::scope::Implement>>>,
    ast_node_instance: Option<Arc<RwLock<tydi_lang_raw_ast::scope::Instance>>>,
    hierarchy: Vec<String>,
    external_implement_flag: bool,
}

impl Implement {
    generate_access!(name, String, get_name, set_name);
    generate_access!(ports, BTreeMap<String, Arc<RwLock<Port>>>, get_ports, set_ports);
    generate_access!(simulation_process, Option<SimulationProcess>, get_simulation_process, set_simulation_process);
    generate_access!(inner_implements, Option<Vec<SimulationInstruction>>, get_inner_implements, set_inner_implements);

    generate_access!(ast_node_implement, Option<Arc<RwLock<tydi_lang_raw_ast::scope::Implement>>>, get_ast_node_implement, set_ast_node_implement);
    generate_access!(ast_node_instance, Option<Arc<RwLock<tydi_lang_raw_ast::scope::Instance>>>, get_ast_node_instance, set_ast_node_instance);
    generate_access!(hierarchy, Vec<String>, get_hierarchy, set_hierarchy);
    generate_access!(external_implement_flag, bool, get_external_implement_flag, set_external_implement_flag);

    pub fn new() -> Self {
        return Self {
            name: String::from(""),
            ports: BTreeMap::new(),
            simulation_process: None,
            inner_implements: None,
            ast_node_instance: None,
            ast_node_implement: None,
            hierarchy: vec![],
            external_implement_flag: false,
        }
    }

}

#[derive(Debug, Clone)]
pub struct SimulationProcess {
    pub instructions: Vec<SimulationInstruction>,


}

#[derive(Debug, Clone)]
pub struct SimulationInstruction {

}