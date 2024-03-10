use std::collections::HashMap;

use ussdframework::prelude::{HashStrAny, USSDRequest};

pub fn buy_airtime(_request: &USSDRequest, _url: &str) -> HashStrAny {
    let mut response = HashMap::new();

    response.insert("status".to_string(), HashStrAny::Str("success".to_string()));
    response.insert("message".to_string(), HashStrAny::Str("Airtime bought successfully".to_string()));

    HashStrAny::Dict(response)
}