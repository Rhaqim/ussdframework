pub mod menubuilder {

    use crate::builder::database::run_migration;
    use crate::builder::file::{build, from_json, to_json};
    use crate::builder::server::actix::start_server;
    use crate::types::FunctionMap;

    pub trait MenuBuilderTrait {
        fn to_json(&self, path: Option<&str>) -> ();
        fn from_json(&self, path: Option<&str>) -> ();
        fn server(port: u16, function_map: FunctionMap, json_seed: Option<&str>) -> std::io::Result<()>;

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
        /// Converts the menu to JSON and writes it to a file.
        pub fn to_json(file_path: Option<&str>) {
            let menu = build(None);

            to_json(file_path, menu)
        }

        /// Loads a menu from a JSON file to the database.
        pub fn from_json(file_path: Option<&str>) {
            from_json(file_path)
        }

        /// Starts the server on the specified port, with the given function map
        /// used to handle USSD function-type screens via the `/ussd` endpoint.
        ///
        /// * `json_seed` — optional path to a JSON file used to seed the database
        ///   on first run when it contains no screens yet.
        pub async fn server(port: u16, function_map: FunctionMap, json_seed: Option<&str>) -> std::io::Result<()> {
            run_migration();

            start_server(port, function_map, json_seed.map(str::to_owned)).await
        }
    }
}
