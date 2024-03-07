pub struct MenuBuilder {
    name: String,
    // You can add more fields as needed
}

impl MenuBuilder {
    pub fn new(name: &str) -> Self {
        MenuBuilder {
            name: name.to_string(),
        }
    }

    pub fn prompt(&mut self, message: &str) {
        println!("{}", message);
        // Handle prompt logic
    }

    pub fn option<F>(&mut self, option: &str, label: &str, handler: F)
    where
        F: FnOnce(&mut MenuBuilder),
    {
        println!("{}. {}", option, label);
        // Handle option logic
        let mut sub_menu_builder = MenuBuilder::new(&format!("{}_{}", self.name, option));
        handler(&mut sub_menu_builder);
    }
}
