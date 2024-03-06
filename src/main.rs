extern crate tokio;

mod core;
mod examples;
mod helper;
mod log;
mod types;

use core::{SessionCache, USSDConfig, USSDGateway, USSDSession};
use redis;
use std::io::{self, Write};

use crate::core::{USSDMenu, USSDRequest};

#[tokio::main]
async fn main() {
    let functions_path = "src/data/functions";

    struct RedisCache {
        // client: redis::Client,
    }

    impl SessionCache for RedisCache {
        fn store_session(&self, session: &USSDSession) -> Result<(), String> {
            // let mut connection = self.client.get_connection().unwrap();
            // let _: () = redis::cmd("SET")
            //     .arg(session.session_id.clone())
            //     .arg(serde_json::to_string(session).unwrap())
            //     .query(&mut connection)
            //     .unwrap();
            Ok(())
        }

        fn retrieve_session(&self, session_id: &str) -> Result<Option<USSDSession>, String> {
            // let mut connection = self.client.get_connection().unwrap();
            // let session: Option<String> = redis::cmd("GET")
            //     .arg(session_id)
            //     .query(&mut connection)
            //     .unwrap();
            // match session {
            //     Some(session) => {
            //         let session: USSDSession = serde_json::from_str(&session).unwrap();
            //         Ok(Some(session))
            //     }
            //     None => Ok(None),
            // }
            Ok(None)
        }
    }

    let config = USSDConfig {
        functions_path: functions_path.to_string(),
        session_cache: Box::new(RedisCache {
            // client: redis::Client::open("redis://localhost/").unwrap(),
        }),
    };

    let menus = USSDMenu::load_from_json("src/data/menu.json").unwrap();

    let gateway = USSDGateway::new(config, menus);

    fn read_user_input() -> Option<String> {
        print!("Enter your choice: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        if let Ok(_) = io::stdin().read_line(&mut input) {
            Some(input.trim().to_string())
        } else {
            None
        }
    }

    async fn initiate_request(gateway: USSDGateway) {
        let input = read_user_input().unwrap();

        let request = USSDRequest {
            input,
            session_id: "123".to_string(),
            msisdn: "123".to_string(),
            request_id: "123".to_string(),
            telco: "Vodafone".to_string(),
            service_code: "123".to_string(),
            country_code: "123".to_string(),
            language: "en".to_string(),
        };

        let response = gateway.process_request(request);

        match response {
            Some(response) => {
                println!("Response: {:?}", response);
            }
            None => {
                println!("Invalid input or screen!");
            }
        }
    }

    initiate_request(gateway).await;

    // let config = USSDConfig::new(
    //     "src/functions".to_string(),
    //     "src/data/menu.json".to_string(),
    //     60,
    // );

    // let ussd_gateway: USSDGateway = USSDGateway::new(config);

    // ussd_gateway.initial();
}
