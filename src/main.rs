extern crate tokio;

mod core;
mod examples;
mod helper;
mod log;
mod types;

use core::{USSDConfig, USSDGateway};

#[tokio::main]
async fn main() {
    let config = USSDConfig::new(
        "src/functions".to_string(),
        "src/data/menu.json".to_string(),
        60,
    );

    let ussd_gateway: USSDGateway = USSDGateway::new(config);

    ussd_gateway.initial();
}
