use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Country<'mirror> {
    name: &'mirror str,
    code: &'mirror str,
}

impl<'mirror> Country<'mirror> {
    pub const fn new(name: &'mirror str, code: &'mirror str) -> Self {
        Self { name, code }
    }

    pub const fn name(&self) -> &str {
        self.name
    }

    pub const fn code(&self) -> &str {
        self.code
    }
}

impl<'mirror> Display for Country<'mirror> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.name, self.code)
    }
}
