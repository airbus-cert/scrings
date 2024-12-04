# scrings

Semantic scanner based on [`tree-sitter`](https://tree-sitter.github.io/tree-sitter/)

`scrings` will find semantically valid script parts in  a flat file (like a memory dump).

Python bindings are available in the `pyscrings` package.

A Volatility 3 plugin is also available to leverage memory dump analysis.

## Usage

`scrings` is available through a command line utility like `strings`:

```
scrings --path [PATH_TO_DUMP] -o -l powershell

...
151297294       $eiD=-join'ylbmessA'[-1..-8];$JOD=-join'epyTteG'[-1..-7]
...
```

`scrings` is also available through a [volatility](./volatility) plugin to leverage memory analysis technics.

## Supported languages

* [Python](https://github.com/tree-sitter/tree-sitter-python) ✅
* [Javascript](https://github.com/tree-sitter/tree-sitter-javascript) ✅
* [Powershell](https://github.com/airbus-cert/tree-sitter-powershell) ✅
* [PHP](https://github.com/tree-sitter/tree-sitter-php) ✅
* [Bash](https://github.com/tree-sitter/tree-sitter-bash) ✅
* [SQL](https://github.com/derekstride/tree-sitter-sql) ✅
* VBS ❌ (ongoing)

## Build

`scrings` is made in Rust 🦀. To built `scrings` you must rely on `cargo` :

```
cargo build --package scrings --bin scrings --features=scrings
```
