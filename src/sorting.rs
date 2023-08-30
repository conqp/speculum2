use crate::Mirror;
use sort_option::SortOption;
use sortable_mirror::SortableMirror;

pub mod sort_option;
pub mod sortable_mirror;

#[derive(Debug)]
pub struct SortOptions(Vec<SortOption>);

impl SortOptions {
    #[must_use]
    pub fn new(options: Vec<SortOption>) -> Self {
        Self(options)
    }

    #[must_use]
    pub const fn for_mirror<'a>(&'a self, mirror: &'a Mirror) -> SortableMirror {
        SortableMirror::new(&self.0, mirror)
    }
}
