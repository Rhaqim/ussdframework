use ussdframework::prelude::*;

mod functions;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Start the MenuBuilder server on port 8080.
    // It serves:
    //   - /ussd          — USSD request handler (loads menus from SQLite DB)
    //   - /api/*         — Admin CRUD API for screens, services, menu items, router options
    //   - everything else — proxied to the Next.js dev server on port 3000
    menubuilder::MenuBuilder::server(8080, functions::get_functions(), Some("examples/data/menu.json")).await
}
