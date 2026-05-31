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
            error!(
                "Service '{}': no Rust function registered and no function_url configured",
                self.function_name
            );
            USSDData::None
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

    match rx.recv() {
        Ok(Ok(val)) => USSDData::new(Some(val)),
        Ok(Err(e)) => {
            error!("Webhook call failed: {}", e);
            USSDData::None
        }
        Err(_) => {
            error!("Webhook thread disconnected unexpectedly");
            USSDData::None
        }
    }
}

