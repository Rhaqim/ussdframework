# Menu Builder

The menu builder is a tool that allows you to create a menu structure for your USSD application. The menu builder is a web-based tool that allows you to create a menu structure by dragging and dropping menu items. The menu builder generates a JSON file that you can use to create the menu structure in your USSD application.

## Features

- Interface and database for building and managing menus
- Drag and drop menu items
- Create a menu structure
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

## Generating a JSON File

Once you have created a menu structure in the menu builder, you can generate a JSON file that you can use in your USSD application. To generate a JSON file, click on the "Generate JSON" button in the menu builder. The JSON file will be downloaded to your computer or you can run it directly from the code.

```rust
use ussdframework::menu_builder::MenuBuilder;

fn main() {
    let json_file = "menu.json";
    
    MenuBuilder::to_json(Some(json_file));
}
```

This code will generate a JSON file with the menu structure and save it to the specified file. If none is provided it will save to the root directory with the name `menu.json`.

>Note: The `to_json` method builds and generates a JSON file with the menu structure that exists in the menu builder database. If you have not created a menu structure in the menu builder, the generated JSON file will be empty.

## Importing a JSON File

You can also import a JSON file into the menu builder to load a menu structure. To import a JSON file, click on the "Import JSON" button in the menu builder and select the JSON file you want to import.

```rust
use ussdframework::menu_builder::MenuBuilder;

fn main() {
    let json_file = "menu.json";
    
    MenuBuilder::from_json(Some(json_file));
}
```

This code will import the JSON file with the menu structure and load it into the menu builder database. If none is provided it will load the `menu.json` file from the root directory.

## Benefits

The `to_json` and `from_json` methods are useful for saving and loading menu structures from JSON files, allowing you to create and manage menu structures outside of the menu builder interface. They can also be used in the menu builder interface, just drag and drop the file into the browser window.

## Conclusion

The menu builder is a powerful tool that allows you to create and manage menu structures for your USSD application. The menu builder is easy to use and provides a visual interface for crafting menus that follow the USSD structure. With the menu builder, you can create complex menu structures quickly and easily, saving you time and effort in developing your USSD application.
