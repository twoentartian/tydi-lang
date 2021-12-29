mod id;
mod database;


#[salsa::database(crate::database::ProjectSpace)]
#[derive(Default)]
pub struct Database {
    storage: salsa::Storage<Database>,
}

impl salsa::Database for Database {

}
