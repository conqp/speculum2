use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Protocol {
    #[serde(rename = "ftp")]
    Ftp,
    #[serde(rename = "http")]
    Http,
    #[serde(rename = "https")]
    Https,
    #[serde(rename = "rsync")]
    Rsync,
}
