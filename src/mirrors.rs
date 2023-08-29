mod mirror;

use mirror::Mirror;
use serde::{Deserialize, Serialize};

const MIRRORS_URL: &str = "https://www.archlinux.org/mirrors/status/json/";

#[derive(Debug, Deserialize, Serialize)]
pub struct Mirrors {
    cutoff: usize,
    last_check: String,
    num_checks: usize,
    check_frequency: usize,
    urls: Vec<Mirror>,
    version: u8,
}

impl Mirrors {
    pub async fn retrieve() -> reqwest::Result<Self> {
        reqwest::get(MIRRORS_URL).await?.json().await
    }
}

impl IntoIterator for Mirrors {
    type Item = Mirror;
    type IntoIter = std::vec::IntoIter<Mirror>;

    fn into_iter(self) -> Self::IntoIter {
        self.urls.into_iter()
    }
}
