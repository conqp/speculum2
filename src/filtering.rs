use crate::{Mirror, Protocol};
use chrono::{DateTime, Local};
use regex::Regex;
use std::collections::HashSet;
use std::time::Duration;

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug)]
pub struct FilterOptions {
    protocols: Option<HashSet<Protocol>>,
    countries: Option<HashSet<String>>,
    max_age: Option<Duration>,
    re_match: Option<Regex>,
    re_inverse_match: Option<Regex>,
    complete: bool,
    active: bool,
    ipv4: bool,
    ipv6: bool,
    isos: bool,
    max_download_time: Option<Duration>,
    // Local timestamp
    now: DateTime<Local>,
}

impl FilterOptions {
    pub fn filter<'filter: 'mirrors, 'mirrors: 'item, 'item>(
        &'filter self,
        mirrors: &'mirrors [Mirror],
    ) -> impl Iterator<Item = &'item Mirror> {
        mirrors.iter().filter(|mirror| self.match_mirror(mirror))
    }

    fn match_mirror(&self, mirror: &Mirror) -> bool {
        if let Some(ref protocols) = self.protocols {
            if !protocols.contains(&mirror.protocol()) {
                return false;
            }
        }

        if let Some(ref countries) = self.countries {
            let country = mirror.country();

            if !(countries.contains(country.name()) | countries.contains(country.code())) {
                return false;
            }
        }

        if let Some(max_age) = self.max_age {
            if mirror.last_sync().unwrap_or_default() < self.now - max_age {
                return false;
            }
        }

        if let Some(ref re_match) = self.re_match {
            if !re_match.is_match(mirror.url().as_str()) {
                return false;
            }
        }

        if let Some(ref re_inverse_match) = self.re_inverse_match {
            if re_inverse_match.is_match(mirror.url().as_str()) {
                return false;
            }
        }

        if self.complete & (mirror.completion_pct() < 1.0) {
            return false;
        }

        if self.active & !mirror.active() {
            return false;
        }

        if self.ipv4 & !mirror.ipv4() {
            return false;
        }

        if self.ipv6 & !mirror.ipv6() {
            return false;
        }

        if self.isos & !mirror.isos() {
            return false;
        }

        if let Some(max_download_time) = self.max_download_time {
            if mirror.download_time().unwrap_or(Duration::MAX) > max_download_time {
                return false;
            }
        }

        true
    }
}
