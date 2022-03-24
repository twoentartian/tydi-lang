use serde::{Deserialize, Serialize};
use err::SimulatorError;

// 1ns => 1 GHz
// 1ps => 1000 GHz

#[derive(Debug, Deserialize, Serialize, Clone)]
#[allow(non_camel_case_types)]
#[serde(tag="signal")]
pub enum SimulatorConfig_Signal {
    Pulse {
        time: String,
        value: String,
    },
    Periodic {
        phase: String,
        period: String,
        value: String,
    },
}

impl SimulatorConfig_Signal {
    pub fn new_pulse() -> Self {
        return Self::Pulse {
            time: format!("0ns"),
            value: format!("0b00000000"),
        };
    }

    pub fn new_periodic() -> Self {
        return Self::Periodic {
            phase: format!("5ns"),
            period: format!("10ns"),
            value: format!("0b00000000"),
        };
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[allow(non_camel_case_types)]
pub struct SimulatorConfig_Port {
    pub name: String,
    pub signals: Vec<SimulatorConfig_Signal>,
}

impl SimulatorConfig_Port {
    pub fn new() -> Self {
        return Self {
            name: format!("port_default"),
            signals: vec![],
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SimulatorClockDomainMapping {
    pub name: String,
    pub expression: String,
    pub phase: String,
}

impl SimulatorClockDomainMapping {
    pub fn new() -> Self {
        return Self {
            name: format!("default"),
            expression: format!("1ns"), // 1ns or 1GHz both are correct
            phase: format!("500ps"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SimulatorConfig {
    pub simulation_step: String,
    pub simulation_stop: String,
    pub top_level_implement: String,
    pub top_level_implement_package: String,
    pub top_level_implement_input_signals: Vec<SimulatorConfig_Port>,
    pub clockdomain_mapping: Vec<SimulatorClockDomainMapping>,
}

impl SimulatorConfig {
    pub fn new() -> Self {
        return Self {
            simulation_step: format!("100ps"),
            simulation_stop: format!("100ns"),
            top_level_implement: format!("TopLevelImplement_name"),
            top_level_implement_package: format!("TopLevelImplement_package_name"),
            top_level_implement_input_signals: vec![],
            clockdomain_mapping: vec![],
        }
    }
}

pub fn generate_default_config_json() -> String {
    let mut default_config = SimulatorConfig::new();

    //top level implement
    let mut signal = SimulatorConfig_Signal::new_pulse();
    let mut port = SimulatorConfig_Port::new();
    port.name = format!("port_a");
    port.signals.push(signal.clone());
    signal = SimulatorConfig_Signal::Pulse {
        time: format!("1ns"),
        value: format!("0b00001010"),
    };
    port.signals.push(signal.clone());
    let mut signal = SimulatorConfig_Signal::new_periodic();
    port.signals.push(signal.clone());
    default_config.top_level_implement_input_signals.push(port);

    //clockdomain mapping
    let mut default_clockdomain = SimulatorClockDomainMapping::new();
    default_config.clockdomain_mapping.push(default_clockdomain.clone());
    default_clockdomain.name = format!("clockdomain_name_1");
    default_clockdomain.expression = format!("10ns");
    default_clockdomain.phase = format!("0ns");
    default_config.clockdomain_mapping.push(default_clockdomain.clone());

    let output = serde_json::to_string_pretty(&default_config).unwrap();
    return output;
}

pub fn parse_config(json: String) -> Result<SimulatorConfig, SimulatorError> {
    let parse_result: Result<SimulatorConfig, serde_json::Error> = serde_json::from_str(&json);
    if parse_result.is_err() {
        return Err(SimulatorError::JsonError(parse_result.err().unwrap()));
    }
    let parse_result = parse_result.ok().unwrap();
    return Ok(parse_result);
}
