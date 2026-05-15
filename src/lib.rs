use std::collections::HashMap;

pub mod ansi;

use ansi::*;

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
    pub fn new(name: &'static str, short: char, kind: ArgKind) -> Self {
        Arg {
            name,
            short: Some(short),
            kind,
        }
    }

    pub fn positional(name: &'static str) -> Self {
        Arg {
            name,
            short: None,
            kind: ArgKind::Positional,
        }
    }

    pub fn no_short(mut self) -> Self {
        self.short = None;
        self
    }
}

#[derive(Debug, Clone)]
pub struct App {
    pub name: &'static str,
    pub about: Option<&'static str>,
    pub category: Option<&'static str>,
    args: Vec<Arg>,
    subcommands: Vec<App>,
}

impl App {
    pub fn new(name: &'static str) -> Self {
        App {
            name,
            about: None,
            category: None,
            args: Vec::new(),
            subcommands: Vec::new(),
        }
    }

    pub fn about(mut self, about: &'static str) -> Self {
        self.about = Some(about);
        self
    }

    pub fn category(mut self, category: &'static str) -> Self {
        self.category = Some(category);
        self
    }

    pub fn arg(mut self, arg: Arg) -> Self {
        self.args.push(arg);
        self
    }

    pub fn subcommand(mut self, sub: App) -> Self {
        self.subcommands.push(sub);
        self
    }

    pub fn print_help(&self) {
        self.print_help_with_parents(&[]);
    }

    fn print_help_with_parents(&self, parents: &[&'static str]) {
        let mut usage_parts: Vec<String> = parents.iter().map(|s| s.to_string()).collect();
        
        usage_parts.push(bold(self.name));

        let has_flags = self.args.iter().any(|a| a.kind != ArgKind::Positional);
        let has_positionals = self.args.iter().any(|a| a.kind == ArgKind::Positional);
        let has_subcommands = !self.subcommands.is_empty();

        if has_flags {
            usage_parts.push(dim("[OPTIONS]"));
        }
        
        if has_subcommands {
            usage_parts.push(cyan("<COMMAND>"));
        }
        
        if has_positionals {
            for arg in self.args.iter().filter(|a| a.kind == ArgKind::Positional) {
                usage_parts.push(yellow(&format!("<{}>", arg.name.to_uppercase())));
            }
        }

        println!("{} {}", bold("usage:"), usage_parts.join(" "));
        
        if let Some(about) = self.about {
            println!();
            println!("  {}", about);
        }
        
        if has_positionals {
            println!();
            println!("{}", bold("arguments:"));
            
            for arg in self.args.iter().filter(|a| a.kind == ArgKind::Positional) {
                println!(
                    "  {}",
                    yellow(&format!("<{}>", arg.name.to_uppercase()))
                );
            }
        }
        
        println!();
        println!("{}", bold("options:"));

        for arg in self.args.iter().filter(|a| a.kind != ArgKind::Positional) {
            let short = arg
                .short
                .map(|c| format!("{}, ", green(&format!("-{c}"))))
                .unwrap_or_else(|| "    ".to_string());

            let long_hint = match arg.kind {
                ArgKind::Value => format!(
                    "{} {}",
                    green(&format!("--{}", arg.name)),
                    yellow(&format!("<{}>", arg.name.to_uppercase()))
                ),
                _ => green(&format!("--{}", arg.name)),
            };

            let kind_label = match arg.kind {
                ArgKind::Flag => dim("[flag]"),
                ArgKind::Count => dim("[count]"),
                ArgKind::Value => dim("[value]"),
                _ => String::new(),
            };
            
            let raw_long = match arg.kind {
                ArgKind::Value => format!("--{} <{}>", arg.name, arg.name.to_uppercase()),
                _ => format!("--{}", arg.name),
            };
            
            let pad = 24usize.saturating_sub(raw_long.len());

            println!("  {short}{long_hint}{:pad$}  {kind_label}", "", pad = pad);
        }
        
        let help_flags = format!("{}, {}", green("-h"), green("--help"));
        let pad = 24usize.saturating_sub("-h, --help".len());
        
        println!(
            "  {help_flags}{:pad$}  {}",
            "",
            dim("print help"),
            pad = pad
        );
        
        if has_subcommands {
            let mut sections: Vec<(Option<&'static str>, Vec<&App>)> = Vec::new();
        
            for sub in &self.subcommands {
                if let Some(section) = sections.iter_mut().find(|(cat, _)| *cat == sub.category) {
                    section.1.push(sub);
                } else {
                    sections.push((sub.category, vec![sub]));
                }
            }
            
            sections.sort_by_key(|(cat, _)| cat.is_none());
        
            let name_width = self.subcommands.iter().map(|s| s.name.len()).max().unwrap_or(0);
            let has_any_category = sections.iter().any(|(cat, _)| cat.is_some());
        
            println!();
            println!("{}", bold("commands:"));
        
            for (cat, subs) in &sections {
                if has_any_category {
                    match cat {
                        Some(label) => println!("  {}", dim(&format!("{label}:"))),
                        None        => println!("  {}", dim("other:")),
                    }
                }
                
                for sub in subs {
                    let about = sub.about.map(|a| dim(a)).unwrap_or_default();
                    let pad = name_width.saturating_sub(sub.name.len());
                    let indent = if has_any_category { "    " } else { "  " };
                    
                    println!("{indent}{}{:pad$}  {about}", cyan(sub.name), "", pad = pad);
                }
            }
        
            let mut full = parents.to_vec();
            
            full.push(self.name);
            
            println!();
            println!(
                "  {}",
                dim(&format!("run '{} <COMMAND> --help' for subcommand help", full.join(" ")))
            );
        }
    }

    pub fn parse(self) -> Matches {
        let raw: Vec<String> = std::env::args().skip(1).collect();
        let slices: Vec<&str> = raw.iter().map(String::as_str).collect();

        self.parse_args(&slices)
    }

    pub fn parse_args(self, argv: &[&str]) -> Matches {
        parse_app(&self, argv)
    }
}

#[derive(Debug, Default)]
pub struct Matches {
    flags: HashMap<&'static str, bool>,
    counts: HashMap<&'static str, usize>,
    values: HashMap<&'static str, String>,
    positionals: Vec<String>,
    matched_sub: Option<(&'static str, Box<Matches>)>,
}

impl Matches {
    pub fn flag(&self, name: &str) -> bool {
        *self.flags.get(name).unwrap_or(&false)
    }

    pub fn count(&self, name: &str) -> usize {
        *self.counts.get(name).unwrap_or(&0)
    }

    pub fn value(&self, name: &str) -> Option<&str> {
        self.values.get(name).map(String::as_str)
    }

    pub fn positionals(&self) -> &[String] {
        &self.positionals
    }

    pub fn subcommand(&self, name: &str) -> Option<&Matches> {
        self.matched_sub
            .as_ref()
            .filter(|(n, _)| *n == name)
            .map(|(_, m)| m.as_ref())
    }

    pub fn subcommand_name(&self) -> Option<&'static str> {
        self.matched_sub.as_ref().map(|(n, _)| *n)
    }
}

fn parse_app(app: &App, argv: &[&str]) -> Matches {
    if argv.iter().any(|&a| a == "--help" || a == "-h") {
        app.print_help();
        std::process::exit(0);
    }

    let mut matches = Matches::default();

    for arg in &app.args {
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
            if let Some(sub) = app.subcommands.iter().find(|s| s.name == token) {
                let sub_matches = parse_app(sub, &argv[i + 1..]);
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

fn find_by_long<'a>(app: &'a App, name: &str) -> Option<&'a Arg> {
    app.args
        .iter()
        .find(|a| a.name == name && a.kind != ArgKind::Positional)
}

fn find_by_short(app: &App, ch: char) -> Option<&Arg> {
    app.args.iter().find(|a| a.short == Some(ch))
}
