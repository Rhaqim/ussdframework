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

#[derive(Debug, Clone, Default)]
pub enum ScreenType {
    #[default]
    Initial,
    Menu,
    Input,
    Quit,
    Custom,
    Function,
}
impl ToString for ScreenType {
    fn to_string(&self) -> String {
        match self {
            ScreenType::Initial => "initial_screen".to_string(),
            ScreenType::Menu => "menu_screen".to_string(),
            ScreenType::Input => "input_screen".to_string(),
            ScreenType::Quit => "quit_Screen".to_string(),
            ScreenType::Custom => "custom_Screen".to_string(),
            ScreenType::Function => "function_Screen".to_string(),
        }
    }
}
