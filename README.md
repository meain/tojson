# tojson

> Convert between yaml, toml and json


## Installation

### MacOS

```
brew install meain/meain/tojson
```

### Cargo

```
cargo install tojson
```

### Source

```
git clone git@github.com:meain/tojson.git
cd tojson && cargo install --force --path .
```

## Usage

```
tojson 0.3.0
Convert from differnt formats to json

USAGE:
    tojson [FLAGS] [OPTIONS] [filename]

FLAGS:
    -h, --help       Prints help information
    -p, --pretty
    -V, --version    Prints version information

OPTIONS:
    -f, --from <from>     [default: auto]  [possible values: auto, yaml, toml, json]
    -t, --to <to>         [default: json]  [possible values: json, yaml, toml]

ARGS:
    <filename>
```

#### Example usecase

```
tojson Cargo.toml | jq -r '.package.version'
```
