use serde_json::json;
use ussdframework::prelude::USSDSession;
use ussdframework::types::HashStrAny;

pub fn buy_airtime(_session: &USSDSession, _url: &str) -> HashStrAny {
    let json = json!({
        "status": "success",
        "message": "Airtime bought successfully"
    });

    let data = HashStrAny::new();

    data.json_to_hash_str_any(json)
}

pub fn get_balance(_session: &USSDSession, _url: &str) -> HashStrAny {
    let json = json!({
        "status": "success",
        "message": "Balance fetched successfully"
    });

    let data = HashStrAny::new();

    data.json_to_hash_str_any(json)
}
