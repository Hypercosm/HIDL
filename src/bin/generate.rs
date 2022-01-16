use anyhow::{ensure, Result};
use fs_err as fs;

fn main() -> Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    ensure!(
        args.len() == 3,
        "Usage: generate <input.hidl> <output.json>"
    );

    let hidl = fs::read_to_string(&args[1])?;

    // TODO: use a propper error: `?` cant be used here because the error borrows the input string
    let tree = hidl::grammar::NamespaceParser::new().parse(&hidl);

    let tree = lalrpop_codespan::report_error(tree, &args[1], &hidl)?;
    let tree = hidl::hir::lower_namespace(tree);

    let tree_json = serde_json::to_string_pretty(&tree)?;

    fs::write(&args[2], tree_json)?;

    Ok(())
}
