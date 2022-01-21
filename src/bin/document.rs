use std::io::{ErrorKind, Write};

use anyhow::{bail, ensure, Result};
use camino::{Utf8Path, Utf8PathBuf};
use fs_err as fs;
use hidl::ast::{Arg, Extension, ExtensionInterface, Func, Interface, Namespace};

fn main() -> Result<()> {
    let args = std::env::args().collect::<Vec<_>>();

    ensure!(
        args.len() == 3,
        "Useage: document <input.hidl> <output_directory>"
    );

    let out_dir = Utf8Path::new(&args[2]);

    let md = fs::metadata(out_dir);
    match md {
        Ok(md) => ensure!(md.file_type().is_dir()),
        Err(err) => {
            if err.kind() == ErrorKind::NotFound {
                fs::create_dir_all(out_dir)?;
            } else {
                bail!(err);
            }
        }
    }

    let hidl = fs::read_to_string(&args[1])?;
    let tree = hidl::grammar::NamespaceParser::new().parse(&hidl);
    let tree = lalrpop_codespan::report_error(tree, &args[1], &hidl)?;
    let tree = hidl::hir::lower_namespace(tree);

    document(&tree, out_dir)?;

    Ok(())
}

fn document(tree: &Namespace, out: &Utf8Path) -> Result<()> {
    let mut f = fs::File::create(out.join("README.md"))?;
    writeln!(f, "# Hypercosm Protocol Docs")?;

    writeln!(f, "## Interfaces")?;

    for i in &tree.interfaces {
        document_interface(&mut f, i)?;
    }

    // TODO: Document types

    writeln!(f, "## Extensions")?;

    for i in &tree.extensions {
        let fname = document_extension(i, out)?;

        writeln!(f, "- [{}]({})", i.name, fname)?;
    }

    Ok(())
}

fn document_extension(ext: &Extension, out: &Utf8Path) -> Result<Utf8PathBuf> {
    let file_name = Utf8PathBuf::from(&ext.name).with_extension("md");

    let mut f = fs::File::create(out.join(&file_name))?;

    writeln!(f, "# Extension {}", ext.name)?;

    writeln!(f, "{}", ext.docs)?;

    writeln!(f, "## Interfaces")?;

    for i in &ext.interfaces {
        document_einterface(&mut f, i)?;
    }

    Ok(file_name)
}

fn document_interface(f: &mut dyn Write, i: &Interface) -> Result<()> {
    writeln!(f, "### {}", i.name)?;
    writeln!(f, "*v{}.{}.{}*\n", i.version.0, i.version.1, i.version.2)?;
    writeln!(f, "{}", i.docs)?;

    writeln!(f, "#### Methods")?;
    for m in &i.methods {
        document_func(f, m)?;
    }

    writeln!(f, "#### Events")?;
    for e in &i.events {
        document_func(f, e)?;
    }

    Ok(())
}

fn document_einterface(f: &mut dyn Write, i: &ExtensionInterface) -> Result<()> {
    writeln!(f, "### {}", i.name)?;

    if let Some(version) = i.version {
        writeln!(f, "*v{}.{}.{}*\n", version.0, version.1, version.2)?;
    }

    writeln!(f, "{}", i.docs)?;

    writeln!(f, "#### Methods")?;
    for m in &i.methods {
        document_func(f, m)?;
    }

    writeln!(f, "#### Events")?;
    for e in &i.events {
        document_func(f, e)?;
    }

    Ok(())
}

fn document_func(w: &mut dyn Write, f: &Func) -> Result<()> {
    write!(w, "##### `{}(", f.name)?;
    // TODO: Link to type?
    // TODO: No trailing comma
    for Arg { name, ty } in &f.args {
        write!(w, "{}: {},", name, ty)?;
    }
    writeln!(w, ")`")?;

    writeln!(w, "{}", f.docs)?;

    Ok(())
}
