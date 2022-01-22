lalrpop_util::lalrpop_mod!(
    #[allow(clippy::all)]
    pub grammar,
    "/src/grammar.rs"
);

// // Uncomment to do in tree generated
// #[allow(clippy::all)]
// pub mod grammar;

pub mod ast;
mod docs;
pub mod hir;
pub mod vfs;
