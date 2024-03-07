pub mod prelude;
pub mod menu;

pub struct UssdApp {
    // You can define any necessary fields here
}

impl UssdApp {
    pub fn new() -> Self {
        // Initialize any necessary resources
        UssdApp {}
    }

    pub fn menu<F>(&mut self, name: &str, builder: F)
    where
        F: FnOnce(&mut menu::MenuBuilder),
    {
        let mut menu_builder = menu::MenuBuilder::new(name);
        builder(&mut menu_builder);
        // Store or process the constructed menu
    }

    pub fn run(&self) {
        // Execute the USSD application
    }
}
