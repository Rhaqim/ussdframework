# USSD Framework

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## Overview

The USSD Framework is a powerful, flexible, and easy-to-extend toolkit for building USSD applications in Rust. It handles session management, screen navigation, input validation, function dispatch, and routing — letting you focus on your business logic.

---

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Architecture](#architecture)
- [Quick Start](#quick-start)
- [Screen Types](#screen-types)
- [Screen Properties](#screen-properties)
- [Menu Configuration](#menu-configuration)
- [Session Management](#session-management)
- [Functions](#functions)
- [Multi-Language Support](#multi-language-support)
- [Input Validation & Max Retries](#input-validation--max-retries)
- [Routing](#routing)
- [Example](#example)
- [License](#license)

---

## Features

| Feature | Description |
|---|---|
| Declarative menus | JSON-driven screen definitions — no code changes required to reshape flows |
| Session management | Built-in in-memory store; plug in Redis, Postgres, or any custom store |
| Input validation | Per-screen regex patterns and max-length constraints |
| Max retries | Limit failed attempts per screen and redirect to a timeout screen |
| Function dispatch | Call Rust functions from any screen; results are stored in session data |
| Expression router | Route to different screens based on session-data conditions |
| Multi-language text | Define screen text per language code with automatic fallback |
| Menu builder | Optional admin UI for managing screens stored in a database |

---

## Installation

```toml
[dependencies]
ussdframework = "0.1.0"
```

---

## Architecture

```
+-------------------------------+
|         Telco / Gateway       |
|  formats USSD requests and    |
|  serializes responses         |
+---------------+---------------+
                |
                v
+-------------------------------+
|      USSD Framework Layer     |
|                               |
|  ┌──────────┐  ┌───────────┐  |
|  │  Router  │  │  Session  │  |
|  └────┬─────┘  └─────┬─────┘  |
|       │              │        |
|  ┌────▼──────────────▼─────┐  |
|  │     Screen Executor     │  |
|  │  Menu / Input / Router  │  |
|  │  Function / Quit        │  |
|  └────────────┬────────────┘  |
+---------------+---------------+
                |
                v
+-------------------------------+
|        Service Layer          |
|  Your Rust functions, HTTP    |
|  calls, database queries      |
+-------------------------------+
```

### Request lifecycle

```
Incoming request
      │
      ▼
 Resolve or create session
      │
      ▼
 Has session timed out? ──Yes──► Restart from InitialScreen
      │ No
      ▼
 Loop over current screen:
  ├── Initial  ──► auto-advance, no display
  ├── Function ──► call registered function, advance
  ├── Router   ──► evaluate expressions, advance
  ├── Menu     ──► display options, wait for input
  ├── Input    ──► display prompt, validate input, wait
  └── Quit     ──► display message, end session
      │
      ▼
 Persist session, return response
```

---

## Quick Start

```rust
use ussdframework::prelude::*;

fn main() {
    let mut app = UssdApp::new(true, None);

    // Load screen definitions
    let content = include_str!("../examples/data/menu.json");
    let menus: USSDMenu = serde_json::from_str(content).unwrap();

    // Register your business-logic functions
    app.register_functions(my_functions());

    let request = USSDRequest {
        msisdn: "1234567890".to_string(),
        session_id: "session-abc".to_string(),
        input: "1".to_string(),
        service_code: "*123#".to_string(),
        language: "en".to_string(),
    };

    let response = app.run(request, menus);
    println!("{}", response.message);
}
```

---

## Screen Types

Each screen in the menu has a `screen_type` that determines how the framework handles it.

| Type | Displays to user | Waits for input | Typical use |
|---|:---:|:---:|---|
| `Initial` | No | No | Entry point; auto-advances to `default_next_screen` |
| `Menu` | Yes | Yes | Numbered list of options; validates selection |
| `Input` | Yes | Yes | Free-text prompt; supports regex + max-length validation |
| `Function` | No | No | Calls a registered Rust function; stores result in session |
| `Router` | No | No | Evaluates conditions against session data; branches to matching screen |
| `Quit` | Yes | No | Final message; ends the session |

### Navigation shortcuts (built-in, any screen)

| User input | Action |
|---|---|
| `0` | Go back one screen |
| `00` | Return to the first screen visited |

---

## Screen Properties

| Property | Type | Required | Description |
|---|---|:---:|---|
| `text` | string or `{"lang": "..."}` map | Yes* | Display text. Supports `{{variable}}` interpolation. *Not rendered for Function/Router/Initial. |
| `screen_type` | string | Yes | One of the six screen types above |
| `default_next_screen` | string | Yes | Fallback next screen when no specific condition matches |
| `menu_items` | object | Menu only | Map of named option objects (see below) |
| `input_identifier` | string | Input only | Key under which the user's input is stored in session data |
| `validation_regex` | string | No | Regex the input must match; shows error and retries on failure |
| `max_length` | number | No | Maximum character count for the input |
| `max_retries` | number | No | Max failed attempts on this screen before redirecting |
| `timeout_screen` | string | No | Screen to redirect to when `max_retries` is exceeded |
| `function` | string | Function only | Name of the registered Rust function to call |
| `router_options` | array | Router only | Ordered list of `{ router_option, next_screen }` pairs |
| `service_code` | string | No | Service code that activates this entry point |

### Menu item structure

```json
"menu_items": {
  "UniqueKey": {
    "option": "1",
    "display_name": "Balance Inquiry",
    "next_screen": "BalanceScreen"
  }
}
```

### Router option structure

```json
"router_options": [
  { "router_option": "{{balance.status == 'success'}}", "next_screen": "BalanceResultScreen" },
  { "router_option": "{{balance.status == 'failed'}}",  "next_screen": "NetworkErrorScreen" }
]
```

---

## Menu Configuration

The top-level JSON object has two keys:

```json
{
  "menus": {
    "ScreenName": { ... }
  },
  "services": {
    "service_name": {
      "function_name": "rust_function_name",
      "function_url":  "http://...",
      "data_key":      "key_in_session"
    }
  }
}
```

### Services table

| Field | Description |
|---|---|
| `function_name` | Must match a key registered via `app.register_functions()` |
| `function_url` | Passed as `url: &str` to your function — use it for HTTP calls |
| `data_key` | The result is stored in `session.data` under this key; use `{{data_key.field}}` in text and router expressions |

A full annotated example lives at [examples/data/menu.json](examples/data/menu.json).

---

## Session Management

The framework keeps a session per `session_id`. The session stores:

| Field | Description |
|---|---|
| `session_id` | Unique identifier from the incoming request |
| `msisdn` | Caller's phone number |
| `language` | Language code (`en`, `fr`, …) used for multi-language text lookup |
| `data` | Key-value store for all collected inputs and function results |
| `current_screen` | Screen being processed |
| `visited_screens` | Stack used for `0` (back) navigation |
| `screen_attempts` | Per-screen failed-attempt counter (for `max_retries`) |
| `end_session` | `true` once a Quit screen is reached |

### Providing a custom session store

Implement `SessionCache` and pass it to `UssdApp::new`:

```rust
use ussdframework::prelude::*;

pub struct RedisSession { /* ... */ }

impl SessionCache for RedisSession {
    fn store_session(&self, session: &USSDSession) -> Result<(), String> {
        let json = serde_json::to_string(session).map_err(|e| e.to_string())?;
        // write json to Redis under session.session_id
        Ok(())
    }

    fn retrieve_session(&self, session_id: &str) -> Result<Option<USSDSession>, String> {
        // read from Redis, deserialize, return
        Ok(None)
    }
}

fn main() {
    let mut app = UssdApp::new(false, Some(Box::new(RedisSession::new())));
    // ...
}
```

> **Note:** The built-in in-memory store is suitable for development and single-instance deployments. For production, use a shared store (Redis, Postgres, etc.) so sessions survive restarts and work across multiple replicas.

---

## Functions

Functions are plain Rust `fn` pointers with the signature:

```rust
fn my_fn(session: &USSDSession, url: &str) -> USSDData;
```

Register them before calling `run`:

```rust
fn get_balance(_session: &USSDSession, url: &str) -> USSDData {
    // call url, parse response …
    let json = serde_json::json!({ "status": "success", "amount": "1,250.00" });
    USSDData::new(None).json_to_hash_str_any(json)
}

fn main() {
    let mut app = UssdApp::new(true, None);

    let mut fns = std::collections::HashMap::new();
    fns.insert("get_balance".to_string(), get_balance as USSDFunction);

    app.register_functions(fns);
}
```

The return value is stored in `session.data` under the service's `data_key`. You can then reference it as `{{data_key.field}}` in screen text and router expressions.

---

## Multi-Language Support

`text` can be either a plain string (backward-compatible) or a language map:

```json
"text": "Welcome"
```

```json
"text": {
  "en": "Welcome to Demo Bank",
  "fr": "Bienvenue à Demo Banque",
  "sw": "Karibu Demo Benki"
}
```

The framework resolves `text` using `session.language`, falling back to `"default"` (the value stored when a plain string is used), then to an empty string. Set the user's language via an Input screen that stores a value into a session key, then use that key in subsequent screens.

---

## Input Validation & Max Retries

```json
"EnterPinScreen": {
  "text": "Enter your 4-digit PIN:",
  "screen_type": "Input",
  "input_identifier": "pin",
  "validation_regex": "^[0-9]{4}$",
  "max_length": 4,
  "max_retries": 3,
  "timeout_screen": "PinLockedScreen",
  "default_next_screen": "VerifyPinFunctionScreen"
}
```

| Property | Effect |
|---|---|
| `validation_regex` | Input is rejected and the screen is re-displayed with an error if the pattern does not match |
| `max_length` | Input longer than this is rejected before the regex is checked |
| `max_retries` | After this many consecutive failures the user is sent to `timeout_screen` |
| `timeout_screen` | Usually a Quit screen explaining the lockout |

---

## Routing

A Router screen evaluates `router_options` in order and jumps to the first matching `next_screen`. If nothing matches it falls through to `default_next_screen`.

Expression syntax: `{{<session_key> <op> '<value>'}}`

| Operator | Example |
|---|---|
| `==` | `{{airtime.status == 'success'}}` |
| `>`  | `{{user.age > 18}}` |
| `>=` | `{{balance.amount >= 100}}` |
| `<`  | `{{retry.count < 3}}` |
| `<=` | `{{pin.attempts <= 5}}` |

Router expressions that do not match the pattern are logged as warnings at menu-load time (via `validate_router_expressions`).

---

## Example

A complete working example with actix-web lives in [examples/](examples/). It covers:

- Main menu navigation
- Balance inquiry with PIN validation and max retries
- Send money with phone/amount/PIN input chain and confirmation
- Buy airtime for own or another number
- Account management (view profile, change PIN)
- Multi-language selection (English / Français)
- Error screens, network-error fallbacks, and session lockout

```bash
# Run the example server
cargo run --example basic_usage

# Or using make
make run-example
```

The server listens on `http://127.0.0.1:3000/ussd` and accepts POST requests:

```json
{
  "msisdn": "0712345678",
  "session_id": "test-session-1",
  "input": "",
  "service_code": "*123#",
  "language": "en"
}
```

---

## License

MIT — see [LICENSE](LICENSE).

## Contributing

Issues and pull requests are welcome. See the [contributing guidelines](CONTRIBUTING.md) for details.

## Authors

- [Rhaqim](https://rhaqim.com)


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

## Usecase

USSD applications interact with telecom networks to provide services to users. They are commonly used for mobile banking, airtime top-up, and other financial services. USSD applications are widely used in developing countries where internet access is limited or expensive. They are also used in emergency services, healthcare, and other critical applications. In order to make full use of the framework a developer would need to have access to the telecom network's USSD specification document. Provided is the architecutre to build a fully functional USSD application.

```bash
+---------------------------------------------------+
|                    Telco Layer                    |
|                                                   |
| - Gets data from various telcos                   |
| - Formats data for USSD requests                  |
| - Formats/serializes responses for telcos         |
+--------------------------+------------------------+
                           |
                           v
+---------------------------------------------------+
|                USSD Framework Layer               |
|                                                   |
| - Receives formatted data from Telco Layer        |
| - Sends formatted data to Service Layer           |
| - Receives responses from Service Layer           |
| - Processes responses according to menu config    |
+--------------------+------------------------------+
                     |
                     v
+---------------------------------------------------+
|                    Service Layer                  |
|                                                   |
| - Interacts with other services                   |
| - Receives HTTP requests from USSD Framework      |
| - Returns responses to USSD Framework             |
+---------------------------------------------------+
```

## License

The USSD Framework is open source software licensed under the [MIT license](LICENSE).

## Contributing

Contributions are welcome! For feature requests, bug reports, or other issues, please create an [issue](https://github.com/Rhaqim/ussdframework/issues) on GitHub. For pull requests, please read the [contributing guidelines](CONTRIBUTING.md) first.

## Authors

- [Rhaqim](https://rhaqim.com)

## Acknowledgements

The USSD Framework was inspired by the [USSD Gateway](https://github.com/ussd/ussdgateway) project built for python.
