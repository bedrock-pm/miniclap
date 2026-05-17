use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Matches {
    pub(crate) flags: HashMap<&'static str, bool>,
    pub(crate) counts: HashMap<&'static str, usize>,
    pub(crate) values: HashMap<&'static str, String>,
    pub(crate) positionals: Vec<String>,
    pub(crate) matched_sub: Option<(&'static str, Box<Matches>)>,
}

impl Matches {
    #[must_use]
    pub fn flag(&self, name: &str) -> bool {
        *self.flags.get(name).unwrap_or(&false)
    }

    #[must_use]
    pub fn count(&self, name: &str) -> usize {
        *self.counts.get(name).unwrap_or(&0)
    }

    pub fn value(&self, name: &str) -> Option<&str> {
        self.values.get(name).map(String::as_str)
    }

    #[must_use]
    pub fn positionals(&self) -> &[String] {
        &self.positionals
    }

    #[must_use]
    pub fn subcommand(&self, name: &str) -> Option<&Matches> {
        self.matched_sub
            .as_ref()
            .filter(|(n, _)| *n == name)
            .map(|(_, m)| m.as_ref())
    }

    #[must_use]
    pub fn subcommand_name(&self) -> Option<&'static str> {
        self.matched_sub.as_ref().map(|(n, _)| *n)
    }
}
