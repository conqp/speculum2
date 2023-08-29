use crate::{Mirror, Protocol};
use chrono::{Duration, Local};
use regex::Regex;
use std::collections::HashSet;

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug)]
pub struct FilterOptions {
    protocols: HashSet<Protocol>,
    countries: HashSet<String>,
    max_age: Duration,
    re_match: Regex,
    re_inverse_match: Regex,
    complete: bool,
    active: bool,
    ipv4: bool,
    ipv6: bool,
    isos: bool,
}

impl FilterOptions {
    pub fn filter<'filter: 'mirrors, 'mirrors: 'item, 'item>(
        &'filter self,
        mirrors: &'mirrors [Mirror],
    ) -> impl Iterator<Item = &'item Mirror> {
        mirrors.iter().filter(|mirror| self.match_mirror(mirror))
    }

    fn match_mirror(&self, mirror: &Mirror) -> bool {
        if !self.protocols.contains(&mirror.protocol()) {
            return false;
        }

        let country = mirror.country();

        if !(self.countries.contains(country.name()) | self.countries.contains(country.code())) {
            return false;
        }

        if mirror.last_sync().unwrap_or_default() < Local::now() - self.max_age {
            return false;
        }

        // TODO: Complete implementation

        true
    }
}
