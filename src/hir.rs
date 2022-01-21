//! Hir has the same types as ast, but with the following invarients:
//!
//! - All enum variants have a number
//! - The implicit interface is None
//! - All interfaces have a version
//! - Docs have been striped with the [`doc`] module

// TODO: At some point make HIR a different type to AST

use std::collections::BTreeSet;

use crate::{
    ast::{
        Enum, EnumField, Extension, ExtensionInterface, Func, ImplicitInterface, Interface,
        Namespace, TypeDef, TypeKind, Version,
    },
    docs,
};

pub fn lower_namespace(
    Namespace {
        name,
        interfaces,
        types,
        extensions,
    }: Namespace,
) -> Namespace {
    Namespace {
        name,
        interfaces: vmap(interfaces, lower_interface),
        types: vmap(types, lower_type_def),
        extensions: vmap(extensions, lower_extension),
    }
}

fn lower_extension(
    Extension {
        name,
        docs,
        version,
        interface,
        interfaces,
        types,
    }: Extension,
) -> Extension {
    let mut new_interfaces = interface
        .into_iter()
        .map(|l| lower_implicit_interface(l, &name, version))
        .collect::<Vec<_>>();

    new_interfaces.extend(
        interfaces
            .into_iter()
            .map(|l| lower_extension_interface(l, version)),
    );

    Extension {
        name,
        docs: docs::lower(&docs),
        version,
        interface: None,
        interfaces: new_interfaces,
        types,
    }
}

fn lower_interface(i: Interface) -> Interface {
    Interface {
        docs: docs::lower(&i.docs),
        name: i.name,
        version: i.version,
        methods: vmap(i.methods, lower_func),
        events: vmap(i.events, lower_func),
    }
}

fn lower_implicit_interface(
    i: ImplicitInterface,
    name: &str,
    version: Version,
) -> ExtensionInterface {
    ExtensionInterface {
        name: name.to_owned(),
        docs: docs::lower(&i.docs),
        version: Some(version),
        methods: vmap(i.methods, lower_func),
        events: vmap(i.events, lower_func),
    }
}

fn lower_extension_interface(i: ExtensionInterface, version: Version) -> ExtensionInterface {
    ExtensionInterface {
        name: i.name,
        docs: docs::lower(&i.docs),
        version: Some(i.version.unwrap_or(version)),
        methods: vmap(i.methods, lower_func),
        events: vmap(i.events, lower_func),
    }
}
fn lower_type_def(TypeDef { name, kind, docs }: TypeDef) -> TypeDef {
    TypeDef {
        name,
        docs: docs::lower(&docs),
        kind: match kind {
            TypeKind::Struct(_) => kind,
            TypeKind::Enum(e) => TypeKind::Enum(lower_enum(e)),
        },
    }
}

fn lower_enum(Enum { backing, fields }: Enum) -> Enum {
    let mut pos = 0;
    let mut seen = BTreeSet::new();
    let mut new_fields = Vec::with_capacity(fields.len());

    for EnumField { name, value } in fields {
        let value = value.unwrap_or(pos);

        if seen.contains(&value) {
            panic!("Enum {} has duplicate value {}", name, value);
        }

        seen.insert(value);
        pos += 1;

        new_fields.push(EnumField {
            name,
            value: Some(value),
        });
    }

    Enum {
        fields: new_fields,
        backing,
    }
}

fn lower_func(m: Func) -> Func {
    let docs = docs::lower(&m.docs);
    Func { docs, ..m }
}

fn vmap<T, U, F: Fn(T) -> U>(v: Vec<T>, f: F) -> Vec<U> {
    v.into_iter().map(f).collect()
}
