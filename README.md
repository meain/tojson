# tojson

> Convert from yaml and toml to json


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

## Example usecase

```
tojson Cargo.toml | jq -r '.package.version'
```
