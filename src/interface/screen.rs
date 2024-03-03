pub mod ussd_screen {
    pub trait UssdScreenInterface {
        fn show(&self) -> String;
        fn get_input(&self) -> String;
    }

    // create a new trait that inherits from UssdScreenInterface
    pub trait UssdScreenInterfaceWithValidation: UssdScreenInterface {
        fn validate(&self, input: String) -> bool;
    }
}