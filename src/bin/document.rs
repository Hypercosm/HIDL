use std::io::{self, ErrorKind, Write};

use anyhow::{bail, ensure, Result};
use camino::{Utf8Path, Utf8PathBuf};
use fs_err as fs;
use hidl::ast::{
    Extension, ExtensionInterface, Func, Interface, Namespace, TypeDef, TypeKind, Version,
};

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

    let w = &mut f;
    writeln!(w, "# Hypercosm Protocol Docs")?;

    doc_all(w, document_interface, &tree.interfaces, "## Interfaces")?;
    doc_all(w, document_type, &tree.types, "## Types")?;

    if !tree.extensions.is_empty() {
        writeln!(w, "## Extensions")?;
        for i in &tree.extensions {
            let fname = document_extension(i, out)?;
            writeln!(w, "- [{}]({})", i.name, fname)?;
        }
    }
    Ok(())
}

fn document_extension(ext: &Extension, out: &Utf8Path) -> Result<Utf8PathBuf> {
    let file_name = Utf8PathBuf::from(&ext.name).with_extension("md");

    let mut f = fs::File::create(out.join(&file_name))?;
    let w = &mut f;

    writeln!(w, "# Extension {}", ext.name)?;
    write_version(w, ext.version)?;
    writeln!(w, "{}", ext.docs)?;

    doc_all(w, document_einterface, &ext.interfaces, "## Interfaces")?;
    doc_all(w, document_type, &ext.types, "## Types")?;

    Ok(file_name)
}

fn document_type(w: &mut dyn Write, ty: &TypeDef) -> Result<()> {
    writeln!(
        w,
        "### {} `{}`",
        match ty.kind {
            TypeKind::Struct(_) => "struct",
            TypeKind::Enum(_) => "enum",
        },
        ty.name
    )?;

    writeln!(w, "{}", ty.docs)?;

    match &ty.kind {
        TypeKind::Struct(s) => {
            for field in &s.fields {
                writeln!(w, "- `{}`: `{}`", field.name, field.ty)?;
            }
        }
        TypeKind::Enum(e) => {
            for field in &e.fields {
                writeln!(w, "- `{}`", field.name)?;
            }
        }
    }

    Ok(())
}

// TODO: Merge document_interface and document_einterface
fn document_interface(w: &mut dyn Write, i: &Interface) -> Result<()> {
    writeln!(w, "### {}", i.name)?;
    write_version(w, i.version)?;
    writeln!(w, "{}", i.docs)?;

    doc_all(w, document_func, &i.events, "#### Events")?;
    doc_all(w, document_func, &i.methods, "#### Methods")?;

    Ok(())
}

fn document_einterface(w: &mut dyn Write, i: &ExtensionInterface) -> Result<()> {
    writeln!(w, "### {}", i.name)?;
    writeln!(w, "{}", i.docs)?;

    doc_all(w, document_func, &i.events, "#### Events")?;
    doc_all(w, document_func, &i.methods, "#### Methods")?;

    Ok(())
}

fn document_func(w: &mut dyn Write, f: &Func) -> Result<()> {
    write!(w, "##### `{}(", f.name)?;
    // TODO: Link to types
    if let [args @ .., larg] = &f.args[..] {
        for arg in args {
            write!(w, "{}: {}, ", arg.name, arg.ty)?;
        }
        write!(w, "{}: {}", larg.name, larg.ty)?;
    }

    write!(w, ")")?;
    if let Some(ret) = &f.ret {
        write!(w, " -> {}", ret)?;
    }
    writeln!(w, "`")?;

    writeln!(w, "{}", f.docs)?;

    Ok(())
}

fn doc_all<T, F: Fn(&mut dyn Write, &T) -> Result<()>>(
    w: &mut dyn Write,
    f: F,
    items: &[T],

    name: &str,
) -> Result<()> {
    if !items.is_empty() {
        writeln!(w, "{}", name)?;
        for i in items {
            f(w, i)?;
        }
    }
    Ok(())
}

fn write_version(w: &mut dyn Write, v: Version) -> io::Result<()> {
    writeln!(w, "*v{}.{}.{}*\n", v.0, v.1, v.2)
}
