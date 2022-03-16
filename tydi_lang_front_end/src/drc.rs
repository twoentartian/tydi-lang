use std::sync::{RwLock, Arc};
use tydi_lang_raw_ast::generate_get;
use tydi_lang_raw_ast::project_arch::Project;
use tydi_lang_raw_ast::scope::{Connection, PortDirection, PortOwner};

#[derive(Clone, Debug)]
pub enum DrcResultType {
    Error,
    Warning,
}

#[derive(Clone, Debug)]
pub enum DesignRuleDetail {
    #[allow(non_camel_case_types)]
    InvalidConnectionPortTypeMismatch_StrictChecking,
    #[allow(non_camel_case_types)]
    InvalidConnectionPortTypeMismatch_NonStrictChecking,
    InvalidConnectionWrongPortDirection,
}

#[derive(Clone, Debug)]
pub struct DesignRuleErrorWarning {
    message: String,
    err_warn_type: DrcResultType,
    detail: DesignRuleDetail,
}

impl DesignRuleErrorWarning {
    generate_get!(message, String, get_message);
    generate_get!(err_warn_type, DrcResultType, get_err_warn_type);
    generate_get!(detail, DesignRuleDetail, get_detail_type);

    pub fn new(err_warn: DrcResultType, detail_type: DesignRuleDetail, msg: String) -> Self {
        return Self {
            message: msg,
            err_warn_type: err_warn,
            detail: detail_type,
        }
    }
}

pub fn tydi_design_rule_check_connection(connection: Arc<RwLock<Connection>>) -> Vec<DesignRuleErrorWarning> {
    let mut output = vec![];

    //check port type
    let lhs_port = connection.read().unwrap().get_lhs_port().get_raw_value();
    let lhs_port_type = lhs_port.read().unwrap().get_type().get_raw_value();
    let rhs_port = connection.read().unwrap().get_rhs_port().get_raw_value();
    let rhs_port_type = rhs_port.read().unwrap().get_type().get_raw_value();
    if connection.read().unwrap().get_check_restrict_type_same() {
        //strict type eq
        if *lhs_port_type.read().unwrap() != *rhs_port_type.read().unwrap() {
            //try non strict type eq
            if !lhs_port_type.read().unwrap().non_strict_eq(&*rhs_port_type.read().unwrap()) {
                output.push(DesignRuleErrorWarning::new (DrcResultType::Error, DesignRuleDetail::InvalidConnectionPortTypeMismatch_StrictChecking, format!("connection name: {}, both strict and non-strict eq tests fail", connection.read().unwrap().get_name())));
            }
            else {
                output.push(DesignRuleErrorWarning::new (DrcResultType::Warning, DesignRuleDetail::InvalidConnectionPortTypeMismatch_StrictChecking, format!("connection name: {}, strict eq test fails but non-strict eq test pass, considering adding \"@NoStrictType@\" at end of connection ?", connection.read().unwrap().get_name())));
            }

        }
    }
    else {
        if !lhs_port_type.read().unwrap().non_strict_eq(&*rhs_port_type.read().unwrap()) {
            output.push(DesignRuleErrorWarning::new (DrcResultType::Error, DesignRuleDetail::InvalidConnectionPortTypeMismatch_NonStrictChecking, format!("connection name: {}, non-strict eq test fails.", connection.read().unwrap().get_name())));
        }
    }

    //check port direction
    let lhs_port_owner = connection.read().unwrap().get_lhs_port_owner();
    let rhs_port_owner = connection.read().unwrap().get_rhs_port_owner();
    let mut lhs_port_direction = lhs_port.read().unwrap().get_direction();
    let mut rhs_port_direction = rhs_port.read().unwrap().get_direction();
    match lhs_port_owner {
        PortOwner::SelfOwner => {
            lhs_port_direction = lhs_port_direction.toggle_direction();
        },
        _ => {}
    }
    match rhs_port_owner {
        PortOwner::SelfOwner => {
            rhs_port_direction = rhs_port_direction.toggle_direction();
        },
        _ => {}
    }

    if lhs_port_direction != PortDirection::Output {
        output.push(DesignRuleErrorWarning::new (DrcResultType::Error, DesignRuleDetail::InvalidConnectionWrongPortDirection, format!("connection name: {}, lhs port is not output", connection.read().unwrap().get_name())));
    }
    if rhs_port_direction != PortDirection::Input {
        output.push(DesignRuleErrorWarning::new (DrcResultType::Error, DesignRuleDetail::InvalidConnectionWrongPortDirection, format!("connection name: {}, rhs port is not input", connection.read().unwrap().get_name())));
    }

    return output;
}

pub fn tydi_design_rule_check(project: Arc<RwLock<Project>>) -> Vec<DesignRuleErrorWarning> {
    let mut output = vec![];
    for (_, package) in project.read().unwrap().packages.clone() {
        let package_scope = package.read().unwrap().scope.clone();
        let impls = package_scope.read().unwrap().implements.clone();
        for (_, single_impl) in impls {
            let impl_scope = single_impl.read().unwrap().get_scope();
            let connections = impl_scope.read().unwrap().connections.clone();
            for (_,single_connection) in connections {
                let mut msgs = tydi_design_rule_check_connection(single_connection.clone());
                output.append(&mut msgs);
            }
        }
    }

    return output;
}