# USSD Framework

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## Overview

The USSD Framework is a powerful and flexible framework for building USSD applications in Rust. It provides a set of tools and utilities to simplify the development of USSD menus, navigation, and user interactions.

## Features

- Easy-to-use API for creating USSD menus and handling user input
- Support for session management and stateful interactions
- Built-in validation and error handling mechanisms
- Extensible architecture to support custom USSD applications
- Cross-platform compatibility for users from other programming languages

## Installation

To use the USSD Framework in your Rust project, add the following line to your `Cargo.toml` file:

```toml
[dependencies]
ussdframework = "0.1.0"
```

## Usage

Here's a simple example of how to create a USSD menu using the USSD Framework with actix-web:

### Example

```rust
use actix_web::{web, App, HttpResponse, HttpServer};
use std::{collections::HashMap, sync::Mutex};
use ussdframework::prelude::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        let session_store = InMemorySessionStore::new();
        let app = UssdApp::new("config/functions".to_string(), Box::new(session_store));

        let content = include_str!("../examples/data/menu.json");
        let menus: USSDMenu = serde_json::from_str(&content).unwrap();

        App::new()
            .app_data(web::Data::new(app))
            .app_data(web::Data::new(menus))
            .route("/ussd", web::post().to(handle_ussd))
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}

async fn handle_ussd(
    req: web::Json<USSDRequest>,
    app: web::Data<UssdApp>,
    menus: web::Data<USSDMenu>,
) -> HttpResponse {
    let request = req.into_inner();
    let response = app.run(request, menus.get_ref().clone());
    HttpResponse::Ok().body(response.message)
}

pub struct InMemorySessionStore {
    data: Mutex<HashMap<String, String>>,
}

impl InMemorySessionStore {
    pub fn new() -> Self {
        Self {
            data: Mutex::new(HashMap::new()),
        }
    }
}

impl SessionCache for InMemorySessionStore {
    fn store_session(&self, session: &USSDSession) -> Result<(), String> {
        let mut data = self.data.lock().unwrap();
        data.insert(
            session.session_id.clone(),
            serde_json::to_string(session).unwrap(),
        );
        Ok(())
    }

    fn retrieve_session(&self, session_id: &str) -> Result<Option<USSDSession>, String> {
        let data = self.data.lock().unwrap();
        match data.get(session_id) {
            Some(session) => Ok(Some(serde_json::from_str(session).unwrap())),
            None => Ok(None),
        }
    }
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
- **functions_path**: The path to the function to call.
- **function_url**: The URL the function calls for other services.
- **data_key**: The key to use for the data returned from the function call.

Here's an example of a menu configuration:

```json
{
	"menus": {
		"InitialScreen": {
			"text": "Welcome to the system",
			"screen_type": "Initial",
			"default_next_screen": "MainScreen"
		},
		"MainScreen": {
			"text": "Main Menu",
			"screen_type": "Menu",
			"default_next_screen": "DefaultNoneScreen",
			"menu_items": {
				"BalanceOption": {
					"option": "1",
					"display_name": "Balance Inquiry",
					"next_screen": "BalanceInquiryScreen"
				},
				"AirtimeOption": {
					"option": "3",
					"display_name": "Buy Airtime",
					"next_screen": "AirtimeScreen"
				}
			}
		},
		"DefaultNoneScreen": {
			"text": "Thank you for using the system",
			"screen_type": "Quit",
			"default_next_screen": "MainScreen"
		},
		"BalanceInquiryScreen": {
			"text": "Select Account",
			"screen_type": "Menu",
			"default_next_screen": "MainScreen",
			"menu_items": {
				"SavingsOption": {
					"option": "1",
					"display_name": "Savings",
					"next_screen": "MainScreen"
				},
				"CurrentOption": {
					"option": "2",
					"display_name": "Current",
					"next_screen": "MainScreen"
				}
			}
		},
		"AirtimeScreen": {
			"text": "Select option",
			"screen_type": "Menu",
			"default_next_screen": "MainScreen",
			"menu_items": {
				"OwnNumberOption": {
					"option": "1",
					"display_name": "Own Number",
					"next_screen": "OwnNumberAmountScreen"
				},
				"OtherNumberOption": {
					"option": "2",
					"display_name": "Other Number",
					"next_screen": "OtherNumberAmountScreen"
				}
			}
		},
		"OwnNumberAmountScreen": {
			"text": "Enter amount",
			"screen_type": "Input",
			"input_identifier": "amount",
			"default_next_screen": "OwnNumberFunctionScreen"
		},
		"OwnNumberFunctionScreen": {
			"text": "Processing...",
			"screen_type": "Function",
			"function": "buy_airtime",
			"default_next_screen": "OwnNumberRouterScreen"
		},
		"OwnNumberRouterScreen": {
			"text": "Routing...",
			"screen_type": "Router",
			"router_options": [
				{
					"router_option": "{{airtime.status == `success`}}",
					"next_screen": "SuccessScreen"
				},
				{
					"router_option": "{{airtime.status == `failed`}}",
					"next_screen": "OwnNumberFailedScreen"
				}
			],
			"default_next_screen": "DefaultNoneScreen"
		},
		"SuccessScreen": {
			"text": "Transaction Successful",
			"screen_type": "Quit",
			"default_next_screen": "MainScreen"
		},
		"OwnNumberFailedScreen": {
			"text": "Transaction Failed",
			"screen_type": "Quit",
			"default_next_screen": "MainScreen"
		}
	},
	"services": {
		"buy_airtime": {
			"function_name": "buy_airtime",
			"functions_path": "services/buy_airtime.js",
			"function_url": "http://localhost:3000/buy_airtime",
			"data_key": "airtime"
		}
	}
}
```

It contains the menu items and the services that can be called from the menu.

## License

The USSD Framework is open source software licensed under the [MIT license](LICENSE).

## Contributing

Contributions are welcome! For feature requests, bug reports, or other issues, please create an [issue](https://github.com/Rhaqim/ussdframework/issues) on GitHub. For pull requests, please read the [contributing guidelines](CONTRIBUTING.md) first.

## Authors

- [Rhaqim](https://rhaqim.com)

## Acknowledgements

The USSD Framework was inspired by the [USSD Gateway](https://github.com/ussd/ussdgateway) project built for python.
