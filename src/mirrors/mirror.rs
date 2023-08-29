mod protocol;

use protocol::Protocol;
use serde::{Deserialize, Serialize};

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Deserialize, Serialize)]
pub struct Mirror {
    url: String,
    protocol: Protocol,
    last_sync: Option<String>,
    completion_pct: f64,
    delay: Option<usize>,
    duration_avg: Option<f64>,
    duration_stddev: Option<f64>,
    score: Option<f64>,
    active: bool,
    country: String,
    country_code: String,
    isos: bool,
    ipv4: bool,
    ipv6: bool,
    details: String,
}
