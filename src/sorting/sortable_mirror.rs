use super::sort_option::SortOption;
use crate::Mirror;
use std::cmp::Ordering;

#[derive(Debug)]
pub struct SortableMirror<'a> {
    sort_options: &'a Vec<SortOption>,
    mirror: &'a Mirror,
}

impl<'a> SortableMirror<'a> {
    pub const fn new(sort_options: &'a Vec<SortOption>, mirror: &'a Mirror) -> Self {
        Self {
            sort_options,
            mirror,
        }
    }
}

impl<'a> Eq for SortableMirror<'a> {}

impl<'a> PartialEq<Self> for SortableMirror<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl<'a> PartialOrd<Self> for SortableMirror<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for SortableMirror<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        for sort_option in self.sort_options {
            let result = sort_option.compare(self.mirror, other.mirror);

            if result == Ordering::Equal {
                continue;
            }

            return result;
        }

        Ordering::Equal
    }
}
