[![License: MIT](https://img.shields.io/badge/License-MIT-blue?style=flat-square)](LICENSE)
[![Crates.io](https://img.shields.io/crates/v/n64-pac?style=flat-square)](https://crates.io/crates/n64-pac)
[![Documentation](https://img.shields.io/docsrs/n64-pac?style=flat-square)](https://docs.rs/n64-pac)

### Description
This crate is a low-level abstraction (aka a [Peripheral Access Crate](https://rust-embedded.github.io/book/start/registers.html))
over the CPU and memory-mapped registers available on the Nintendo 64 console.

Due to the low-level nature of the API, most projects (games especially) are unlikely to use this crate directly.

### API Coverage
| Peripheral | Registers | Complete |
|:----------:|:---------:|:--------:|
|    CP0     | 32 of 32  | &#10003; |
|    CP1     |  0 of 32  | &#10005; |
|     MI     |  4 of 4   | &#10003; |
|     VI     | 15 of 15  | &#10003; |
|     AI     |  0 of ?   | &#10005; |
|     PI     |  0 of ?   | &#10005; |
|     RI     |  0 of ?   | &#10005; |
|     SI     |  6 of 6*  | &#10003; |

_* The SI likely contains more registers that haven't been fully researched._


### Usage
In your project's `Cargo.toml`:
```Toml
[dependencies]
n64-pac = "0.2.0"
```

Refer to the [docs](https://docs.rs/n64-pac) for examples and details regarding safety.

This crate is only intended to be used in the N64 embedded environment. 

### Nightly Rust
Please note this crate requires a nightly rust toolchain in order to use nightly-only inline assembly features and
arbitrary discriminants.