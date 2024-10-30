use diesel::prelude::SqliteConnection;
use diesel::Connection;
use dotenvy::dotenv;
use std::env;


pub mod models;
pub mod schema;
pub mod tenants;
pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("error connecting to {}", database_url))
}

#[cfg(test)]

mod tests {
    use super::*;
    use crate::models::Tenant;
    use crate::tenants::get_all_tenants;
    use crate::tenants;
    use rusqlite::{Connection as ManualConnection, Result};

    struct TestContext {
        connection: ManualConnection,
    }

    impl TestContext {
        fn new() -> Self {
            println!("setting up resources");
            let connection = Self::create_new_test_database().unwrap();

            Self {
                connection,
            }
        }

        fn create_new_test_database() -> Result<ManualConnection> {
            let conn = ManualConnection::open("/home/lars/workspaces/moro/lrar/backend/db/database.db")?;
            // embedded_migrations::run(&conn);
            conn.execute("CREATE TABLE tenants (
                id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
                name VARCHAR NOT NULL,
                height INTEGER NOT NULL,
                profile_picture_uri VARCHAR NOT NULL,
                burns INTEGER NOT NULL);", ())?;
            Ok(conn)
        }
    }
    impl Drop for TestContext {
        fn drop(&mut self) -> () {
            self.connection.execute("DROP TABLE tenants", ()).unwrap();
            println!("cleaning up resources");
        }
    }


    #[test]
    fn should_push_new_tenant_to_database() {
        let _test_context = TestContext::new();
        create_new_user();
        let mut statement = _test_context.connection.prepare("SELECT count(*) FROM tenants").unwrap();
        let result = statement.column_count();
        println!("{:?}", result);
        assert_eq!(1, result);
    }

    fn create_new_user() -> Tenant {
        let new_user: Tenant = Tenant {
            name: "test".to_string(),
            height: 20,
            profile_picture_uri: "TEST".to_string(),
            burns: 0,
        };
        new_user
    }

    #[test]
    fn should_get_all_tenants() {
        let _test_context = TestContext::new();
        let test_user = create_new_user();
        tenants::create_tenant(test_user);
        let tenants_in_database = get_all_tenants();
        assert_eq!(1, tenants_in_database.len());
    }


}