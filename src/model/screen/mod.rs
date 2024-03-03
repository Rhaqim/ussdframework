pub mod custom;
pub mod function;
pub mod initial;
pub mod input;
pub mod menu;
pub mod quit;
pub mod router;

pub trait USSDScreens {
    fn next_screen(&self) -> Box<dyn USSDScreens>;
}

pub struct BaseScreen {
    pub next_screen: String,
}
