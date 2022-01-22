use std::io::Write;

use anyhow::{ensure, Result};
use fs_err as fs;

use hidl::vfs;

fn main() -> Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    ensure!(
        args.len() == 3,
        "Usage: generate <input.hidl> <output.json>"
    );

    let hidl = fs::read_to_string(&args[1])?;

    let tree = hidl::grammar::NamespaceParser::new().parse(&hidl);
    let tree = lalrpop_codespan::report_error(tree, &args[1], &hidl)?;
    // debug2::dbg!(&tree);
    let tree = hidl::hir::lower_namespace(tree);

    let tree_json = serde_json::to_string_pretty(&tree)?;

    // TODO: Convenience methods in VFS
    let mut vfs = vfs::FS::new();
    let file = vfs.open(&args[2]);
    file.write_all(tree_json.as_bytes())?;
    vfs.save()?;

    Ok(())
}
