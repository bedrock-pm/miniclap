use miniclap::{App, Arg, ArgKind};

fn app() -> App {
    App::new("test")
        .arg(Arg::new("output", 'o', ArgKind::Value))
        .arg(Arg::new("format", 'f', ArgKind::Value))
}

#[test]
fn value_absent_is_none() {
    let m = app().parse_args(&[]);

    assert_eq!(m.value("output"), None);
}

#[test]
fn value_long_separate() {
    let m = app().parse_args(&["--output", "file.txt"]);

    assert_eq!(m.value("output"), Some("file.txt"));
}

#[test]
fn value_long_equals() {
    let m = app().parse_args(&["--output=file.txt"]);

    assert_eq!(m.value("output"), Some("file.txt"));
}

#[test]
fn value_short_separate() {
    let m = app().parse_args(&["-o", "file.txt"]);

    assert_eq!(m.value("output"), Some("file.txt"));
}

#[test]
fn value_short_adjacent() {
    let m = app().parse_args(&["-ofile.txt"]);

    assert_eq!(m.value("output"), Some("file.txt"));
}

#[test]
fn value_multiple_independent() {
    let m = app().parse_args(&["--output", "a.txt", "--format", "json"]);

    assert_eq!(m.value("output"), Some("a.txt"));
    assert_eq!(m.value("format"), Some("json"));
}

#[test]
fn unknown_value_is_none() {
    let m = app().parse_args(&[]);

    assert_eq!(m.value("nonexistent"), None);
}
