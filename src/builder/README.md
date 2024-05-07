# Menu Builder

The menu builder is a tool that allows you to create a menu structure for your USSD application. The menu builder is a web-based tool that allows you to create a menu structure by dragging and dropping menu items. The menu builder generates a JSON file that you can use to create the menu structure in your USSD application.

## Features

- Interface and database for building and managing menus
- Drag and drop menu items
- Create a menu structure
- Generate a JSON file
- Export the JSON file
- Import a JSON file

## Usage

To use the menu builder, enable the feature in cargo.toml:

```toml
[dependencies]
ussdframework = { version = "0.1", features = ["menu-builder"] }
```

Then, import the menu builder in your application:

```rust
use ussdframework::menu_builder::MenuBuilder;
```

You can now use the menu builder to create a menu structure for your USSD application.

## Example

Here is an example of how to use the menu builder to create a menu structure:

```rust
use ussdframework::menu_builder::MenuBuilder;

fn main() {
    MenuBuilder::server(8080);
}
```

This code will start the menu builder server on port 8080. You can now access the menu builder by visiting `http://localhost:8080` in your web browser.

## Conclusion

The menu builder is a powerful tool that allows you to create and manage menu structures for your USSD application. The menu builder is easy to use and provides a visual interface for crafting menus that follow the USSD structure. With the menu builder, you can create complex menu structures quickly and easily, saving you time and effort in developing your USSD application.
