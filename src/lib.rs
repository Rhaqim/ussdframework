mod core;
mod log;
pub mod prelude;
pub mod types;
mod utils;

extern crate serde;

use core::{
    process_request, InMemorySessionStore, SessionCache, USSDMenu, USSDRequest, USSDResponse,
};

#[cfg(feature = "menubuilder")]
mod builder;

/// Represents a USSD application.
///
/// The application owns the registered function map (previously a process-global).
/// This makes the app self-contained, testable, and safe for multi-app processes.
pub struct UssdApp {
    pub session_cache: Box<dyn SessionCache>,
    function_map: types::FunctionMap,
}

impl UssdApp {
    /// Creates a new instance of `UssdApp`.
    ///
    /// * `built_in_session_manager` — use the in-process in-memory store.
    /// * `session_manager` — supply an external cache implementation.
    pub fn new(
        built_in_session_manager: bool,
        session_manager: Option<Box<dyn SessionCache>>,
    ) -> UssdApp {
        let session_cache: Box<dyn SessionCache> =
            if built_in_session_manager || session_manager.is_none() {
                Box::new(InMemorySessionStore::new())
            } else {
                session_manager.unwrap()
            };

        UssdApp {
            session_cache,
            function_map: types::FunctionMap::new(),
        }
    }

    /// Register a batch of USSD functions.
    ///
    /// Functions are stored in the app instance rather than a process global,
    /// so multiple independent `UssdApp` instances can have different function sets.
    pub fn register_functions(&mut self, functions_map: types::FunctionMap) {
        for (path, function) in functions_map {
            self.function_map.entry(path).or_insert(function);
        }
    }

    /// Runs the USSD application with the given request and screens.
    ///
    /// Router expressions in the menu are validated before processing begins.
    /// Invalid expressions are logged as errors but do not abort the request —
    /// the affected router screen will fall through to `default_next_screen`.
    pub fn run(&self, request: USSDRequest, screens: USSDMenu) -> USSDResponse {
        // Validate router expressions and log any problems at startup time.
        if let Err(expr_errors) = screens.validate_router_expressions() {
            for msg in &expr_errors {
                crate::error!("Router expression validation: {}", msg);
            }
        }

        process_request(&request, &self.session_cache, &screens, &self.function_map)
    }

    /// Displays the menu message to stdout.
    pub fn display_menu(&self, ussd_response: &USSDResponse) {
        println!("{}", ussd_response.message);
    }

    /// Starts the MenuBuilder server.
    ///
    /// Only available when the `menubuilder` feature is enabled.
    ///
    /// This mode is intended for **building** menus: it serves the admin frontend (proxied
    /// from the Next.js dev server), exposes the CRUD API for screens/services, and handles
    /// `/ussd` requests by loading menus directly from the SQLite database.
    ///
    /// # Arguments
    ///
    /// * `port`      — TCP port the Actix server will bind to (e.g. `8080`).
    /// * `json_seed` — Optional path to a JSON file used to seed the database on first run
    ///   when it contains no screens yet. Pass `None` if you're managing the database
    ///   exclusively through the admin portal.
    ///
    /// # Non-menubuilder mode
    ///
    /// When the `menubuilder` feature is **not** enabled, call `run()` with a
    /// developer-provided `USSDMenu` instead.
    /// # Database
    ///
    /// By default the SQLite database file is `menu.sqlite3` in the current working directory.
    /// Pass a `database_url` (e.g. `Some("path/to/my.sqlite3")`) to use a different file, or
    /// set the `USSD_DATABASE_URL` environment variable before starting the server.
    #[cfg(feature = "menubuilder")]
    pub async fn serve(
        &self,
        port: u16,
        json_seed: Option<&str>,
        database_url: Option<&str>,
    ) -> std::io::Result<()> {
        use builder::database::run_migration;
        use builder::server::actix::start_server;

        if let Some(url) = database_url {
            std::env::set_var("USSD_DATABASE_URL", url);
        }

        run_migration();
        start_server(
            port,
            self.function_map.clone(),
            json_seed.map(str::to_owned),
            None, // env var already set above
        ).await
    }
}


