pub mod menubuilder {
    use std::collections::HashMap;

    use crate::builder::schema::{
        model_screen::screens::dsl::screens as screen_dsl,
        model_service::services::dsl::services as services_dsl,
    };
    use crate::builder::{Database, DatabaseManager};
    use crate::builder::{Screen as ScreenModel, Service as ServiceModel};
    use crate::core::USSDMenu;

    pub trait MenuBuilderTrait {
        fn new(service_code: &str) -> Self;
        fn build(&self) -> USSDMenu;
    }

    pub struct MenuBuilder {
        pub service_code: String,
    }

    impl MenuBuilderTrait for MenuBuilder {
        fn new(service_code: &str) -> Self {
            MenuBuilder {
                service_code: service_code.to_string(),
            }
        }

        fn build(&self) -> USSDMenu {
            let mut db = DatabaseManager::new();

            let mut menus = HashMap::new();
            let mut services = HashMap::new();

            let service: Vec<ServiceModel> = db.get_many().expect("Failed to get services");

            for s in service {
                services.insert(s.name.clone(), s.to_ussd_service());
            }

            USSDMenu { menus, services }
        }
    }
}
