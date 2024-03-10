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

You can find a complete example of a USSD application built with the USSD Framework [EXAMPLE](examples/basic_usage.rs). or buy running the following command:

```bash
cargo run --example basic_usage
```

Using make:

```bash
make run-example
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

You can find an example of a menu configuration [here](examples/data/menu.json).



It contains the menu items and the services that can be called from the menu.

## License

The USSD Framework is open source software licensed under the [MIT license](LICENSE).

## Contributing

Contributions are welcome! For feature requests, bug reports, or other issues, please create an [issue](https://github.com/Rhaqim/ussdframework/issues) on GitHub. For pull requests, please read the [contributing guidelines](CONTRIBUTING.md) first.

## Authors

- [Rhaqim](https://rhaqim.com)

## Acknowledgements

The USSD Framework was inspired by the [USSD Gateway](https://github.com/ussd/ussdgateway) project built for python.
