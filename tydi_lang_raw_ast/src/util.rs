
#[macro_export]
macro_rules! generate_set {
    ($id:ident, $t: ty, $id_set_func:ident) => {
        #[allow(dead_code)]
        pub fn $id_set_func(&mut self, target: $t) {
            self.$id = target;
        }
    };
}

#[macro_export]
macro_rules! generate_get {
    ($id:ident, $t: ty, $id_get_fun:ident) => {
        #[allow(dead_code)]
        pub fn $id_get_fun(& self) -> $t {
            return self.$id.clone();
        }
    };
}

#[macro_export]
macro_rules! generate_access {
    ($id:ident, $t: ty, $id_get_fun:ident, $id_set_func:ident) => {
        generate_set!($id, $t, $id_set_func);
        generate_get!($id, $t, $id_get_fun);
    };
}

#[macro_export]
macro_rules! generate_set_in_arc_rwlock {
    ($id:ident, $t: ty, $id_set_func:ident) => {
        pub fn $id_set_func(&mut self, target: $t) {
            self.$id = Arc::new(RwLock::new(target));
        }
    };
}

pub trait PrettyPrint {
    fn pretty_print(&self, depth: u32, verbose: bool) -> String;
}

pub trait EnableDocument {
    fn get_document(&self) -> Option<String>;
    fn set_document(&mut self, document: Option<String>);
}

pub fn generate_padding(depth: u32) -> String {
    return str::repeat("  ", depth as usize);
}

pub fn generate_random_str(length: usize) -> String {
    use rand::{thread_rng, Rng};
    use rand::distributions::Alphanumeric;
    let rand_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(length)
            .map(char::from)
            .collect();
    return rand_string;
}

pub fn rename_id_to_il(name: String) -> String {
    let output = name.clone();
    let output = output.replace("@", "AT");
    let output = output.replace("(", "UU");
    let output = output.replace(")", "UU");
    return output;
}