pub mod menubuilder {
    use std::collections::HashMap;
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

    use crate::builder::server::actix::start_server;
    use crate::builder::{Database, DatabaseManager};
    use crate::builder::{Screen as ScreenModel, Service as ServiceModel};
    use crate::core::USSDMenu;
    use crate::info;

    pub trait MenuBuilderTrait {
        fn new(service_code: &str) -> Self;
        fn build(&self) -> USSDMenu;
        fn to_json(&self, path: Option<&str>) -> ();
    }

    pub struct MenuBuilder {}

    impl MenuBuilder {
        fn build(&self) -> USSDMenu {
            let mut db = DatabaseManager::new();

            let mut menus = HashMap::new();
            let mut services = HashMap::new();

            let menu: Vec<ScreenModel> = db.get_many().expect("Failed to get screens");

            for m in menu {
                menus.insert(m.name.clone(), m.to_ussd_screen());
            }

            let service: Vec<ServiceModel> = db.get_many().expect("Failed to get services");

            for s in service {
                services.insert(s.name.clone(), s.to_ussd_service());
            }

            USSDMenu { menus, services }
        }

        pub fn to_json(&self, file_path: Option<&str>) {
            let menu = self.build();

            match file_path {
                Some(path) => {
                    let res = menu.save_to_json(path);
                    match res {
                        Ok(_) => println!("Menu saved to {}", path),
                        Err(e) => println!("Failed to save menu: {}", e),
                    }
                }
                None => {
                    let res = menu.save_to_json("menu.json");
                    match res {
                        Ok(_) => println!("Menu saved to menu.json"),
                        Err(e) => println!("Failed to save menu: {}", e),
                    }
                }
            }
        }

        fn run_migration() {
            info!("Running migration");

            let mut db = DatabaseManager::new();

            const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

            info!("Running pending migrations");

            let _ = &db.connection.run_pending_migrations(MIGRATIONS);

            info!("Migration complete");
        }

        pub async fn server(port: u16) -> std::io::Result<()> {
            Self::run_migration();
            
            start_server(port).await
        }
    }
}
