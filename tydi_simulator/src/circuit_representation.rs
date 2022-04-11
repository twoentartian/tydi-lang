use std::sync::{Arc, RwLock};
use std::collections::BTreeMap;
use tydi_lang_raw_ast::project_arch::Project;

use err::SimulatorError;

use circuit_connection;
use circuit_frequency;
use circuit_port;
use circuit_implement;
use circuit_port::{Port, PortBindDirection};
use simulator_config_file::ConfigItem_SimulatorClockDomainMapping;
use tydi_lang_raw_ast;
use tydi_lang_raw_ast::connection::PortOwner;
use tydi_lang_raw_ast::scope::Variable;
use crate::{generate_set, generate_get, generate_access};

#[derive(Clone)]
pub struct CircuitGraph {
    implements: BTreeMap<String, Arc<RwLock<circuit_implement::Implement>>>,
    connections: BTreeMap<String, Arc<RwLock<circuit_connection::Connection>>>,
    clockdomain_mapping: Option<circuit_frequency::SimulationClockDomainMapping>,

}

impl CircuitGraph {
    generate_access!(implements, BTreeMap<String, Arc<RwLock<circuit_implement::Implement>>>, get_implements, set_implements);
    generate_access!(connections, BTreeMap<String, Arc<RwLock<circuit_connection::Connection>>>, get_connections, set_connections);
    generate_access!(clockdomain_mapping, Option<circuit_frequency::SimulationClockDomainMapping>, get_clockdomain_mapping, set_clockdomain_mapping);

    pub fn new() -> Self {
        return Self {
            implements: BTreeMap::new(),
            connections: BTreeMap::new(),
            clockdomain_mapping: None,
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

impl CircuitGraph {
    fn add_single_connection(&mut self, ast_connection: Arc<RwLock<tydi_lang_raw_ast::scope::Connection>>, src_port: Arc<RwLock<circuit_port::Port>>, sink_port: Arc<RwLock<circuit_port::Port>>) -> Result<Arc<RwLock<circuit_connection::Connection>>, SimulatorError> {
        let mut connection_to_insert = circuit_connection::Connection::new();
        connection_to_insert.set_src_port(Some(src_port.clone()));
        connection_to_insert.set_sink_port(Some(sink_port.clone()));

        let src_implement = src_port.read().unwrap().get_parent_implement().unwrap().clone();
        let sink_implement = sink_port.read().unwrap().get_parent_implement().unwrap().clone();
        let src_implement_name = src_implement.read().unwrap().get_name();
        let sink_implement_name = sink_implement.read().unwrap().get_name();
        //connection_to_insert.set_name(format!("{}.{} -> {}.{} ({})", src_implement_name, src_port.read().unwrap().get_name(), sink_implement_name, sink_port.read().unwrap().get_name(), ast_connection.read().unwrap().get_name()));
        connection_to_insert.set_name(format!("{}__{}__{}", ast_connection.read().unwrap().get_name(), src_implement_name, sink_implement_name));

        let connection_to_insert_name = connection_to_insert.get_name();
        let inserted_connection = Arc::new(RwLock::new(connection_to_insert));
        let result = self.connections.insert(connection_to_insert_name, inserted_connection.clone());
        assert!(result.is_none());

        return Ok(inserted_connection);
    }

    fn find_corresponding_simulation_port(&self, simulation_implement: Arc<RwLock<circuit_implement::Implement>>, ast_port: Arc<RwLock<tydi_lang_raw_ast::scope::Port>>, ast_port_owner: &PortOwner) -> Result<(Arc<RwLock<circuit_port::Port>>, circuit_port::PortBindDirection), SimulatorError>
    {
        let output_port;
        let output_port_bind_direction;

        match ast_port_owner {
            PortOwner::UnknownPortOwner => { unreachable!() }
            PortOwner::SelfOwner => {
                output_port_bind_direction = PortBindDirection::Internal;
                let all_ports_in_simulation_implement = simulation_implement.read().unwrap().get_ports();
                let target_port = all_ports_in_simulation_implement.get(&ast_port.read().unwrap().get_name()).expect("Bug: port name not found in this implement");
                output_port = target_port.clone();

            }
            PortOwner::ExternalOwner(instance_name, _, array_index) => {
                output_port_bind_direction = PortBindDirection::External;

                let external_simulation_implement_name = match array_index {
                    None => { format!("{}::{}", simulation_implement.read().unwrap().get_name(), instance_name.clone()) }
                    Some(var) => { format!("{}::{}@{}", simulation_implement.read().unwrap().get_name(), instance_name.clone(), String::from(var.read().unwrap().get_var_value().get_raw_value().clone())) }
                };
                let external_simulation_implement = self.implements.get(&external_simulation_implement_name).expect("Bug: the external simulation implement should be in the circuit graph");
                let external_simulation_implement_ports = external_simulation_implement.read().unwrap().get_ports();
                let target_port = external_simulation_implement_ports.get(&ast_port.read().unwrap().get_name()).expect("Bug: port name not found in this implement");
                output_port = target_port.clone();
            }
        }

        return Ok((output_port, output_port_bind_direction));
    }

    fn add_implement_to_circuit_graph(&mut self, implement: Arc<RwLock<tydi_lang_raw_ast::implement::Implement>>, hierarchy: &Vec<String>) -> Result<(), SimulatorError> {
        let mut implement_simu = Arc::new(RwLock::new(circuit_implement::Implement::new()));

        let implement_name_hierarchy = get_hierarchy_name(&hierarchy);
        {
            let mut implement_simu_write = implement_simu.write().unwrap();
            implement_simu_write.set_name(implement_name_hierarchy.clone());
            implement_simu_write.set_external_implement_flag(implement.read().unwrap().get_external_implement_flag());
        }

        let implement_scope = implement.read().unwrap().get_scope();

        //add sub implement
        {
            let sub_implements = implement_scope.read().unwrap().instances.clone();
            for (name, instance) in sub_implements {
                let implement_of_instance = instance.read().unwrap().get_implement_type().get_raw_value();
                let mut new_hierarchy = hierarchy.clone();
                new_hierarchy.push(name.clone());
                self.add_implement_to_circuit_graph(implement_of_instance.clone(), &new_hierarchy)?;
            }
        }

        //add ports
        {
            let streamlet = implement.read().unwrap().get_derived_streamlet().unwrap();
            let streamlet_scope = streamlet.read().unwrap().get_scope();
            let ports = streamlet_scope.read().unwrap().ports.clone();
            let mut ports_simu = BTreeMap::new();
            for (name, port) in ports {
                let mut port_simu = circuit_port::Port::new();
                port_simu.set_name(name.clone());
                port_simu.set_parent_implement(Some(implement_simu.clone()));

                let clockdomain_var = port.read().unwrap().get_clock_domain();
                if self.clockdomain_mapping.is_none() { panic!("clockdomain mapping is not set"); }
                let clockdomain = circuit_frequency::SimulationClockDomain::convert_from_ast_clockdomain_var(clockdomain_var.read().unwrap().get_var_value().get_raw_value(), self.clockdomain_mapping.clone().unwrap());
                if clockdomain.is_err() { return Err(SimulatorError::ConfigError(clockdomain.err().unwrap())); }
                port_simu.set_clock_domain(clockdomain.ok().unwrap());

                ports_simu.insert(port_simu.get_name(), Arc::new(RwLock::new(port_simu)));
            }
            implement_simu.write().unwrap().set_ports(ports_simu);
        }

        //add connection
        {
            let connections = implement_scope.read().unwrap().connections.clone();
            for (_, ast_connection) in connections {
                let ast_src_port = ast_connection.read().unwrap().get_lhs_port().get_raw_value();
                let ast_src_port_owner = ast_connection.read().unwrap().get_lhs_port_owner();
                let (src_port, src_port_bind_direction) = self.find_corresponding_simulation_port(implement_simu.clone(), ast_src_port.clone(), &ast_src_port_owner).expect("Bug: we should always find the corresponding port");

                let ast_sink_port = ast_connection.read().unwrap().get_rhs_port().get_raw_value();
                let ast_sink_port_owner = ast_connection.read().unwrap().get_rhs_port_owner();
                let (sink_port, sink_port_bind_direction) = self.find_corresponding_simulation_port(implement_simu.clone(), ast_sink_port.clone(), &ast_sink_port_owner).expect("Bug: we should always find the corresponding port");

                let connection = self.add_single_connection(ast_connection.clone(), src_port.clone(), sink_port.clone()).expect("Bug: we should never meet an error here");

                fn add_connection_for_port(port: Arc<RwLock<circuit_port::Port>>, port_bind_direction: &PortBindDirection, connection: Arc<RwLock<circuit_connection::Connection>>) {
                    match port_bind_direction {
                        PortBindDirection::Unknown => { unreachable!() }
                        PortBindDirection::Internal => {
                            let mut port_write = port.write().unwrap();
                            assert!(port_write.get_bind_internal_connection().is_none());
                            port_write.set_bind_internal_connection(Some(connection.clone()));
                        }
                        PortBindDirection::External => {
                            let mut port_write = port.write().unwrap();
                            assert!(port_write.get_bind_external_connection().is_none());
                            port_write.set_bind_external_connection(Some(connection.clone()));
                        }
                    }
                }

                add_connection_for_port(src_port.clone(), &src_port_bind_direction, connection.clone());
                add_connection_for_port(sink_port.clone(), &sink_port_bind_direction, connection.clone());
            }
        }

        //add to graph
        let insert_result = self.implements.insert(implement_simu.read().unwrap().get_name(), implement_simu.clone());
        if insert_result.is_some() { panic!("insert on an existing key, consider naming bug in circuit graph") }

        return Ok(());
    }
}

pub fn tydi_raw_ast_to_circuit(project: Arc<RwLock<Project>>, top_level_implement_name: String, top_level_implement_package: String, clockdomain_mapping: &circuit_frequency::SimulationClockDomainMapping) -> Result<Arc<RwLock<CircuitGraph>>, SimulatorError> {
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
    output.write().unwrap().set_clockdomain_mapping(Some(clockdomain_mapping.clone()));

    let mut hierarchy = vec![top_level_implement_name.clone()];
    {
        let result = output.write().unwrap().add_implement_to_circuit_graph(target_implement.clone(), &hierarchy);
        if result.is_err() { return Err(result.err().unwrap()); }
    }

    return Ok(output);
}

