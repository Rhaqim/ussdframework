use serde::{Deserialize, Serialize};

use crate::{error, info, types::{FunctionMap, USSDData}};

use super::USSDSession;

#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
pub struct USSDService {
    pub function_name: String,
    /// HTTP URL to POST the session to when no Rust function is registered.
    ///
    /// When `function_name` is not found in the `FunctionMap` (e.g. when running
    /// as a Docker service where no Rust code can be injected), the framework
    /// will POST the current `USSDSession` as JSON to this URL and store the
    /// JSON response in `session.data[data_key]`.
    ///
    /// **Webhook contract**
    ///
    /// Request: `POST <function_url>` with `Content-Type: application/json` and the
    /// serialised `USSDSession` as the body.
    ///
    /// Response: any valid JSON value — string, number, object, array — which is
    /// stored directly in `session.data[data_key]` as `USSDData`.
    ///
    /// Example (Python handler):
    /// ```python
    /// @app.post("/ussd/check_balance")
    /// def check_balance(session: dict):
    ///     return {"balance": "KES 1,234.56", "account": session["data"]["account_number"]}
    /// ```
    pub function_url: Option<String>,
    pub data_key: String,
    pub service_code: Option<String>,
}

pub trait USSDServiceTrait {
    fn call(&self, session: &mut USSDSession, function_map: &FunctionMap);
}

impl USSDServiceTrait for USSDService {
    fn call(&self, session: &mut USSDSession, function_map: &FunctionMap) {
        let result = if let Some(f) = function_map.get(&self.function_name).cloned() {
            // ── Rust function registered — use it directly (original behaviour) ──
            info!("Calling registered Rust function: {}", self.function_name);
            let url_arg = self.function_url.as_deref().unwrap_or("");
            f(session, url_arg)
        } else if let Some(url) = &self.function_url {
            // ── No Rust function registered — call the webhook URL ──
            info!("No Rust function '{}' registered — calling webhook: {}", self.function_name, url);
            call_webhook(url, session)
        } else {
            let mut result = std::collections::HashMap::new();
            result.insert("error".to_string(), USSDData::Str(format!(
                "Service '{}': no Rust function registered and no function_url configured",
                self.function_name
            )));
            USSDData::Dict(result)
        };

        session.data.insert(self.data_key.clone(), result);
    }

}


// ── Webhook HTTP call ─────────────────────────────────────────────────────────

/// POST the current session to a remote URL and return the response as `USSDData`.
///
/// Runs the HTTP request in a dedicated OS thread so it is safe to call from
/// both synchronous and async (Actix/Tokio) contexts without runtime conflicts.
fn call_webhook(url: &str, session: &USSDSession) -> USSDData {
    let url = url.to_owned();

    // Serialize the session once, before crossing the thread boundary.
    let body = match serde_json::to_value(session) {
        Ok(v) => v,
        Err(e) => {
            error!("Failed to serialise session for webhook '{}': {}", url, e);
            return USSDData::None;
        }
    };

    // Spawn a plain OS thread so `reqwest::blocking` never conflicts with
    // whatever Tokio runtime (if any) is running on the calling thread.
    let (tx, rx) = std::sync::mpsc::channel();
    let url_for_error = url.clone();
    std::thread::spawn(move || {
        let result = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .and_then(|client| {
                client
                    .post(&url)
                    .json(&body)
                    .send()?
                    .error_for_status()?
                    .json::<serde_json::Value>()
            });
        let _ = tx.send(result);
    });

    // `recv_timeout` provides a hard deadline that covers pre-request hangs
    // (DNS stalls, connection queuing) that occur before the reqwest timeout
    // can apply.  We add a small buffer on top of the reqwest timeout (10 s)
    // to avoid racing with it under normal slow-but-succeeding requests.
    match rx.recv_timeout(std::time::Duration::from_secs(12)) {
        Ok(Ok(val)) => USSDData::new(Some(val)),
        Ok(Err(e)) => {
            error!("Webhook call failed: {}", e);
            USSDData::None
        }
        Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
            error!("Webhook '{}' timed out after 12 s — thread may be stuck in DNS/IO", url_for_error);
            USSDData::None
        }
        Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
            error!("Webhook thread disconnected unexpectedly");
            USSDData::None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::USSDData;
    use std::collections::HashMap;

    fn make_session() -> USSDSession {
        USSDSession::new(
            "sess-svc-1".to_string(),
            "MainMenu".to_string(),
            "en".to_string(),
            "+254700000000".to_string(),
        )
    }

    fn make_service(fn_name: &str, data_key: &str) -> USSDService {
        USSDService {
            function_name: fn_name.to_string(),
            function_url: None,
            data_key: data_key.to_string(),
            service_code: None,
        }
    }

    // ── Registered Rust function path ─────────────────────────────────────────

    #[test]
    fn call_uses_registered_rust_function() {
        let svc = make_service("echo_fn", "result");
        let mut function_map: HashMap<String, crate::types::USSDFunction> = HashMap::new();
        function_map.insert(
            "echo_fn".to_string(),
            |_session, _url| USSDData::Str("hello from rust".to_string()),
        );

        let mut session = make_session();
        svc.call(&mut session, &function_map);

        assert!(
            matches!(session.data.get("result"), Some(USSDData::Str(v)) if v == "hello from rust")
        );
    }

    #[test]
    fn registered_function_receives_function_url_as_arg() {
        let mut svc = make_service("url_echo", "url_result");
        svc.function_url = Some("http://example.com/webhook".to_string());

        let mut function_map: HashMap<String, crate::types::USSDFunction> = HashMap::new();
        function_map.insert(
            "url_echo".to_string(),
            |_session, url| USSDData::Str(url.to_string()),
        );

        let mut session = make_session();
        svc.call(&mut session, &function_map);

        assert!(
            matches!(
                session.data.get("url_result"),
                Some(USSDData::Str(v)) if v == "http://example.com/webhook"
            )
        );
    }

    // ── No function registered, no function_url ────────────────────────────────

    #[test]
    fn no_function_no_url_stores_error_dict() {
        let svc = make_service("missing_fn", "result");
        let function_map: HashMap<String, crate::types::USSDFunction> = HashMap::new();

        let mut session = make_session();
        svc.call(&mut session, &function_map);

        // An error dict should be inserted, not None.
        assert!(matches!(session.data.get("result"), Some(USSDData::Dict(_))));
    }

    // ── USSDService struct ─────────────────────────────────────────────────────

    #[test]
    fn service_default_has_empty_fields() {
        let svc = USSDService::default();
        assert!(svc.function_name.is_empty());
        assert!(svc.function_url.is_none());
        assert!(svc.data_key.is_empty());
    }
}

