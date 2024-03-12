use serde_json::json;
use ussdframework::prelude::USSDRequest;
use ussdframework::types::HashStrAny;

pub fn buy_airtime(_request: &USSDRequest, _url: &str) -> HashStrAny {
    let json = json!({
        "status": "success",
        "message": "Airtime bought successfully"
    });

    let data = HashStrAny::new();

    data.json_to_hash_str_any(json)
}

pub fn get_balance(_request: &USSDRequest, _url: &str) -> HashStrAny {
    let json = json!({
        "status": "success",
        "message": "Balance fetched successfully"
    });

    let data = HashStrAny::new();

    data.json_to_hash_str_any(json)
}
