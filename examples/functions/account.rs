use serde_json::json;
use ussdframework::prelude::USSDSession;
use ussdframework::types::USSDData;

pub fn get_account(session: &USSDSession, url: &str) -> USSDData {
    // Use the session-level msisdn field; fall back to session data if present.
    let msisdn = session
        .fetch_session_data("msisdn")
        .and_then(|d| d.as_str().map(|s| s.to_string()))
        .unwrap_or_else(|| session.msisdn.clone());

    print!("Sending request to: {} with msisdn: {}", url, msisdn);

    let json = json!({
        "status": "success",
        "message": "Account fetched successfully"
    });

    let data = USSDData::new(Some(json));

    data
}

