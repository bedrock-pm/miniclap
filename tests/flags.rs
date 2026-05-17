use miniclap::{App, Arg, ArgKind};

fn app() -> App {
    App::new("test")
        .arg(Arg::new("verbose", 'v', ArgKind::Flag))
        .arg(Arg::new("quiet", 'q', ArgKind::Flag))
}

#[test]
fn flag_absent_is_false() {
    let m = app().parse_args(&[]);

    assert!(!m.flag("verbose"));
    assert!(!m.flag("quiet"));
}

#[test]
fn flag_long() {
    let m = app().parse_args(&["--verbose"]);

    assert!(m.flag("verbose"));
    assert!(!m.flag("quiet"));
}

#[test]
fn flag_short() {
    let m = app().parse_args(&["-v"]);

    assert!(m.flag("verbose"));
}

#[test]
fn flag_short_combined() {
    let m = app().parse_args(&["-vq"]);

    assert!(m.flag("verbose"));
    assert!(m.flag("quiet"));
}

#[test]
fn unknown_flag_is_false() {
    let m = app().parse_args(&[]);

    assert!(!m.flag("nonexistent"));
}
