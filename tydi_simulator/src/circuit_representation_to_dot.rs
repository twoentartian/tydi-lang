use circuit_representation::CircuitGraph;
use to_dot::ToDot;

fn convert_name_to_accepted_dot_name(name: String) -> String {
    let mut output = name.clone();
    output = output.replace("::", "__");
    output = output.replace("@", "_AT_");
    return output;
}

impl ToDot for CircuitGraph {
    fn to_dot(&self) -> String {
        let mut output = String::from("");

        let all_implements = self.get_implements();
        for (implement_name, implement) in all_implements {
            let mut implement_output_line = String::from("");
            let all_ports = implement.read().unwrap().get_ports();

            for (port_name, _) in all_ports {
                implement_output_line.push_str("|");
                implement_output_line.push_str(&format!("<{}>{}", convert_name_to_accepted_dot_name(port_name.clone()), port_name.clone()));
            }
            let impl_name = convert_name_to_accepted_dot_name(implement_name.clone());
            let implement_color = if !implement.read().unwrap().get_external_implement_flag() {"color=red, "} else { "" };
            implement_output_line = format!("{} [{}shape=record, label=\"{{<component>{}{}}}\"];\n", impl_name.clone(), implement_color, impl_name.clone(), implement_output_line);
            output.push_str(&implement_output_line);
        }

        let all_connections = self.get_connections();
        for (connection_name, connection) in all_connections {
            let mut connection_output_line = String::from("");
            let src_port = connection.read().unwrap().get_src_port();
            let sink_port = connection.read().unwrap().get_sink_port();
            if src_port.is_none() || sink_port.is_none() { unreachable!("connection should never meet none port") }
            let src_port = src_port.unwrap();
            let sink_port = sink_port.unwrap();
            let src_implement = src_port.read().unwrap().get_parent_implement().unwrap();
            let sink_implement = sink_port.read().unwrap().get_parent_implement().unwrap();
            let src_implement_name = convert_name_to_accepted_dot_name(src_implement.read().unwrap().get_name());
            let sink_implement_name = convert_name_to_accepted_dot_name(sink_implement.read().unwrap().get_name());
            let src_port_name = convert_name_to_accepted_dot_name(src_port.read().unwrap().get_name());
            let sink_port_name = convert_name_to_accepted_dot_name(sink_port.read().unwrap().get_name());
            connection_output_line = format!("{}:{} -> {}:{} [label=\"{}\"] ;\n", src_implement_name, src_port_name, sink_implement_name, sink_port_name, connection_name);
            output.push_str(&connection_output_line);
        }

        output = format!("digraph {{\n\
        {}\n\
        }}\n", output);
        return output;
    }
}

