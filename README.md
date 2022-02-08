[![crates.io](https://img.shields.io/crates/v/env_ast.svg)](https://crates.io/crates/env_ast)
[![docs.rs](https://docs.rs/env_ast/badge.svg)](https://docs.rs/env_ast/)
[![license](https://img.shields.io/crates/l/env_ast)](https://github.com/WilliamVenner/env_ast/blob/master/LICENSE)

# env_ast

Dead simple procedural macro that mimics [`env!`](https://doc.rust-lang.org/std/env/index.html) but outputs AST tokens instead of a string literal.

**WARNING:** This macro is potentially DANGEROUS and can introduce arbitrary code execution (ACE) if used improperly. If you need this macro, make sure you REALLY need it.

# Usage

Simply add to your [Cargo.toml](https://doc.rust-lang.org/cargo/reference/manifest.html) file:

```toml
[dependencies]
env_ast = "*"
```

And in your code:

```rust
#[macro_use] extern crate env_ast;

fn it_works() {}
fn default_works() {}

fn main() {
    env_ast!("MY_ENV_VAR")(); // For this to compile, MY_ENV_VAR must be set to `it_works` at build time
    env_ast!("ENV_VAR_THAT_IS_NOT_SET", default_works)(); // You can provide a default set of tokens if the environment variable is not found
}
```