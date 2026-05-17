# miniclap

a no-dependency command line parser for rust.

## installation

install *miniclap* via cargo:

```sh
cargo add miniclap
```

install the latest unstable version of *miniclap*:

```sh
cargo add miniclap --git https://github.com/bedrock-pm/miniclap.git
```

## usage

define your app with a builder, then parse:

```rust
use miniclap::{App, Arg, ArgKind};

let app = App::new("tool")
    .about("does something useful")
    .arg(Arg::new("verbose", 'v', ArgKind::Count))
    .arg(Arg::new("output",  'o', ArgKind::Value))
    .arg(Arg::positional("file"));

let matches = app.parse_args(&["--output", "out.txt", "-vv", "input.txt"]);

println!("{}", matches.count("verbose")); // 2
println!("{}", matches.value("output").unwrap()); // "out.txt"
println!("{}", matches.positionals()[0]); // "input.txt"
```

### arg kinds

| kind | description | example |
|---|---|---|
| `Flag` | present or absent | `--verbose` |
| `Count` | how many times it appears | `-vvv` → 3 |
| `Value` | takes a string argument | `--output file.txt` or `--output=file.txt` |
| `Positional` | bare non-flag arguments | `mytool foo.txt` |

### subcommands

```rust
let app = App::new("tool")
    .subcommand(
        App::new("add")
            .about("add a package")
            .arg(Arg::positional("package"))
            .arg(Arg::new("force", 'f', ArgKind::Flag))
    );

let matches = app.parse_args(&["add", "--force", "serde"]);
let sub = matches.subcommand("add").unwrap();

println!("{}", sub.flag("force")); // true
println!("{}", sub.positionals()[0]); // "serde"
```

subcommands can be nested arbitrarily deep. `-h` / `--help` at any level prints help scoped to that subcommand.

### help

*miniclap* generates and prints help automatically — no setup needed. calling `parse` or `parse_args` will intercept `-h` / `--help`, print the help screen, and exit.

you can also print help manually:

```rust
// print help for the root app
app.print_help();

// print help for a subcommand, with the correct parent prefix in the usage line
app.find_subcommand("add").unwrap().print_help_with_parents(&["mytool"]);
```

> [!TIP]
> if your command module has a fallthrough arm that should show subcommand help, use
> `print_help_with_parents` rather than `print_help` so the usage line reads
> `mytool add <COMMAND>` instead of just `add <COMMAND>`.

### short flags without a char

use `.no_short()` to define a long-only flag:

```rust
Arg::new("dry-run", ' ', ArgKind::Flag).no_short()
```

### categories

subcommands can be grouped under named categories in the help output:

```rust
App::new("sub").category("plumbing")
```

subcommands without a category are grouped under *other:* if any categorised commands exist.

## api

### `App`

| method | description |
|---|---|
| `App::new(name)` | create a new app |
| `.about(text)` | set the description shown in help |
| `.category(label)` | set the category for help grouping |
| `.arg(arg)` | add an argument |
| `.subcommand(app)` | add a subcommand |
| `.args()` | iterate over registered args |
| `.find_subcommand(name)` | look up a subcommand by name |
| `.parse()` | parse `std::env::args()` |
| `.parse_args(argv)` | parse a given slice |
| `.print_help()` | print help and return |
| `.print_help_with_parents(parents)` | print help with a parent prefix |

### `Matches`

| method | description |
|---|---|
| `.flag(name)` | `true` if the flag was passed |
| `.count(name)` | how many times the flag appeared |
| `.value(name)` | `Some(&str)` if a value was provided |
| `.positionals()` | slice of bare positional arguments |
| `.subcommand(name)` | `Some(&Matches)` for the matched subcommand |
| `.subcommand_name()` | name of the matched subcommand, if any |

## why does this exist?

i got tired of waiting for clap and it's many dependencies to install. so i made a no-dep clap alternative!
