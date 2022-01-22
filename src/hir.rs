//! Hir has the same types as ast, but with the following invarients:
//!
//! - All enum variants have a number
//! - Extensions implicit interface has been moved into interfaces list
//! - Docs have been striped with the [`doc`] module

// TODO: At some point make HIR a different type to AST
// TODO: Lint agains []u8, suggest bytes

use std::collections::BTreeSet;

use crate::{
    ast::{
        Enum, EnumField, Extension, ExtensionInterface, Func, ImplicitInterface, Interface,
        Namespace, TypeDef, TypeKind,
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
        .map(|l| lower_implicit_interface(l, &name))
        .collect::<Vec<_>>();

    new_interfaces.extend(interfaces.into_iter().map(lower_extension_interface));

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

fn lower_implicit_interface(i: ImplicitInterface, name: &str) -> ExtensionInterface {
    ExtensionInterface {
        name: name.to_owned(),
        docs: docs::lower(&i.docs),
        methods: vmap(i.methods, lower_func),
        events: vmap(i.events, lower_func),
    }
}

fn lower_extension_interface(i: ExtensionInterface) -> ExtensionInterface {
    ExtensionInterface {
        name: i.name,
        docs: docs::lower(&i.docs),
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
        // We implement rust semantics, see
        // https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=22141a2d4288b7056d1b336844f827fa

        let value = value.unwrap_or(pos);
        pos = value + 1;

        if seen.contains(&value) {
            panic!("Enum {} has duplicate value {}", name, value);
        }

        seen.insert(value);

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

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::ast::IntType;

    use super::*;

    #[test]
    fn test_enum() {
        let enm = Enum {
            backing: IntType::I16,
            fields: vec![
                EnumField {
                    name: "a".to_owned(),
                    value: None,
                },
                EnumField {
                    name: "b".to_owned(),
                    value: None,
                },
                EnumField {
                    name: "c".to_owned(),
                    value: Some(100),
                },
                EnumField {
                    name: "d".to_owned(),
                    value: None,
                },
                EnumField {
                    name: "e".to_owned(),
                    value: Some(50),
                },
                EnumField {
                    name: "f".to_owned(),
                    value: None,
                },
            ],
        };

        let lowered = Enum {
            backing: IntType::I16,
            fields: vec![
                EnumField {
                    name: "a".to_owned(),
                    value: Some(0),
                },
                EnumField {
                    name: "b".to_owned(),
                    value: Some(1),
                },
                EnumField {
                    name: "c".to_owned(),
                    value: Some(100),
                },
                EnumField {
                    name: "d".to_owned(),
                    value: Some(101),
                },
                EnumField {
                    name: "e".to_owned(),
                    value: Some(50),
                },
                EnumField {
                    name: "f".to_owned(),
                    value: Some(51),
                },
            ],
        };
        assert_eq!(lower_enum(enm), lowered);
    }
}
