use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};
use deep_clone::DeepClone;
use evaluated_flag::{EvaluatedFlag, EvaluatedState};
use evaluated_flag::EvaluatedState::NotEvaluate;
use crate::data_type::DataType;
use crate::error::ErrorCode;
use crate::{generate_get, generate_access, generate_set};
use crate::inferable::Inferable;
use crate::port::{Port};
use crate::scope::{Scope, ScopeRelationType, ScopeType};
use crate::streamlet::Streamlet;
use crate::util::{generate_padding, PrettyPrint, EnableDocument};
use crate::variable::Variable;
use crate::tydi_il;
use derived_macro::EnableDocument;

#[derive(Clone, Debug)]
pub enum ImplementType {
    UnknownType,
    NormalImplement,
    AnyImplementOfStreamlet(String, Option<Arc<RwLock<Streamlet>>>),
    TemplateImplement(Vec<Arc<RwLock<Variable>>>),
    DummyImplement,
}

impl DeepClone for ImplementType {
    fn deep_clone(&self) -> Self {
        return match self {
            // not deep clone for ImplementType::AnyImplementOfStreamlet because parent streamlet shouldn't be cloned
            ImplementType::AnyImplementOfStreamlet(name, streamlet) => { ImplementType::AnyImplementOfStreamlet(name.deep_clone(), streamlet.clone()) }
            ImplementType::TemplateImplement(template_exps) => { ImplementType::TemplateImplement(template_exps.deep_clone()) }
            _ => { self.clone() }
        }
    }
}

impl From<ImplementType> for String {
    fn from(type_: ImplementType) -> Self {
        match type_ {
            ImplementType::UnknownType => { return String::from("UnknownType"); },
            ImplementType::NormalImplement => { return String::from("NormalImplement"); },
            ImplementType::AnyImplementOfStreamlet(s, _) => { return format!("AnyImplementOfStreamlet({})", s.clone()); },
            ImplementType::TemplateImplement(vars) => {
                let mut output = String::from("");
                for v in vars {
                    let type_ = v.read().unwrap().get_type();
                    output.push_str(&format!("@{}", String::from((*(type_.read().unwrap())).clone()) ));
                }
                return output;
            },
            ImplementType::DummyImplement => { return String::from("DummyImplement"); },
        }
    }
}

impl PrettyPrint for ImplementType {
    fn pretty_print(&self, _: u32, _: bool) -> String {
        return String::from(self.clone());
    }
}

#[derive(Clone, Debug, EnableDocument)]
pub struct Implement {
    name: String,

    implement_type: ImplementType,
    scope: Arc<RwLock<Scope>>,

    derived_streamlet_var: Arc<RwLock<Variable>>,
    derived_streamlet: Option<Arc<RwLock<Streamlet>>>,

    evaluated_state: EvaluatedState,
    simulation_process: Option<String>,
    docu: Option<String>,

    parent_implement_ref: Option<Arc<RwLock<Implement>>>,
}

impl DeepClone for Implement {
    fn deep_clone(&self) -> Self {
        let output = Self {
            name: self.name.deep_clone(),

            implement_type: self.implement_type.deep_clone(),
            scope: self.scope.deep_clone(),
            derived_streamlet_var: self.derived_streamlet_var.deep_clone(),
            derived_streamlet: self.derived_streamlet.deep_clone(),

            evaluated_state: self.evaluated_state.deep_clone(),
            simulation_process: self.simulation_process.deep_clone(),

            docu: self.docu.deep_clone(),

            parent_implement_ref: self.parent_implement_ref.clone(),//we use shallow clone here because parent implement should not change.
        };
        {
            output.scope.write().unwrap().set_self_ref(output.scope.clone());
        }

        //change the scope relationship of if & for scope
        {
            let if_blocks = output.scope.read().unwrap().if_blocks.clone();
            for (_, if_block) in if_blocks {
                let if_block_read = if_block.read().unwrap();
                let if_block_scope = if_block_read.get_scope();
                let mut scope_write = if_block_scope.write().unwrap();
                for (_, rela) in &mut scope_write.scope_relationships {
                    rela.set_target_scope(output.scope.clone());
                }
            }
            let for_blocks = output.scope.read().unwrap().for_blocks.clone();
            for (_, for_block) in for_blocks {
                let for_block_read = for_block.read().unwrap();
                let for_block_scope = for_block_read.get_scope();
                let mut scope_write = for_block_scope.write().unwrap();
                for (_, rela) in &mut scope_write.scope_relationships {
                    rela.set_target_scope(output.scope.clone());
                }
            }
        }
        return output;
    }
}

impl tydi_il::ToTydiIL for Implement {
    fn to_tydi_il(&self, type_alias_map: &mut HashMap<String, (String, Vec<String>)>, depth:u32) -> String {
        let mut output = String::from("");

        //document
        let docu_str = match &self.docu {
            None => { String::from("") }
            Some(docu) => { format!("{}", docu) }
        };
        let streamlet = self.derived_streamlet.as_ref().unwrap().read().unwrap();
        let streamlet_docu = match streamlet.get_document() {
            None => { String::from("") }
            Some(docu) => { format!("{}", docu) }
        };

        //streamlet_ports
        let streamlet_ports = streamlet.get_scope().read().unwrap().ports.clone();
        let mut streamlet_port_content = String::from("");
        for (_,port) in streamlet_ports {
            let str = port.read().unwrap().to_tydi_il(type_alias_map, depth+1);
            streamlet_port_content.push_str(&format!("{},\n", str));
        }

        //instance
        let instances = self.scope.read().unwrap().instances.clone();
        let mut instance_content = String::from("");
        for (_,instance) in instances {
            let str = instance.read().unwrap().to_tydi_il(type_alias_map, depth+2);
            instance_content.push_str(&format!("{};\n", str));
        }

        //connections
        let connections = self.scope.read().unwrap().connections.clone();
        let mut connection_content = String::from("");
        for (_,connection) in connections {
            let str = connection.read().unwrap().to_tydi_il(type_alias_map, depth+2);
            connection_content.push_str(&format!("{};\n", str));
        }

        //// expand streamlet and print it again in implement
        // output.push_str(
        //     &format!("\
        // {}\
        // {}streamlet {} = (\n\
        //   {}\
        // {}) {{\n\
        // {}\
        // {}impl:{{\n\
        // {}\
        // {}\
        // {}}},\n\
        // {}}};\n\
        // ",
        //              streamlet_docu,
        //              generate_padding(depth), crate::util::rename_id_to_il(self.name.clone()),
        //              streamlet_port_content,
        //              generate_padding(depth),
        //              &docu_str,
        //              generate_padding(depth + 1),
        //              instance_content,
        //              connection_content,
        //              generate_padding(depth + 1),
        //              generate_padding(depth)));

        //// don't expand streamlet and use streamlet reference
        output.push_str(
            &format!("\
        {}{}\n\
        {}streamlet {} = {} {{\n\
        {}impl:{}\n\
        {}{{\n\
        {}\
        {}\
        {}}},\n\
        {}}};\n\
        ",
                     generate_padding(depth), streamlet_docu,
                     generate_padding(depth), crate::util::rename_id_to_il(self.name.clone()), crate::util::rename_id_to_il(streamlet.get_name()),
                     generate_padding(depth + 1), &docu_str,
                     generate_padding(depth + 1),
                     instance_content,
                     connection_content,
                     generate_padding(depth + 1),
                     generate_padding(depth)));

        return output;
    }
}

impl EvaluatedFlag for Implement {
    fn get_evaluate_flag(&self) -> EvaluatedState {
        return self.evaluated_state.clone();
    }

    fn set_evaluate_flag(&mut self, flag: EvaluatedState) {
        self.evaluated_state = flag;
    }
}

impl Implement {
    generate_get!(name, String, get_name);
    generate_access!(implement_type, ImplementType, get_type, set_type);
    generate_get!(scope, Arc<RwLock<Scope>>, get_scope);
    generate_access!(derived_streamlet_var, Arc<RwLock<Variable>>, get_derived_streamlet_var, set_derived_streamlet_var);
    generate_access!(derived_streamlet, Option<Arc<RwLock<Streamlet>>>, get_derived_streamlet, set_derived_streamlet);
    generate_access!(simulation_process, Option<String>, get_simulation_process, set_simulation_process);
    generate_access!(parent_implement_ref, Option<Arc<RwLock<Implement>>>, get_parent_ref, set_parent_ref);

    //find the most elder parent implement
    fn get_parent_implement(&self) -> Option<Arc<RwLock<Implement>>> {
        return match self.parent_implement_ref.clone() {
            None => { None }
            Some(parent_ref) => {
                if parent_ref.read().unwrap().parent_implement_ref.is_none() {
                    Some(parent_ref.clone())
                } else {
                    parent_ref.read().unwrap().get_parent_implement()
                }
            }
        }
    }

    pub fn get_instance_impl_dependency(&self) -> Vec<String> {
        let mut output = vec![];
        let instances = self.scope.read().unwrap().instances.clone();
        for (_, instance) in instances {
            let instance_impl = instance.read().unwrap().get_implement_type().get_raw_value();
            let has_parent_ref = instance_impl.read().unwrap().get_parent_implement();
            match has_parent_ref {
                None => {
                    output.push(instance_impl.read().unwrap().get_name());
                }
                Some(has_parent_ref) => {
                    output.push(has_parent_ref.read().unwrap().get_name());
                }
            }
        }
        return output;
    }

    pub fn set_name(&mut self, name_: String) {
        self.name = name_.clone();
        self.scope.write().unwrap().set_name(format!("implement_{}", name_.clone()));
    }

    pub fn new(name_: String, type_: ImplementType) -> Self {
        let scope_ = Arc::new(RwLock::new(Scope::new(format!("implement_{}", name_.clone()), ScopeType::ImplementScope)));
        {
            scope_.write().unwrap().set_self_ref(scope_.clone());
        }
        Self {
            name: name_,
            implement_type: type_,
            scope: scope_,

            derived_streamlet_var: Arc::new(RwLock::new(Variable::new(String::from(""), DataType::UnknownType, String::from("")))),
            derived_streamlet: None,

            evaluated_state: NotEvaluate,
            simulation_process: None,

            docu: None,
            parent_implement_ref: None,
        }
    }

    pub fn new_instance(& self, name_: String, package_: Option<String>, streamlet_: Inferable<Arc<RwLock<Implement>>>, template_argexp: Vec<Arc<RwLock<Variable>>>) -> Result<(), ErrorCode> {
        let mut scope = self.scope.write().unwrap();
        return scope.new_instance(name_.clone(), package_, streamlet_.clone(), template_argexp);
    }

    pub fn new_connection(& self, name_: String, lhs_port_: Inferable<Arc<RwLock<Port>>>, rhs_port_: Inferable<Arc<RwLock<Port>>>, delay_: Variable) -> Result<(), ErrorCode> {
        let mut scope = self.scope.write().unwrap();
        return scope.new_connection(name_.clone(), lhs_port_.clone(), rhs_port_.clone(), delay_.clone());
    }

    pub fn new_variable(&self, name_: String, type_: DataType, exp_: String) -> Result<(), ErrorCode> {
        let mut scope = self.scope.write().unwrap();
        return scope.new_variable(name_.clone(), type_.clone(), exp_.clone());
    }
}

impl From<Implement> for String {
    fn from(implement: Implement) -> Self {
        return format!("Implement({})", implement.get_name());
    }
}

impl PrettyPrint for Implement {
    fn pretty_print(&self, depth: u32, verbose: bool) -> String {
        let mut output = String::new();

        //enter Implement
        let derived_streamlet_var = self.derived_streamlet_var.read().unwrap().get_type();
        let derived_streamlet = self.derived_streamlet.clone();
        let derived_streamlet_representation = match derived_streamlet {
            None => String::from((*derived_streamlet_var.read().unwrap()).clone()),
            Some(streamlet) => String::from((*streamlet.read().unwrap()).clone()),
        };
        output.push_str(&format!("{}Implement({})<{}> -> {}{{\n", generate_padding(depth), self.name.clone(), String::from(self.implement_type.clone()), derived_streamlet_representation));
        //enter scope
        output.push_str(&format!("{}", self.scope.read().unwrap().pretty_print(depth+1, verbose)));
        //enter simulation process
        output.push_str(&format!("{}simulation_process{{{:?}}}\n", generate_padding(depth+1), self.simulation_process.clone()));
        //leave Implement
        output.push_str(&format!("{}}}", generate_padding(depth)));

        return output;
    }
}

impl Scope {
    pub fn new_implement(&mut self, name_: String, type_: ImplementType) -> Result<Arc<RwLock<Scope>>, ErrorCode> {
        if self.scope_type != ScopeType::BasicScope { return Err(ErrorCode::ScopeNotAllowed(format!("not allowed to define implement outside of base scope"))); }

        match self.implements.get(&name_) {
            None => {}
            Some(_) => { return Err(ErrorCode::IdRedefined(format!("implement {} already defined", name_.clone()))); }
        };

        let implement = Implement::new(name_.clone(), type_.clone());
        {
            let parent_scope = self.self_ref.clone().unwrap();
            implement.scope.write().unwrap().new_relationship_with_name(parent_scope.clone(), String::from("base"), ScopeRelationType::ImplementScopeRela);
        }

        let scope_copy = implement.scope.clone();
        let implement_box = Arc::new(RwLock::new(implement));
        self.implements.insert(name_.clone(), implement_box.clone());
        return Ok(scope_copy);
    }

    pub fn with_implement(&mut self, implement: Arc<RwLock<Implement>>) -> Result<(), ErrorCode> {
        if self.scope_type != ScopeType::BasicScope && self.scope_type != ScopeType::ImplementScope { return Err(ErrorCode::ScopeNotAllowed(format!("not allowed to define implement outside of base scope"))); }

        let name_ = implement.read().unwrap().get_name();
        match self.implements.get(&name_) {
            None => {}
            Some(_) => { return Err(ErrorCode::IdRedefined(format!("implement {} already defined", name_.clone()))); }
        };

        self.implements.insert(name_, implement);
        return Ok(());
    }

    pub fn resolve_implement_in_current_scope(& self, name_: String) -> Result<Arc<RwLock<Implement>>, ErrorCode> {
        return match self.implements.get(&name_) {
            None => { Err(ErrorCode::IdNotFound(format!("variable {} not found", name_))) }
            Some(var) => { Ok(var.clone()) }
        };
    }

    fn _resolve_implement_in_scope(target_scope: Arc<RwLock<Scope>>, name_: &String, allowed_relationships: &HashSet<ScopeRelationType>) -> Result<Arc<RwLock<Implement>>, ErrorCode> {
        let target_scope_r = target_scope.read().unwrap();

        //find self scope
        match target_scope_r.resolve_implement_in_current_scope(name_.clone()) {
            Ok(var) => { return Ok(var) }
            Err(_) => {}
        }

        //find in parent scope
        for (_, scope_real) in &(target_scope_r.scope_relationships) {
            let result = Scope::_resolve_implement_in_scope(scope_real.get_target_scope().clone(), &name_, &allowed_relationships);
            match result {
                Ok(var) => {return Ok(var)}
                Err(_) => {}
            }
        }

        return Err(ErrorCode::IdNotFound(format!("implement {} not found", name_.clone())));
    }

    pub fn resolve_implement_with_relation(& self, name_: String, allowed_relationships: Vec<ScopeRelationType>) -> Result<Arc<RwLock<Implement>>, ErrorCode> {
        match self.resolve_implement_in_current_scope(name_.clone()) {
            Ok(var) => { return Ok(var) }
            Err(_) => {}
        }

        let mut allowed_relationships_hash = HashSet::new();
        for allowed_relationship in allowed_relationships {
            allowed_relationships_hash.insert(allowed_relationship.clone());
        }

        //find in parent scope
        for (_, scope_real) in &(self.scope_relationships) {
            let result = Scope::_resolve_implement_in_scope(scope_real.get_target_scope().clone(), &name_, & allowed_relationships_hash);
            match result {
                Ok(var) => {return Ok(var)}
                Err(_) => {}
            }
        }

        return Err(ErrorCode::IdNotFound(format!("variable {} not found", name_.clone())));
    }

    pub fn resolve_implement_from_scope(& self, name_: String) -> Result<Arc<RwLock<Implement>>, ErrorCode> {
        use crate::scope::ScopeRelationType::*;
        let allowed_relationships = vec![GroupScopeRela, UnionScopeRela,
                                         StreamScopeRela, StreamletScopeRela, ImplementScopeRela];
        return self.resolve_implement_with_relation(name_, allowed_relationships);
    }
}