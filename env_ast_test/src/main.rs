fn main() {
	#[cfg(not(debug_assertions))] {
		fn it_works() {}
		fn default_works() {}

		env_ast::env_ast!("ENV_AST_TEST")();
		env_ast::env_ast!("ENV_AST_DO_NOT_SET", default_works)();
		std::process::exit(123);
	}
}
