<div style="text-align: center;">
   <h1>USSD Framework</h1>

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
</div>

## Overview

The USSD Framework is a powerful and flexible framework designed to be easy to use and extensible for building USSD applications. It provides a set of tools and utilities to simplify the development of USSD menus, navigation, and user interactions. With a simple API for creating USSD menus and handling user input. It supports session management and stateful interactions, with built-in validation and error handling mechanisms.

## Features

- Easy-to-use API for creating USSD menus and handling user input
- Support for session management and stateful interactions
- Built-in validation and error handling mechanisms
- Extensible architecture to support custom USSD applications
- [Menu builder](src/builder/README.md) for creating custom menus in the application and storing them in a database (Optional)
- Cross-platform compatibility for users from other programming languages

## Installation

To use the USSD Framework in your Rust project, add the following line to your `Cargo.toml` file:

```toml
[dependencies]
ussdframework = "0.1.0"
```

## Usage

### Initialization

To create a new USSD application, you need to create a new instance of the USSD Framework and configure it with the necessary settings. You can then start the application by calling the `run` method.

```rust
    use ussdframework::prelude::*;

    fn main() {
        // Create a new instance of the USSD Framework
        let mut ussd = UssdApp::new(true, None);

        // Fetch the menu configuration from a file
        let content = include_str!("../examples/data/menu.json");
        let menus: USSDMenu = serde_json::from_str(&content).unwrap();

        let request = UssdRequest {
            msisdn: "1234567890".to_string(),
            session_id: "1234567890".to_string(),
            input: 0,
            service_code: "*123#".to_string(),
            language: "en".to_string(),
        };

        ussd.run(request, menus);
    }
```

### Menu Configuration

The menus loaded need to be a specific format. Each menu items would have the following properties:

- **text**: The text to display to the user.
- **screen_type**: The type of screen to display.
- **default_next_screen**: The next screen to navigate to by default if no option is selected in the case of a menu or input screen.
- **menu_items**: The list of menu items to display to the user in the case of a menu screen.
- **input_identifier**: The identifier to use for the input in the case of an input screen.
- **function**: The function to call in the case of a function screen.
- **router_options**: The list of options to use for routing in the case of a router screen.

Each menu can be of the following types:

- **Initial**: The entry point of the Menus when they start a session, automatically navigates to the next screen.
- **Menu**: A menu screen that displays a list of options to the user.
- **Input**: A screen that takes input from the user.
- **Function**: A screen that calls a function and navigates to the next screen based on the result.
- **Router**: A screen that navigates to the next screen based on the result of a function call.
- **Quit**: A screen that ends the session, displaying a message to the user.

The services that can be called from the menu are also defined in the configuration. Each service has the following properties:

- **function_name**: The name of the function to call.
- **function_url**: The URL the function calls for other services.
- **data_key**: The key to use for the data returned from the function call.

You can find an example of a menu configuration [here](examples/data/menu.json).

It contains the menu items and the services that can be called from the menu.

### Session Management

The USSD Framework supports session management and stateful interactions. It keeps track of the user's session and navigates to the next screen based on the user's input. The `UssdSession` struct stores the user's session data and update it as needed. If the built-in session management is not sufficient, you can implement your own session management logic. The session must implement the `SessionCache` trait.

> **NOTE:** The USSD Framework does provide a default `IN MEMORY` session management implementation.But it is `HIGHLY` advised that you implement your own session management logic.

```rust
    use ussdframework::prelude::*;

    pub struct RedisSession {
        client: redis::Client,
        connection: redis::Connection,
    }

    impl RedisSession {
        pub fn new() -> Self {
            let client = redis::Client::open("redis://<username>:<password>@localhost/").unwrap();
            let connection = client.get_connection().unwrap();

            RedisSession { client, connection }
        }
    }

    impl SessionCache for RedisSession {
        fn store_session(&self, session: &USSDSession) -> Result<(), String> {
            let session_str = serde_json::to_string(session).map_err(|e| e.to_string())?;
            self.connection
                .set(session.session_id.clone(), session_str)
                .map_err(|e| e.to_string())?;
            Ok(())
        }

        fn retrieve_session(&self, session_id: &str) -> Result<Option<USSDSession>, String> {
            let session_str: Option<String> = self
                .connection
                .get(session_id)
                .map_err(|e| e.to_string())?;
            match session_str {
                Some(session_str) => {
                    let session: USSDSession =
                        serde_json::from_str(&session_str).map_err(|e| e.to_string())?;
                    Ok(Some(session))
                }
                None => Ok(None),
            }
        }
    }

    fn main() {
        // Create a new instance of the USSD Framework
        let mut ussd = UssdApp::new(false, Some(Box::new(RedisSession::new())));

        ...
    }

```

### Functions

The USSD Framework supports calling functions from the menu configuration. You can define a function to call in the menu configuration and implement it in your application. The functions must be registered with the USSD Framework before they can be called. The functions must conform to the `USSDFunction: fn(&USSDSession, &str) -> USSDData;`

```rust
    use ussdframework::prelude::*;

    fn my_function(session: &USSDSession, input: &str) -> USSDData {
         let json = json!({
            "status": "success",
            "message": "Function called"
        });

        let data = USSDData::new();

        // Convert the JSON data to USSDData by passing the JSON data to the USSDData.json_to_hash_str_any() method
        data.json_to_hash_str_any(json)
    }

    fn my_function2(session: &USSDSession, input: &str) -> USSDData {
         let json = json!({
            "status": "success",
            "message": "Function2 called"
        });

        // Convert the JSON data to USSDData by passing the JSON data to the USSDData::new() method
        let data = USSDData::new(Some(json));

        data
    }

    fn functions() -> FunctionMap {
        let mut functions = HashMap::new();
        functions.insert("my_function".to_string(), my_function);
        functions.insert("my_function2".to_string(), my_function2);

        functions
    }

    fn main() {
        // Create a new instance of the USSD Framework
        let mut ussd = UssdApp::new(true, None);

        // Register the function with the USSD Framework
        ussd.register_functions(functions());

        ...
    }
```

### Example

You can find a complete example of a USSD application built with the USSD Framework and actix-web [EXAMPLE](examples). or buy running the following command:

```bash
cargo run --example basic_usage
```

Using make:

```bash
make run-example
```

## License

The USSD Framework is open source software licensed under the [MIT license](LICENSE).

## Contributing

Contributions are welcome! For feature requests, bug reports, or other issues, please create an [issue](https://github.com/Rhaqim/ussdframework/issues) on GitHub. For pull requests, please read the [contributing guidelines](CONTRIBUTING.md) first.

## Authors

- [Rhaqim](https://rhaqim.com)

## Acknowledgements

The USSD Framework was inspired by the [USSD Gateway](https://github.com/ussd/ussdgateway) project built for python.
