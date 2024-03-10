# USSD Framework

<!-- [![Build Status](https://img.shields.io/travis/username/repo.svg)](https://travis-ci.org/username/repo) -->
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/licenses/MIT)

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

## License

The USSD Framework is open source software licensed under the [MIT license](https://opensource.org/licenses/MIT).

## Contributing

Contributions are welcome! For feature requests, bug reports, or other issues, please create an [issue](https://github.com/Rhaqim/ussdframework/issues) on GitHub. For pull requests, please read the [contributing guidelines](CONTRIBUTING.md) first.

## Authors

- [Rhaqim](https://rhaqim.com)

## Acknowledgements

The USSD Framework was inspired by the [USSD Gateway](https://github.com/ussd/ussdgateway) project built for python.
