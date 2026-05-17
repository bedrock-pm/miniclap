use miniclap::{App, Arg, ArgKind};

fn app() -> App {
    App::new("prog")
        .arg(Arg::new("verbose", 'v', ArgKind::Count))
        .subcommand(
            App::new("add")
                .arg(Arg::positional("package"))
                .arg(Arg::new("force", 'f', ArgKind::Flag)),
        )
        .subcommand(
            App::new("remote")
                .subcommand(App::new("list"))
                .subcommand(App::new("add").arg(Arg::positional("url"))),
        )
}

#[test]
fn no_subcommand() {
    let m = app().parse_args(&[]);

    assert_eq!(m.subcommand_name(), None);
}

#[test]
fn subcommand_name() {
    let m = app().parse_args(&["add", "serde"]);

    assert_eq!(m.subcommand_name(), Some("add"));
}

#[test]
fn subcommand_positional() {
    let m = app().parse_args(&["add", "serde"]);
    let sub = m.subcommand("add").unwrap();

    assert_eq!(sub.positionals(), &["serde"]);
}

#[test]
fn subcommand_flag() {
    let m = app().parse_args(&["add", "--force", "serde"]);
    let sub = m.subcommand("add").unwrap();

    assert!(sub.flag("force"));
    assert_eq!(sub.positionals(), &["serde"]);
}

#[test]
fn root_flag_before_subcommand() {
    let m = app().parse_args(&["-v", "add", "serde"]);

    assert_eq!(m.count("verbose"), 1);
    assert_eq!(m.subcommand_name(), Some("add"));
}

#[test]
fn subcommand_none_returns_none() {
    let m = app().parse_args(&["add", "serde"]);

    assert!(m.subcommand("remote").is_none());
}

#[test]
fn nested_subcommand_name() {
    let m = app().parse_args(&["remote", "list"]);

    assert_eq!(m.subcommand_name(), Some("remote"));

    let remote = m.subcommand("remote").unwrap();

    assert_eq!(remote.subcommand_name(), Some("list"));
}

#[test]
fn nested_subcommand_positional() {
    let m = app().parse_args(&["remote", "add", "https://example.com"]);
    let remote = m.subcommand("remote").unwrap();
    let add = remote.subcommand("add").unwrap();

    assert_eq!(add.positionals(), &["https://example.com"]);
}
