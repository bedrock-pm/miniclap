#[derive(Debug, Clone, PartialEq)]
pub enum ArgKind {
    Flag,
    Count,
    Value,
    Positional,
}

#[derive(Debug, Clone)]
pub struct Arg {
    pub name: &'static str,
    pub short: Option<char>,
    pub kind: ArgKind,
}

impl Arg {
    #[must_use]
    pub fn new(name: &'static str, short: char, kind: ArgKind) -> Self {
        Arg {
            name,
            short: Some(short),
            kind,
        }
    }

    #[must_use]
    pub fn positional(name: &'static str) -> Self {
        Arg {
            name,
            short: None,
            kind: ArgKind::Positional,
        }
    }

    #[must_use]
    pub fn no_short(mut self) -> Self {
        self.short = None;
        self
    }
}
