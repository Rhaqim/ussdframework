use serde_json::json;
use ussdframework::prelude::USSDSession;
use ussdframework::types::USSDData;

pub fn get_account(session: &USSDSession, url: &str) -> USSDData {
    let msisdn = session
        .fetch_session_data("msisdn")
        .unwrap()
        .as_str()
        .unwrap();

    print!("Sending request to: {} with msisdn: {}", url, msisdn);

    let json = json!({
        "status": "success",
        "message": "Account fetched successfully"
    });

    let data = USSDData::new();

    data.json_to_hash_str_any(json)
}
