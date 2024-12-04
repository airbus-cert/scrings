# pyscrings

Python bindings for `scrings` powered by PyO3 and maturin

## Usage

```
from pyscrings import powershell, javascript, php, bash, python, sql

with open("path_to_dump", "rb") as f:
    for (offset, match) in powershell(f):
        print((offset, repr(match)))

```

## Supported languages

* [Python](https://github.com/tree-sitter/tree-sitter-python) ✅
* [Javascript](https://github.com/tree-sitter/tree-sitter-javascript) ✅
* [Powershell](https://github.com/airbus-cert/tree-sitter-powershell) ✅
* [PHP](https://github.com/tree-sitter/tree-sitter-php) ✅
* [Bash](https://github.com/tree-sitter/tree-sitter-bash) ✅
* [SQL](https://github.com/derekstride/tree-sitter-sql) ✅
* VBS ❌ (on going)

## Build

`pyscrings` is powered by [`maturin`](https://github.com/PyO3/maturin)

```
git clone 
cd scrings\pyscrings

pip install maturin
maturin build

```