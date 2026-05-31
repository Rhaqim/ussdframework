/// Standalone binary entry point — used when running ussdframework as a Docker service.
///
/// Configuration is done entirely through environment variables:
///
/// | Variable            | Default         | Description                                    |
/// |---------------------|-----------------|------------------------------------------------|
/// | `USSD_PORT`         | `8080`          | TCP port the server binds to                   |
/// | `USSD_DATABASE_URL` | `menu.sqlite3`  | SQLite path or Postgres connection string       |
/// | `USSD_JSON_SEED`    | *(unset)*       | Path to a JSON file used to seed the DB once   |
///
/// Requires the `menubuilder` Cargo feature:
///   `cargo build --release --features menubuilder`

#[tokio::main]
async fn main() {
    #[cfg(feature = "menubuilder")]
    {
        let port: u16 = std::env::var("USSD_PORT")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(8080);

        let json_seed = std::env::var("USSD_JSON_SEED").ok();

        // USSD_DATABASE_URL is read directly by the framework via dotenv / env.
        // Pass None here so the env var takes effect without duplication.
        let app = ussdframework::UssdApp::new(true, None);

        if let Err(e) = app
            .serve(port, json_seed.as_deref(), None)
            .await
        {
            eprintln!("Server error: {e}");
            std::process::exit(1);
        }
    }

    #[cfg(not(feature = "menubuilder"))]
    {
        eprintln!("This binary requires the 'menubuilder' feature.");
        eprintln!("Rebuild with:  cargo build --release --features menubuilder");
        std::process::exit(1);
    }
}
