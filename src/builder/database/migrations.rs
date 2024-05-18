use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use crate::{builder::DatabaseManager, info};

pub fn run_migration() {
    info!("Running migration");

    let mut db = DatabaseManager::new();

    const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

    info!("Running pending migrations");

    let _ = &db.connection.run_pending_migrations(MIGRATIONS);

    info!("Migration complete");
}
