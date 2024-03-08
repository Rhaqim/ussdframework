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
ussd-framework = "0.1.0"
```

## Usage

Here's a simple example of how to create a USSD menu using the USSD Framework:

```rust
use ussdframework::prelude::*;

fn main() {
    let mut app = UssdApp::new();

    app.menu("main", |menu| {
        menu.prompt("Welcome to the USSD Framework!");
        menu.option("1", "Option 1", |menu| {
            menu.prompt("You selected Option 1");
        });
        menu.option("2", "Option 2", |menu| {
            menu.prompt("You selected Option 2");
        });
    });

    app.run();
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
