use ussdframework::prelude::*;

mod functions;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut app = UssdApp::new(true, None);
    app.register_functions(functions::get_functions());

    // Start the MenuBuilder server.
    // - json_seed:    path to JSON file used to seed the DB on first run (pass None to skip)
    // - database_url: SQLite file path (None = "menu.sqlite3" in cwd, or set USSD_DATABASE_URL)
    app.serve(8080, Some("examples/data/menu.json"), None).await
}
