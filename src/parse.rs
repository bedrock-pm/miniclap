use crate::arg::{Arg, ArgKind};
use crate::app::App;
use crate::matches::Matches;

#[must_use]
pub fn parse_app(app: &App, argv: &[&str], parents: &[&'static str]) -> Matches {
    if argv.iter().any(|&a| a == "--help" || a == "-h") {
        app.print_help_with_parents(parents);
        std::process::exit(0);
    }

    let mut matches = Matches::default();

    for arg in app.args() {
        match arg.kind {
            ArgKind::Flag => {
                matches.flags.insert(arg.name, false);
            }
            ArgKind::Count => {
                matches.counts.insert(arg.name, 0);
            }
            _ => {}
        }
    }

    let mut i = 0;

    while i < argv.len() {
        let token = argv[i];

        if !token.starts_with('-') {
            if let Some(sub) = app.find_subcommand(token) {
                let mut sub_parents = parents.to_vec();
                sub_parents.push(app.name);

                let sub_matches = parse_app(sub, &argv[i + 1..], &sub_parents);
                matches.matched_sub = Some((sub.name, Box::new(sub_matches)));
                return matches;
            }

            matches.positionals.push(token.to_string());
            i += 1;
            continue;
        }

        if let Some(rest) = token.strip_prefix("--") {
            let (long_name, inline_val) = match rest.split_once('=') {
                Some((k, v)) => (k, Some(v)),
                None => (rest, None),
            };

            if let Some(arg) = find_by_long(app, long_name) {
                match arg.kind {
                    ArgKind::Flag => {
                        matches.flags.insert(arg.name, true);
                    }
                    ArgKind::Count => {
                        *matches.counts.entry(arg.name).or_insert(0) += 1;
                    }
                    ArgKind::Value => {
                        let val = if let Some(v) = inline_val {
                            v.to_string()
                        } else {
                            i += 1;
                            argv.get(i).unwrap_or(&"").to_string()
                        };
                        matches.values.insert(arg.name, val);
                    }
                    ArgKind::Positional => {}
                }
            }
            i += 1;
            continue;
        }

        if let Some(chars_str) = token.strip_prefix('-') {
            let chars: Vec<char> = chars_str.chars().collect();
            let mut ci = 0;

            while ci < chars.len() {
                let ch = chars[ci];

                if let Some(arg) = find_by_short(app, ch) {
                    match arg.kind {
                        ArgKind::Flag => {
                            matches.flags.insert(arg.name, true);
                        }
                        ArgKind::Count => {
                            *matches.counts.entry(arg.name).or_insert(0) += 1;
                        }
                        ArgKind::Value => {
                            let val = if ci + 1 < chars.len() {
                                let v: String = chars[ci + 1..].iter().collect();
                                ci = chars.len();
                                v
                            } else {
                                i += 1;
                                argv.get(i).unwrap_or(&"").to_string()
                            };
                            matches.values.insert(arg.name, val);
                        }
                        ArgKind::Positional => {}
                    }
                }
                ci += 1;
            }

            i += 1;
            continue;
        }

        i += 1;
    }

    matches
}

#[must_use]
pub fn find_by_long(app: &App, name: &str) -> Option<Arg> {
    app.args()
        .find(|a| a.name == name && a.kind != ArgKind::Positional)
        .cloned()
}

#[must_use]
pub fn find_by_short(app: &App, ch: char) -> Option<Arg> {
    app.args().find(|a| a.short == Some(ch)).cloned()
}
