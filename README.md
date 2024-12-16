# scrings

Semantic scanner based on [`tree-sitter`](https://tree-sitter.github.io/tree-sitter/)

`scrings` is a strings utility that will output only semantically valid strings based on tree-sitter grammar. For each script language we made a list of semantic nodes discriminant enough to detect the target language.

Python bindings are available in the `pyscrings` package.

A [volatility](./volatility) 3 plugin is also available to leverage memory dump analysis.

## Usage

`scrings` is available through a command line utility like `strings`:

```
scrings 0.1.0
Airbus CERT <cert@airbus.com>

USAGE:
    scrings.exe [FLAGS] [OPTIONS] [bash]

FLAGS:
        --escape     Escape string before print
    -h, --help       Prints help information
    -o, --offset     Print offset in file
    -V, --version    Prints version information

OPTIONS:
    -l, --language <language>    Language to match [possible values: powershell, bash, python, sql, javascript, php]
    -p, --path <path>            Path to the script file
    -s, --step <step>            Min length [default: 20]

ARGS:
    <bash>    bash
```

```
scrings --path [PATH_TO_DUMP] -o -l powershell

...
151297294       $eiD=-join'ylbmessA'[-1..-8];$JOD=-join'epyTteG'[-1..-7]
...
```

## Install 

`scrings` is available on crates.io:

```
cargo +nightly install scrings --features="scrings"
```

## Supported languages

* [Python](https://github.com/tree-sitter/tree-sitter-python) ✅
* [Javascript](https://github.com/tree-sitter/tree-sitter-javascript) ✅
* [Powershell](https://github.com/airbus-cert/tree-sitter-powershell) ✅
* [PHP](https://github.com/tree-sitter/tree-sitter-php) ✅
* [Bash](https://github.com/tree-sitter/tree-sitter-bash) ✅
* [SQL](https://github.com/derekstride/tree-sitter-sql) ✅
* VBS ❌ (ongoing)

## Build

`scrings` is made in Rust 🦀.

⚠️ Use nightly version of Rust ⚠️

To built `scrings` you must rely on `cargo` :

```
git clone https://github.com/airbus-cert/scrings
cd scrings
cargo build --package scrings --bin scrings --features=scrings
```
