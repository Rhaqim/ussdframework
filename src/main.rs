extern crate tokio;

mod core;
mod helper;
mod log;
mod types;

use core::USSDGateway;

#[tokio::main]
async fn main() {
    let ussd_gateway = USSDGateway::new(
        "src/functions".to_string(),
        "src/data/menu.json".to_string(),
    );
    ussd_gateway.initial();
}
