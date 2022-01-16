fn main() {
    lalrpop::Configuration::new()
        // Uncoment when developing local to inspect generated parser for type errors
        // .generate_in_source_tree()
        .process()
        .unwrap();
}

