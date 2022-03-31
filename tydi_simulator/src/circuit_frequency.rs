use std::collections::HashMap;
use tydi_lang_raw_ast::scope::{ClockDomainValue, VariableValue};
use regex::{Regex, RegexSet};
use lazy_static::lazy_static;
use simulator_config_file::ConfigItem_SimulatorConfig;

#[derive(Debug, Clone)]
pub struct SimulationClockDomain {
    time_ps : u64,  //ps
    frequency: u64, //Hz
    phase_ps: u64,
}

impl SimulationClockDomain {
    pub fn new() -> Self {
        return Self {
            time_ps: 0,
            frequency: 0,
            phase_ps: 0,
        }
    }

    pub fn convert_from_ast_clockdomain_var(clock_domain: VariableValue, clockdomain_mapping: SimulationClockDomainMapping) -> Result<Self, String> {
        match clock_domain {
            VariableValue::ClockDomain(x) => {
                match x {
                    ClockDomainValue::Unknown => { unreachable!() }
                    ClockDomainValue::Default => {
                        let result = clockdomain_mapping.mapping.get("default");
                        if result.is_none() { return Err(format!("{} clockdomain is not found", "default")); }
                        let result = result.unwrap();
                        return Ok(result.clone());
                    }
                    ClockDomainValue::ClockDomain(cd) => {
                        let result = clockdomain_mapping.mapping.get(&cd);
                        if result.is_none() { return Err(format!("{} clockdomain is not found ", cd.clone())); }
                        let result = result.unwrap();
                        return Ok(result.clone());
                    }
                }
            }
            _ => unreachable!("only clockdomain values can use this function")
        }
    }
}

#[derive(Debug, Clone)]
pub struct SimulationClockDomainMapping {
    pub mapping: HashMap<String, SimulationClockDomain>,
}

impl SimulationClockDomainMapping {
    pub fn new() -> Self {
        return Self {
            mapping: HashMap::new(),
        };
    }

    fn convert_to_time_ps(exp: String) -> Result<u64, String> {
        lazy_static! {
            static ref ps_RE: RegexSet = RegexSet::new(&[
                r"(\d)ps",
                r"(\d)ns",
                r"(\d)us",
                r"(\d)ms",
                r"(\d)s",
            ]).unwrap();
            static ref hz_RE: RegexSet = RegexSet::new(&[
                r"(\d)Hz",
                r"(\d)kHz",
                r"(\d)MHz",
                r"(\d)GHz",
            ]).unwrap();
        }
        let matches: Vec<_> = ps_RE.matches(&exp).into_iter().collect();
        if matches.len() > 1 { return Err(format!("{} is an invalid expression", exp.clone())); }
        if matches.len() == 1 {
            let pow_index = matches[0];
            let factor: u64 = 1000_u64.pow(pow_index as u32);
            let mut exp_clone = exp.clone();
            exp_clone = exp_clone.chars().filter(|c| c.is_digit(10)).collect();
            let number = exp_clone.parse::<u64>();
            if number.is_err() { return Err(number.err().unwrap().to_string()); }
            let number = number.ok().unwrap();
            return Ok(number * factor);
        }

        let matches: Vec<_> = hz_RE.matches(&exp).into_iter().collect();
        if matches.len() > 1 { return Err(format!("{} is an invalid expression", exp.clone())); }
        if matches.len() == 1 {
            let pow_index = matches[0];
            let factor: u64 = 1000_u64.pow(pow_index as u32);
            let mut exp_clone = exp.clone();
            exp_clone = exp_clone.chars().filter(|c| c.is_digit(10)).collect();
            let number = exp_clone.parse::<u64>();
            if number.is_err() { return Err(number.err().unwrap().to_string()); }
            let number = number.ok().unwrap();
            let hz = number * factor;
            let ps = 1_000_000_000_000_000_u64 / hz;
            return Ok(ps);
        }

        return Err(format!("{} is an invalid expression", exp.clone()));
    }

    pub fn convert_from_configure_file_item(&mut self, clockdomain_name: &String, frequency_expression: &String, phase: &String) -> Result<(), String> {
        let mut clockdomain_simulation = SimulationClockDomain::new();

        clockdomain_simulation.time_ps = SimulationClockDomainMapping::convert_to_time_ps(frequency_expression.clone())?;
        clockdomain_simulation.phase_ps = SimulationClockDomainMapping::convert_to_time_ps(phase.clone())?;
        clockdomain_simulation.frequency = 1_000_000_000_000_000_u64 / clockdomain_simulation.time_ps;

        self.mapping.insert(clockdomain_name.clone(), clockdomain_simulation);

        return Ok(());
    }

    pub fn convert_from_configure_file(&mut self, config: &ConfigItem_SimulatorConfig) -> Result<(), String> {
        self.mapping.clear();
        for single_cd in config.clockdomain_mapping.clone() {
            self.convert_from_configure_file_item(&single_cd.name, &single_cd.expression, &single_cd.phase)?;
        }

        return Ok(());
    }
}