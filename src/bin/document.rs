use std::io::{self, ErrorKind, Write};

use anyhow::{bail, ensure, Result};
use camino::{Utf8Path, Utf8PathBuf};
use fs_err as fs;
use heck::ToTitleCase;
use hidl::{
    ast::{Extension, ExtensionInterface, Func, Interface, Namespace, TypeDef, TypeKind, Version},
    vfs::{self, File},
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
    let mut fs = hidl::vfs::FS::new();

    // let w = fs.open();
    let mut readme = File::new();
    let w = &mut readme;

    let mut pages = vec![];

    writeln!(w, "# Hypercosm Protocol Docs")?;

    doc_all(w, document_interface, &tree.interfaces, "## Interfaces")?;
    doc_all(w, document_type, &tree.types, "## Types")?;

    pages.push(("Core".to_owned(), "README.md".to_owned()));

    if !tree.extensions.is_empty() {
        writeln!(w, "## Extensions")?;
        for i in &tree.extensions {
            let fname = document_extension(i, out, &mut fs)?;
            writeln!(w, "- [{}]({})", i.name, &fname)?;
            pages.push((i.name.to_title_case(), fname.into_string()));
        }
    }

    let sumarry = fs.open(out.join("SUMMARY.md"));

    for (title, link) in pages {
        writeln!(sumarry, "- [{}]({})", title, link)?;
    }

    fs.add_file(readme, out.join("README.md"));

    fs.save()?;

    Ok(())
}

fn document_extension(ext: &Extension, out: &Utf8Path, fs: &mut vfs::FS) -> Result<Utf8PathBuf> {
    let file_name = Utf8PathBuf::from(&ext.name).with_extension("md");

    let w = fs.open(out.join(&file_name));

    writeln!(w, "# Extension `{}`", ext.name)?;
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
            TypeKind::Struct(_) => "Struct",
            TypeKind::Enum(_) => "Enum",
            TypeKind::Flags(_) => "Flags",
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
        TypeKind::Flags(f) => {
            for field in &f.fields {
                writeln!(w, "- `{} = {:b}`", field.name, field.value)?;
            }
        }
    }

    Ok(())
}

// TODO: Merge document_interface and document_einterface
fn document_interface(w: &mut dyn Write, i: &Interface) -> Result<()> {
    writeln!(w, "### Interface `{}`", i.name)?;
    write_version(w, i.version)?;
    writeln!(w, "{}", i.docs)?;

    doc_all(w, document_func, &i.events, "#### Events")?;
    doc_all(w, document_func, &i.methods, "#### Methods")?;

    Ok(())
}

fn document_einterface(w: &mut dyn Write, i: &ExtensionInterface) -> Result<()> {
    writeln!(w, "### Interface `{}`", i.name)?;
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
