//! [![crates.io](https://img.shields.io/crates/v/env_ast.svg)](https://crates.io/crates/env_ast)
//! [![docs.rs](https://docs.rs/env_ast/badge.svg)](https://docs.rs/env_ast/)
//! [![license](https://img.shields.io/crates/l/env_ast)](https://github.com/WilliamVenner/env_ast/blob/master/LICENSE)
//! 
//! # env_ast
//! 
//! Dead simple procedural macro that mimics [`env!`](https://doc.rust-lang.org/std/env/index.html) but outputs AST tokens instead of a string literal.
//! 
//! **WARNING:** This macro is potentially DANGEROUS and can introduce arbitrary code execution (ACE) if used improperly. If you need this macro, make sure you REALLY need it.
//! 
//! # Usage
//! 
//! Simply add to your [Cargo.toml](https://doc.rust-lang.org/cargo/reference/manifest.html) file:
//! 
//! ```toml
//! [dependencies]
//! env_ast = "*"
//! ```
//! 
//! And in your code:
//! 
//! ```ignore
//! #[macro_use] extern crate env_ast;
//! 
//! fn it_works() {}
//! fn default_works() {}
//! 
//! fn main() {
//!     env_ast!("MY_ENV_VAR")(); // For this to compile, MY_ENV_VAR must be set to `it_works` at build time
//!     env_ast!("ENV_VAR_THAT_IS_NOT_SET", default_works)(); // You can provide a default set of tokens if the environment variable is not found
//! }
//! ```

use proc_macro::TokenStream;
use syn::parse::Parser;
use quote::ToTokens;

#[proc_macro]
pub fn env_ast(tokens: TokenStream) -> TokenStream {
	let parser = syn::punctuated::Punctuated::<syn::Expr, syn::Token![,]>::parse_separated_nonempty;
	let args = match parser.parse(tokens) {
		Ok(args) => args,
		Err(err) => return err.into_compile_error().into()
	};

	fn get_str_lit(expr: &syn::Expr) -> Option<String> {
		Some(match expr {
			syn::Expr::Lit(lit) => match &lit.lit {
				syn::Lit::Str(str) => str.value(),
				syn::Lit::ByteStr(bytes) => String::from_utf8(bytes.value()).ok()?,
				syn::Lit::Byte(byte) => byte.value().to_string(),
				syn::Lit::Char(char) => char.value().to_string(),
				syn::Lit::Verbatim(lit) => lit.to_string(),
				_ => return None
			},
			_ => return None
		})
	}

	if args.is_empty() {
		return "compile_error!(\"No arguments provided\")".parse().unwrap();
	}

	let env_name = match get_str_lit(&args[0]) {
		Some(env_name) => env_name,
		None => "compile_error!(\"Expected a string literal for environment variable name\")".parse().unwrap()
	};

	let env_var = std::env::var(env_name).ok().map(|var| match var.parse::<TokenStream>() {
		Ok(var) => var,
		Err(err) => format!("compile_error!(\"{}\")", err.to_string().replace('\\', "\\\\").replace('"', "\\\"")).parse().unwrap()
	});

	match args.len() {
		1 => env_var.unwrap_or_else(|| "compile_error!(\"Environment variable not present\")".parse().unwrap()),
		2 => env_var.unwrap_or_else(|| args[1].to_token_stream().into()),
		_ => "compile_error!(\"Too many arguments, expected 1 or 2 (\"environment variable\", [default])\")".parse().unwrap()
	}
}

#[test]
#[cfg(test)]
fn test_env_ast() {
	let output = std::process::Command::new("cargo")
		.args(&["run", "--release", "--manifest-path", concat!(env!("CARGO_MANIFEST_DIR"), "/env_ast_test/Cargo.toml")])
		.env("ENV_AST_TEST", "it_works")
		.output().unwrap();

	let status = output.status.code();

	if status == Some(123) {
		return;
	} else {
		panic!(
			"Unexpected exit code: {:?}\n\n== Stdout ==\n{}\n\n== Stderr ==\n{}",
			status,
			String::from_utf8_lossy(&output.stdout),
			String::from_utf8_lossy(&output.stderr)
		);
	}
}