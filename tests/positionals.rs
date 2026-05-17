use miniclap::{App, Arg, ArgKind};

fn app() -> App {
    App::new("test")
        .arg(Arg::positional("file"))
        .arg(Arg::new("verbose", 'v', ArgKind::Flag))
}

#[test]
fn positionals_absent() {
    let m = app().parse_args(&[]);

    assert!(m.positionals().is_empty());
}

#[test]
fn single_positional() {
    let m = app().parse_args(&["foo.txt"]);

    assert_eq!(m.positionals(), &["foo.txt"]);
}

#[test]
fn multiple_positionals() {
    let m = app().parse_args(&["a.txt", "b.txt", "c.txt"]);

    assert_eq!(m.positionals(), &["a.txt", "b.txt", "c.txt"]);
}

#[test]
fn positionals_alongside_flag() {
    let m = app().parse_args(&["-v", "foo.txt"]);

    assert!(m.flag("verbose"));
    assert_eq!(m.positionals(), &["foo.txt"]);
}

#[test]
fn flag_between_positionals() {
    let m = app().parse_args(&["a.txt", "-v", "b.txt"]);

    assert!(m.flag("verbose"));
    assert_eq!(m.positionals(), &["a.txt", "b.txt"]);
}
