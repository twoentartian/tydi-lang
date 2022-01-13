mod id;
mod database;
mod identifier;
mod ident;

mod project_arch;
mod scope_var;
mod logical_data_type;

mod thin_wrapper;

#[salsa::database(crate::database::ProjectSpace)]
#[derive(Default)]
pub struct Database {
    storage: salsa::Storage<Database>,
}

impl salsa::Database for Database {

}


#[cfg(test)]
mod db_tests {
    use std::borrow::Borrow;
    use std::collections::BTreeMap;
    use std::fs::DirBuilder;
    use std::rc::Weak;
    use std::sync::Arc;
    use crate::database::*;
    use crate::Database;
    #[test]
    fn test_db_structure() {
        let mut db = Database::default();
        let package0_id = ident(&db, String::from("package0"));
        let package_id = db.intern_package(Package::new(package0_id));

        print!("package0_id = {}\n package_id = {}\n",package0_id, package_id);
        let search_package_result : Package = db.lookup_intern_package(package_id);
        println!("search_package_result.identifier = {}", search_package_result.identifier);
        let search_package_name = db.lookup_intern_identifier(search_package_result.identifier);
        println!("search_package_name = {}", search_package_name);
        assert_eq!(String::from(search_package_name), String::from("package0"));
    }

    #[test]
    fn test_project() {
        let mut db = Database::default();
        let project_id = ident(&db, String::from("project_genesis"));
        let mut project = Project::new(project_id);

        /// add package0
        {
            let package0_nameid = ident(&db, String::from("package0"));
            let package0_id = db.intern_package(Package::new(package0_nameid));
            project.with_package(String::from("package0"), package0_id);
            assert_eq!(project.packages.contains_key("package0"), true);
        }

        /// add package1
        {
            let package1_nameid = ident(&db, String::from("package1"));
            let package1_id = db.intern_package(Package::new(package1_nameid));
            project.with_package(String::from("package1"), package1_id);
            assert_eq!(project.packages.contains_key("package0"), true);
            assert_eq!(project.packages.contains_key("package1"), true);
            assert_eq!(project.packages.contains_key("package2"), false);
        }
        db.set_project(Arc::new(project));
        {
            let result : Arc<Project> = db.project();
            assert_eq!(result.packages.contains_key("package0"), true);
            assert_eq!(result.packages.contains_key("package1"), true);
            assert_eq!(result.packages.contains_key("package2"), false);
        }

        /// add package2
        {
            let result : Arc<Project> = db.project();
            let mut project = (*result).clone();
            let package2_nameid = ident(&db, String::from("package2"));
            let package2_id = db.intern_package(Package::new(package2_nameid));
            project.with_package(String::from("package2"), package2_id);
            assert_eq!(project.packages.contains_key("package2"), true);

            /// write back to db
            db.set_project(Arc::new(project));
        }
        {
            let result : Arc<Project> = db.project();
            assert_eq!(result.packages.contains_key("package2"), true);
        }

        /// access packages
        {
            let packages : Arc<BTreeMap<String,Id<Package>>> = db.all_packages();
            for (package_name, package_id) in &*packages {
                println!("{}", package_name);
            }
        }


    }
}