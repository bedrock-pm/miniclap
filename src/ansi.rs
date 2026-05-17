#[must_use]
pub fn bold(s: &str) -> String {
    format!("\x1b[1m{s}\x1b[0m")
}

#[must_use]
pub fn dim(s: &str) -> String {
    format!("\x1b[2m{s}\x1b[0m")
}

#[must_use]
pub fn cyan(s: &str) -> String {
    format!("\x1b[36m{s}\x1b[0m")
}

#[must_use]
pub fn yellow(s: &str) -> String {
    format!("\x1b[33m{s}\x1b[0m")
}

#[must_use]
pub fn green(s: &str) -> String {
    format!("\x1b[32m{s}\x1b[0m")
}
