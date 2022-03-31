extern crate tydi_lang_raw_ast;
extern crate tydi_lang_parser;
extern crate tydi_lang_front_end;
extern crate serde_json;
extern crate serde;
extern crate regex;
extern crate lazy_static;

use lazy_static::lazy_static;
use regex::RegexSet;

mod util;
mod err;
mod to_dot;
mod simulator_config_file;
mod circuit_representation;
mod circuit_representation_to_dot;
mod circuit_representation_remove_bypass;
mod circuit_implement;
mod circuit_port;
mod circuit_connection;
mod circuit_frequency;

mod test;


fn main() {
    lazy_static! {
            static ref ps_RE: RegexSet = RegexSet::new(&[
                r"(\d)ps",
                r"(\d)ns",
                r"(\d)us",
                r"(\d)ms",
                r"(\d)s",
            ]).unwrap();
        }
    let matches: Vec<_> = ps_RE.matches("2s").into_iter().collect();
    if matches.len() != 1 { return; }
    let pow_index = matches[0];
    let factor: u64 = 1000_u64.pow(pow_index as u32);

    print!("{}", factor);
}

