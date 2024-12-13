# pyscrings

Python bindings for `scrings` powered by PyO3 and maturin

`scrings` is a strings utility that will output only semantically valid strings based on tree-sitter grammar. For each script language we made a list of semantic nodes discriminant enough to detect the target language.

The main purpose is to support [`volatility`](../volatility).

## Install 

`pyscrings` is available on PyPi:

```
pip install pyscrings
```

## Usage

`pyscrings` is using FileObject interface and will output a generator.

```
from pyscrings import powershell, javascript, php, bash, python, sql

with open("path_to_dump", "rb") as f:
    for (offset, match) in powershell(f):
        print((offset, repr(match)))

```

Another example using a BytesIO buffer :
```
import pyscrings, io
list(pyscrings.sql(io.BytesIO(b"********* select * from table *************"), 4))

[(10, 'select * from table')]
```

## Docs

### Powershell

```
def powershell(buffer : BinaryIO, step: Optional[int]) -> Generator[(int, str)]:
    '''
    Powershell strings with semantic validation
    
    :param buffer: input buffer to parse
    :param step: strings step use to find printable strings (default = 20)
    :returns: genrator of offset, valid powershell strings
    '''
```

### Sql

```
def sql(buffer : BinaryIO, step: Optional[int]) -> Generator[(int, str)]:
    '''
    SQL strings with semantic validation
    
    :param buffer: input buffer to parse
    :param step: strings step use to find printable strings (default = 20)
    :returns: genrator of offset, valid sql strings
    '''
```

### Javascript

```
def javascript(buffer : BinaryIO, step: Optional[int]) -> Generator[(int, str)]:
    '''
    Javascript strings with semantic validation
    
    :param buffer: input buffer to parse
    :param step: strings step use to find printable strings (default = 20)
    :returns: genrator of offset, valid javascript strings
    '''
```

### Bash

```
def bash(buffer : BinaryIO, step: Optional[int]) -> Generator[(int, str)]:
    '''
    Bash strings with semantic validation
    
    :param buffer: input buffer to parse
    :param step: strings step use to find printable strings (default = 20)
    :returns: genrator of offset, valid bash strings
    '''
```

### PHP

```
def php(buffer : BinaryIO, step: Optional[int]) -> Generator[(int, str)]:
    '''
    PHP strings with semantic validation
    
    :param buffer: input buffer to parse
    :param step: strings step use to find printable strings (default = 20)
    :returns: genrator of offset, valid php strings
    '''
```

### Python

```
def python(buffer : BinaryIO, step: Optional[int]) -> Generator[(int, str)]:
    '''
    Python strings with semantic validation
    
    :param buffer: input buffer to parse
    :param step: strings step use to find printable strings (default = 20)
    :returns: genrator of offset, valid python strings
    '''
```

## Build

`pyscrings` is powered by [`maturin`](https://github.com/PyO3/maturin)

```
git clone https://github.com/airbus-cert/scrings
cd scrings\pyscrings

pip install maturin
maturin build --profile release

pip install target\wheels\*.whl

```