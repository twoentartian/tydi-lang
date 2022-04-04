/*
pub const STD_LIB_PACKAGE_NAME: Option<String> = None; // works for project which only has one package
pub const STD_LIB_PACKAGE_NAME: Option<String> = Some(xxx); // works for project which contains more than one packages
 */
pub const STD_LIB_PACKAGE_NAME: Option<String> = None;

/*
external impl void_i<type_in: type> of void_s<type type_in> {};
 */
pub const STD_VOID_IMPL_NAME: &str = "void_i";

/*
external impl duplicator_i<data_type: type, output_channel: int> of duplicator_s<type data_type, output_channel> {};
*/
pub const STD_DUPLICATOR_IMPL_NAME: &str = "duplicator_i";