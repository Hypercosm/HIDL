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
    writeln!(f, "# Hypercosm Protocol Docs")?;

    if !tree.interfaces.is_empty() {
        writeln!(f, "## Interfaces")?;
        for i in &tree.interfaces {
            document_interface(&mut f, i)?;
        }
    }

    if !tree.types.is_empty() {
        writeln!(f, "## Types")?;
        for i in &tree.types {
            document_type(&mut f, i)?;
        }
    }

    if !tree.extensions.is_empty() {
        writeln!(f, "## Extensions")?;
        for i in &tree.extensions {
            let fname = document_extension(i, out)?;
            writeln!(f, "- [{}]({})", i.name, fname)?;
        }
    }
    Ok(())
}

fn document_extension(ext: &Extension, out: &Utf8Path) -> Result<Utf8PathBuf> {
    let file_name = Utf8PathBuf::from(&ext.name).with_extension("md");

    let mut f = fs::File::create(out.join(&file_name))?;

    writeln!(f, "# Extension {}", ext.name)?;
    write_version(&mut f, ext.version)?;

    writeln!(f, "{}", ext.docs)?;

    if !ext.interfaces.is_empty() {
        writeln!(f, "## Interfaces")?;
        for i in &ext.interfaces {
            document_einterface(&mut f, i)?;
        }
    }

    if !ext.types.is_empty() {
        writeln!(f, "## Types")?;
        for i in &ext.types {
            document_type(&mut f, i)?;
        }
    }

    Ok(file_name)
}

fn document_type(f: &mut dyn Write, ty: &TypeDef) -> Result<()> {
    writeln!(
        f,
        "### {} `{}`",
        match ty.kind {
            TypeKind::Struct(_) => "struct",
            TypeKind::Enum(_) => "enum",
        },
        ty.name
    )?;

    writeln!(f, "{}", ty.docs)?;

    match &ty.kind {
        TypeKind::Struct(s) => {
            for field in &s.fields {
                writeln!(f, "- `{}`: `{}`", field.name, field.ty)?;
            }
        }
        TypeKind::Enum(e) => {
            for field in &e.fields {
                writeln!(f, "- `{}`", field.name)?;
            }
        }
    }

    Ok(())
}

// TODO: Merge document_interface and document_einterface
fn document_interface(f: &mut dyn Write, i: &Interface) -> Result<()> {
    writeln!(f, "### {}", i.name)?;
    write_version(f, i.version)?;
    writeln!(f, "{}", i.docs)?;

    if !i.events.is_empty() {
        writeln!(f, "#### Events")?;
        for e in &i.events {
            document_func(f, e)?;
        }
    }

    if !i.methods.is_empty() {
        writeln!(f, "#### Methods")?;
        for m in &i.methods {
            document_func(f, m)?;
        }
    }

    Ok(())
}

fn document_einterface(f: &mut dyn Write, i: &ExtensionInterface) -> Result<()> {
    writeln!(f, "### {}", i.name)?;

    if let Some(version) = i.version {
        write_version(f, version)?;
    }

    writeln!(f, "{}", i.docs)?;

    if !i.events.is_empty() {
        writeln!(f, "#### Events")?;
        for e in &i.events {
            document_func(f, e)?;
        }
    }

    if !i.methods.is_empty() {
        writeln!(f, "#### Methods")?;
        for m in &i.methods {
            document_func(f, m)?;
        }
    }

    Ok(())
}

fn document_func(w: &mut dyn Write, f: &Func) -> Result<()> {
    write!(w, "##### `{}(", f.name)?;
    // TODO: Link to type?
    // TODO: No trailing comma

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

fn write_version(f: &mut dyn Write, v: Version) -> io::Result<()> {
    writeln!(f, "*v{}.{}.{}*\n", v.0, v.1, v.2)
}
