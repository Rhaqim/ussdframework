use ussdframework::prelude::*;

mod functions;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut app = UssdApp::new(true, None);
    app.register_functions(functions::get_functions());

    // menubuilder mode: menus are loaded from the SQLite database.
    // The admin frontend (Next.js) is proxied at every route except /ussd and /api/*.
    // Pass a JSON path to seed the database on first run when it is empty.
    app.serve(8080, Some("examples/data/menu.json")).await
}
