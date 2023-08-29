mod country;
mod protocol;

use chrono::{DateTime, FixedOffset};
use country::Country;
use protocol::Protocol;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Deserialize, Serialize)]
pub struct Mirror {
    url: String,
    protocol: Protocol,
    last_sync: Option<String>,
    completion_pct: f64,
    delay: Option<u64>,
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

impl Mirror {
    #[must_use]
    pub fn url(&self) -> &str {
        self.url.as_str()
    }

    #[must_use]
    pub fn protocol(&self) -> Protocol {
        self.protocol.clone()
    }

    #[must_use]
    pub fn last_sync(&self) -> Option<DateTime<FixedOffset>> {
        self.last_sync
            .as_ref()
            .and_then(|datetime| DateTime::parse_from_rfc3339(datetime.as_str()).ok())
    }

    #[must_use]
    pub const fn completion_pct(&self) -> f64 {
        self.completion_pct
    }

    #[must_use]
    pub fn delay(&self) -> Option<Duration> {
        self.delay.map(Duration::from_micros)
    }

    #[must_use]
    pub const fn duration_avg(&self) -> Option<f64> {
        self.duration_avg
    }

    #[must_use]
    pub const fn duration_stddev(&self) -> Option<f64> {
        self.duration_stddev
    }

    #[must_use]
    pub const fn score(&self) -> Option<f64> {
        self.score
    }

    #[must_use]
    pub const fn active(&self) -> bool {
        self.active
    }

    #[must_use]
    pub fn country(&self) -> Country {
        Country::new(self.country.as_str(), self.country_code.as_str())
    }

    #[must_use]
    pub const fn isos(&self) -> bool {
        self.isos
    }

    #[must_use]
    pub const fn ipv4(&self) -> bool {
        self.ipv4
    }

    #[must_use]
    pub const fn ipv6(&self) -> bool {
        self.ipv6
    }

    #[must_use]
    pub fn details(&self) -> &str {
        self.details.as_str()
    }
}
