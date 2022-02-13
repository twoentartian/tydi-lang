pub mod util;
pub mod error;
pub mod project_arch;
pub mod scope;
pub mod variable;
pub mod data_type;
pub mod logical_data_type;
pub mod group_union_type;
pub mod steam_type;
pub mod bit_null_type;
pub mod streamlet;
pub mod port;
pub mod implement;
pub mod type_alias;
pub mod inferable;
pub mod instances;
pub mod connection;
pub mod if_for;
pub mod deep_clone;

mod test_print_project;

#[macro_use]
extern crate lazy_static;
extern crate rand;
