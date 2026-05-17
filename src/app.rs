use crate::ansi::{bold, dim, cyan, yellow, green};
use crate::arg::{Arg, ArgKind};
use crate::matches::Matches;
use crate::parse::parse_app;

#[derive(Debug, Clone)]
pub struct App {
    pub name: &'static str,
    pub about: Option<&'static str>,
    pub category: Option<&'static str>,
    args: Vec<Arg>,
    subcommands: Vec<App>,
}

impl App {
    #[must_use]
    pub fn new(name: &'static str) -> Self {
        App {
            name,
            about: None,
            category: None,
            args: Vec::new(),
            subcommands: Vec::new(),
        }
    }

    #[must_use]
    pub fn about(mut self, about: &'static str) -> Self {
        self.about = Some(about);
        self
    }

    #[must_use]
    pub fn category(mut self, category: &'static str) -> Self {
        self.category = Some(category);
        self
    }

    #[must_use]
    pub fn arg(mut self, arg: Arg) -> Self {
        self.args.push(arg);
        self
    }

    #[must_use]
    pub fn subcommand(mut self, sub: App) -> Self {
        self.subcommands.push(sub);
        self
    }

    pub fn args(&self) -> impl Iterator<Item = &Arg> {
        self.args.iter()
    }

    #[must_use]
    pub fn find_subcommand(&self, name: &str) -> Option<&App> {
        self.subcommands.iter().find(|s| s.name == name)
    }

    pub fn print_help(&self) {
        self.print_help_with_parents(&[]);
    }

    pub fn print_help_with_parents(&self, parents: &[&'static str]) {
        let mut usage_parts: Vec<String> = parents.iter().map(std::string::ToString::to_string).collect();

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
            println!("  {about}");
        }

        if has_positionals {
            println!();
            println!("{}", bold("arguments:"));

            for arg in self.args.iter().filter(|a| a.kind == ArgKind::Positional) {
                println!("  {}", yellow(&format!("<{}>", arg.name.to_uppercase())));
            }
        }

        println!();
        println!("{}", bold("options:"));

        for arg in self.args.iter().filter(|a| a.kind != ArgKind::Positional) {
            let short = arg
                .short.map_or_else(|| "    ".to_string(), |c| format!("{}, ", green(&format!("-{c}"))));

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
                ArgKind::Positional => String::new(),
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

            let name_width = self
                .subcommands
                .iter()
                .map(|s| s.name.len())
                .max()
                .unwrap_or(0);
            let has_any_category = sections.iter().any(|(cat, _)| cat.is_some());

            println!();
            println!("{}", bold("commands:"));

            for (cat, subs) in &sections {
                if has_any_category {
                    match cat {
                        Some(label) => println!("  {}", dim(&format!("{label}:"))),
                        None => println!("  {}", dim("other:")),
                    }
                }

                for sub in subs {
                    let about = sub.about.map(dim).unwrap_or_default();
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
                dim(&format!(
                    "run '{} <COMMAND> --help' for subcommand help",
                    full.join(" ")
                ))
            );
        }
    }

    pub fn parse(self) -> Matches {
        let raw: Vec<String> = std::env::args().skip(1).collect();
        let slices: Vec<&str> = raw.iter().map(String::as_str).collect();
        self.parse_args(&slices)
    }

    #[must_use]
    pub fn parse_args(self, argv: &[&str]) -> Matches {
        parse_app(&self, argv, &[])
    }
}
