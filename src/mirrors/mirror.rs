mod country;
mod protocol;

use chrono::{DateTime, FixedOffset};
pub use country::Country;
use log::{info, warn};
pub use protocol::Protocol;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time::Instant;

const CORE_FILES: &str = "core/os/x86_64/core.files";

#[allow(clippy::struct_excessive_bools)]
#[derive(Clone, Debug, Deserialize, Serialize)]
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
    #[serde(default)]
    download_time: Option<Duration>,
}

impl Mirror {
    /// Returns the mirror URL
    ///
    /// # Panics
    /// Panics if the URL is malformed
    #[must_use]
    pub fn url(&self) -> Url {
        Url::parse(&self.url).unwrap_or_else(|_| panic!("Malformed URL on mirror: {}", self.url))
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

    #[must_use]
    pub const fn download_time(&self) -> Option<Duration> {
        self.download_time
    }

    /// Measure the download time of this mirror
    ///
    /// # Errors
    /// Returns an `[reqwest::Error]` on download errors
    ///
    /// # Panics
    /// Panics on malformed URLs
    pub async fn measure(&mut self) -> reqwest::Result<()> {
        let start = Instant::now();
        reqwest::get(
            self.url()
                .join(CORE_FILES)
                .expect("Malformed measurement URL"),
        )
        .await
        .inspect_err(|error| warn!("{error}"))?;
        let download_time = Instant::now() - start;
        self.download_time.replace(download_time);
        info!("Measured mirror {} @ {:?}", self.url, download_time);
        Ok(())
    }
}
