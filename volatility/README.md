# Volatility plugin for scrings

Semantic scanner base on [`tree-sitter`](https://tree-sitter.github.io/tree-sitter/)

## Supported languages

* [Python](https://github.com/tree-sitter/tree-sitter-python) ✅
* [Javascript](https://github.com/tree-sitter/tree-sitter-javascript) ✅
* [Powershell](https://github.com/airbus-cert/tree-sitter-powershell) ✅
* [PHP](https://github.com/tree-sitter/tree-sitter-php) ✅
* [Bash](https://github.com/tree-sitter/tree-sitter-bash) ✅
* [SQL](https://github.com/derekstride/tree-sitter-sql) ✅
* VBS ❌ (on going)

# Install
⚠️ To use this plugin you must install `pyscrings` in the same python env of `volatility` ⚠️

## ScringsScan

`ScringsScan` plugin will scan the kernel memory using semantic backend and only match semantic valid strings depending on the language choose.

```
git clone https://github.com/airbus-cert/scrings

vol -f [PATH_MEMORY_DUMP] -p scrings\volatility scrings.ScringsScan -l powershell

```

## VadScringsScan

`VadScringsScan` plugin will scan Windows VAD memory range using semantic scanner.

By default it will scan all processes, but it's possible to specify a PID through `--pid` argument. 

```
git clone 

vol -f [PATH_MEMORY_DUMP] -p scrings\volatility scrings.VadScringsScan -l powershell


```
