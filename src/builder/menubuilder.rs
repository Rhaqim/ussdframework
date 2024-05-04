pub mod menubuilder {
    use crate::menu::USSDMenu;
    use crate::ussd_screens::Screen;
    use crate::ussd_service::USSDService;
    use diesel::{self, prelude::*};

    pub trait MenuBuilderTrait {
        fn new(service_code: &str, connection: Option<DbConnection>) -> Self;
        fn add_screen(&self, screen: Screen);
        fn add_service(&self, service: USSDService);
        fn build(&self) -> USSDMenu;
    }

    pub struct MenuBuilder {
        pub service_code: String,
        pub connection: Option<DbConnection>,
    }

    impl MenuBuilderTrait for MenuBuilder {
        fn new(service_code: &str, connection: Option<DbConnection>) -> Self {
            MenuBuilder {
                service_code: service_code.to_string(),
                connection,
            }
        }

        fn add_screen(&self, screen: Screen) {
            if let Some(ref conn) = self.connection {
                diesel::insert_into(screens::table)
                    .values(&screen)
                    .execute(conn)
                    .expect("Error saving new screen");
            } else {
                panic!("No database connection provided");
            }
        }

        fn add_service(&self, service: USSDService) {
            if let Some(ref conn) = self.connection {
                // Add the service to the database
                diesel::insert_into(services::table)
                    .values(&service)
                    .execute(conn)
                    .expect("Error saving new service");
            } else {
                panic!("No database connection provided");
            }
        }

        fn build(&self) -> USSDMenu {
            let mut menus = HashMap::new();
            let mut services = HashMap::new();

            if let Some(ref conn) = self.connection {
                // Add the screens and services to the menu
                for screen in Screen::belonging_to(&self.service_code)
                    .load::<Screen>(conn)
                    .expect("Error loading screens")
                {
                    menus.insert(screen.name.clone(), screen);
                }

                for service in USSDService::belonging_to(&self.service_code)
                    .load::<USSDService>(conn)
                    .expect("Error loading services")
                {
                    services.insert(service.name.clone(), service);
                }
            } else {
                panic!("No database connection provided");
            }

            USSDMenu { menus, services }
        }
    }
}
