use miniclap::{App, Arg, ArgKind};

fn app() -> App {
    App::new("test").arg(Arg::new("verbose", 'v', ArgKind::Count))
}

#[test]
fn count_absent_is_zero() {
    let m = app().parse_args(&[]);

    assert_eq!(m.count("verbose"), 0);
}

#[test]
fn count_long_once() {
    let m = app().parse_args(&["--verbose"]);

    assert_eq!(m.count("verbose"), 1);
}

#[test]
fn count_long_twice() {
    let m = app().parse_args(&["--verbose", "--verbose"]);

    assert_eq!(m.count("verbose"), 2);
}

#[test]
fn count_short_once() {
    let m = app().parse_args(&["-v"]);

    assert_eq!(m.count("verbose"), 1);
}

#[test]
fn count_short_stacked() {
    let m = app().parse_args(&["-vvv"]);

    assert_eq!(m.count("verbose"), 3);
}

#[test]
fn count_mixed() {
    let m = app().parse_args(&["-vv", "--verbose"]);

    assert_eq!(m.count("verbose"), 3);
}

#[test]
fn unknown_count_is_zero() {
    let m = app().parse_args(&[]);

    assert_eq!(m.count("nonexistent"), 0);
}
