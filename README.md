![GitHub Workflow Status](https://img.shields.io/github/workflow/status/rodolfoghi/cnpj-util-rust/Rust) ![Crates.io](https://img.shields.io/crates/v/cnpj-util) ![Crates.io](https://img.shields.io/crates/d/cnpj-util) ![GitHub issues](https://img.shields.io/github/issues/rodolfoghi/cnpj-util-rust)


# CNPJ util

CNPJ util inspired in [brazilian-utils/cnpj](https://github.com/brazilian-utils/brazilian-utils/blob/master/src/utilities/cnpj/index.ts).

## Usage

Add the following to your `Cargo.toml`:
```rust
[dependencies]
cnpj_util = "0.1.2"
```

## Examples

### Format:
```rust
use cnpj_util as cnpj;

fn main() {
    println!("{}", cnpj::format("46843485000186")); // 46.843.485/0001-86
    println!("{}", cnpj::format("468434850001860000000000")); // 46.843.485/0001-86
    println!("{}", cnpj::format("46.?ABC843.485/0001-86abc")); // 46.843.485/0001-86
}
```

### Validate:
```rust
use cnpj_util as cnpj;

fn main() {
    assert_eq!(false, is_valid("12312312312"));
    assert_eq!(false, is_valid("6ad0.t391.9asd47/0ad001-00"));
    assert_eq!(true, is_valid("13723705000189"));
    assert_eq!(true, is_valid("60.391.947/0001-00"));
}
```