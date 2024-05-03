use serde_json::json;
use ussdframework::prelude::USSDSession;
use ussdframework::types::HashStrAny;

pub fn buy_airtime(session: &USSDSession, url: &str) -> HashStrAny {
    let amount = session
        .fetch_session_data("amount")
        .unwrap()
        .as_str()
        .unwrap();

    print!("Sending request to: {} with amount: {}", url, amount);

    let json = json!({
        "status": "success",
        "message": "You successful bought airtime worth ".to_owned() + amount
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
