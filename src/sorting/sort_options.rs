use crate::Mirror;
use std::cmp::Ordering;
use std::time::Duration;

pub type SortOptions = Vec<Field>;

#[derive(Debug)]
pub enum Field {
    Url,
    Protocol,
    LastSync,
    CompletionPct,
    Delay,
    DurationAvg,
    DurationStddev,
    Score,
    Country,
}

impl Field {
    #[must_use]
    pub fn compare(&self, lhs: &Mirror, rhs: &Mirror) -> Ordering {
        match self {
            Self::Url => lhs.url().cmp(&rhs.url()),
            Self::Protocol => lhs.protocol().cmp(&rhs.protocol()),
            Self::LastSync => lhs
                .last_sync()
                .unwrap_or_default()
                .cmp(&rhs.last_sync().unwrap_or_default()),
            Self::CompletionPct => lhs.completion_pct().total_cmp(&rhs.completion_pct()),
            Self::Delay => rhs
                .delay()
                .unwrap_or(Duration::MAX)
                .cmp(&lhs.delay().unwrap_or(Duration::MAX)),
            Self::DurationAvg => rhs
                .duration_avg()
                .unwrap_or(f64::MAX)
                .total_cmp(&lhs.duration_avg().unwrap_or(f64::MAX)),
            Self::DurationStddev => rhs
                .duration_stddev()
                .unwrap_or(f64::MAX)
                .total_cmp(&lhs.duration_stddev().unwrap_or(f64::MAX)),
            Self::Score => lhs
                .score()
                .unwrap_or(f64::MAX)
                .total_cmp(&rhs.score().unwrap_or(f64::MAX)),
            Self::Country => lhs.country().cmp(&rhs.country()),
        }
    }
}
