use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialOrd, PartialEq, Serialize)]
pub enum Protocol {
    #[serde(rename = "https")]
    Https,
    #[serde(rename = "rsync")]
    Rsync,
    #[serde(rename = "http")]
    Http,
    #[serde(rename = "ftp")]
    Ftp,
}
