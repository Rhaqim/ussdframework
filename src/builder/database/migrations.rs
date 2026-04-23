use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use crate::{builder::DatabaseManager, info};

#[cfg(all(feature = "db-sqlite", not(feature = "db-postgres")))]
const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

#[cfg(feature = "db-postgres")]
const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations_pg");

pub fn run_migration() {
    info!("Running migration");

    let mut db = DatabaseManager::new();

    info!("Running pending migrations");

    let _ = &db.connection.run_pending_migrations(MIGRATIONS);

    info!("Migration complete");
}
