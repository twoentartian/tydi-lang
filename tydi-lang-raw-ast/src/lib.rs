mod util;
mod error;
mod project_arch;
mod scope;
mod variable;
mod data_type;
mod logical_data_type;
mod group_union_type;
mod steam_type;
mod bit_null_type;
mod streamlet;
mod port;
mod implement;
mod type_alias;
mod inferable;
mod instances;

#[macro_use]
extern crate lazy_static;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
