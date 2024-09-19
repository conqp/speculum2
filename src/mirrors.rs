pub mod mirror;

use futures::future::join_all;
use mirror::Mirror;
use serde::{Deserialize, Serialize};

const MIRRORS_URL: &str = "https://www.archlinux.org/mirrors/status/json/";

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Mirrors {
    cutoff: usize,
    last_check: String,
    num_checks: usize,
    check_frequency: usize,
    urls: Vec<Mirror>,
    version: u8,
}

impl Mirrors {
    /// Retrieves mirrors from the web API
    ///
    /// # Errors
    /// Returns an `[reqwest::Error]` if the mirrors could not be retrieved or parsed
    pub async fn retrieve() -> reqwest::Result<Self> {
        reqwest::get(MIRRORS_URL).await?.json().await
    }

    pub fn urls(&self) -> &[Mirror] {
        self.urls.as_slice()
    }
}

impl IntoIterator for Mirrors {
    type Item = Mirror;
    type IntoIter = std::vec::IntoIter<Mirror>;

    fn into_iter(self) -> Self::IntoIter {
        self.urls.into_iter()
    }
}

pub async fn measure(mirrors: &mut [Mirror]) {
    join_all(mirrors.iter_mut().map(|mirror| mirror.measure())).await;
}
