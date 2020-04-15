![GitHub Workflow Status](https://img.shields.io/github/workflow/status/rodolfoghi/cnpj-util-rust/Rust) ![Crates.io](https://img.shields.io/crates/v/cnpj-util) ![Crates.io](https://img.shields.io/crates/d/cnpj-util)


# CNPJ util

CNPJ util inspired in [brazilian-utils/cnpj](https://github.com/brazilian-utils/brazilian-utils/blob/master/src/utilities/cnpj/index.ts).

## Usage

Add the following to your `Cargo.toml`:
```rust
[dependencies]
cnpj_util = "0.1.0"
```

## Examples

Format:
```rust
use cnpj_util as cnpj;

fn main() {
    println!("{}", cnpj::format("46843485000186")); // 46.843.485/0001-86
    println!("{}", cnpj::format("468434850001860000000000")); // 46.843.485/0001-86
    println!("{}", cnpj::format("46.?ABC843.485/0001-86abc")); // 46.843.485/0001-86
}
```
