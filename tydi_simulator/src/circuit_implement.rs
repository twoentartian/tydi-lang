use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

use crate::circuit_port::Port;
use crate::{generate_set, generate_get, generate_access};

use tydi_lang_raw_ast;

#[derive(Debug, Clone)]
pub struct Implement {
    name: String,

    ports: BTreeMap<String, Arc<RwLock<Port>>>,

    simulation_process: Option<SimulationProcess>,
    inner_implements: Option<Vec<SimulationInstruction>>,
    ast_node: Option<Arc<RwLock<tydi_lang_raw_ast::implement::Implement>>>,
}

impl Implement {
    generate_access!(name, String, get_name, set_name);
    generate_access!(ports, BTreeMap<String, Arc<RwLock<Port>>>, get_ports, set_ports);
    generate_access!(simulation_process, Option<SimulationProcess>, get_simulation_process, set_simulation_process);
    generate_access!(inner_implements, Option<Vec<SimulationInstruction>>, get_inner_implements, set_inner_implements);

    pub fn new() -> Self {
        return Self {
            name: String::from(""),
            ports: BTreeMap::new(),
            simulation_process: None,
            inner_implements: None,
            ast_node: None,
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