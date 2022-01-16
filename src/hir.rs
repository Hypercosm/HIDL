//! Hir has the same types as ast, but with the following invarients:
//!
//! - All enum variants have a number
//! - The implicit interface is None
//! - All interfaces have a version

use std::collections::BTreeSet;

use crate::ast::{
    Enum, EnumField, Extension, ExtensionInterface, ImplicitInterface, Namespace, TypeDef,
    TypeKind, Version,
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
        interfaces,
        types: types.into_iter().map(lower_type_def).collect(),
        extensions: extensions.into_iter().map(lower_extension).collect(),
    }
}

fn lower_extension(
    Extension {
        name,
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
        version,
        interface: None,
        interfaces: new_interfaces,
        types,
    }
}

fn lower_implicit_interface(
    i: ImplicitInterface,
    name: &str,
    version: Version,
) -> ExtensionInterface {
    ExtensionInterface {
        name: name.to_owned(),
        version: Some(version),
        methods: i.methods,
        events: i.events,
    }
}

fn lower_extension_interface(i: ExtensionInterface, version: Version) -> ExtensionInterface {
    ExtensionInterface {
        name: i.name,
        version: Some(i.version.unwrap_or(version)),
        methods: i.methods,
        events: i.events,
    }
}
fn lower_type_def(TypeDef { name, kind }: TypeDef) -> TypeDef {
    TypeDef {
        name,
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
