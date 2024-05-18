pub mod menubuilder {
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

    use crate::builder::file::{build, from_json, to_json};
    use crate::builder::server::actix::start_server;
    use crate::builder::DatabaseManager;
    use crate::info;

    pub trait MenuBuilderTrait {
        fn new() -> Self;
        fn to_json(&self, path: Option<&str>) -> ();
        fn from_json(&self, path: Option<&str>) -> ();
        fn run_migration() -> ();
        fn server(port: u16) -> std::io::Result<()>;

        // TODO: Implement the following methods
        fn initial(&self, name: &str, text: &str) -> ();
        fn menu(&self, name: &str, text: &str) -> ();
        fn input(&self, name: &str, text: &str) -> ();
        fn function(&self, name: &str, text: &str) -> ();
        fn router(&self, name: &str, text: &str) -> ();
        fn quit(&self, name: &str, text: &str) -> ();
    }

    pub struct MenuBuilder {}

    impl MenuBuilder {
        pub fn new() -> MenuBuilder {
            info!("Creating new MenuBuilder");
            MenuBuilder {}
        }

        pub fn to_json(&self, file_path: Option<&str>) {
            let menu = build();

            to_json(file_path, menu)
        }

        pub fn from_json(&self, file_path: Option<&str>) {
            from_json(file_path)
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
