use serde::{Deserialize, Serialize};

#[derive(Debug, debug2::Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Namespace {
    pub name: String,
    pub interfaces: Vec<Interface>,
    pub types: Vec<TypeDef>,
    pub extensions: Vec<Extension>,
}

#[derive(Debug, debug2::Debug, Clone, PartialEq, Serialize, Deserialize)]

pub struct Extension {
    pub name: String,
    pub version: Version,
    pub docs: String,
    #[serde(default, skip_serializing_if = "always_none")]
    pub interface: Option<ImplicitInterface>,
    pub interfaces: Vec<ExtensionInterface>,
    pub types: Vec<TypeDef>,
}

#[derive(Debug, debug2::Debug, Clone, PartialEq, Serialize, Deserialize)]

pub struct Interface {
    pub name: String,
    pub docs: String,
    pub version: Version,
    pub methods: Vec<Func>,
    pub events: Vec<Func>,
}

#[derive(Debug, debug2::Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImplicitInterface {
    // Gets version and name from extension
    pub docs: String,
    pub methods: Vec<Func>,
    pub events: Vec<Func>,
}

#[derive(Debug, debug2::Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExtensionInterface {
    pub name: String,
    pub docs: String,
    // In HIR, may get version from extension
    //
    // TODO: Assert is present during (de)serialization
    pub version: Option<Version>,
    pub methods: Vec<Func>,
    pub events: Vec<Func>,
}

pub type Version = (u8, u8, u8);

#[derive(Debug, debug2::Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Func {
    pub name: String,
    pub docs: String,
    pub args: Vec<Arg>,
    pub ret: Option<Type>,
}

#[derive(Debug, debug2::Debug, Clone, PartialEq, Serialize, Deserialize)]

pub enum Type {
    Primitive(PrimType),
    Custom(String),
    Array(Box<Type>),
    Dictionary(Box<Dictionary>),
    IntType(IntType),
}

#[derive(Debug, debug2::Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Dictionary {
    pub key: Type,
    pub value: Type,
}

#[derive(Debug, debug2::Debug, Clone, PartialEq, Serialize, Deserialize)]

pub enum PrimType {
    String,
    Object,
    Uuid,
    Bytes,
    Bool,
    Matrix4x4,
    F32,
    F64,
}

#[derive(Debug, debug2::Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum IntType {
    U8,
    U16,
    U32,
    U64,

    VU8,
    VU16,
    VU32,
    VU64,

    I8,
    I16,
    I32,
    I64,

    VI8,
    VI16,
    VI32,
    VI64,
}

#[derive(Debug, debug2::Debug, Clone, PartialEq, Serialize, Deserialize)]

pub struct Arg {
    pub name: String,
    pub ty: Type,
}

#[derive(Debug, debug2::Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypeDef {
    pub name: String,
    pub docs: String,
    pub kind: TypeKind,
}

#[derive(Debug, debug2::Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TypeKind {
    Struct(Struct),
    Enum(Enum),
}

#[derive(Debug, debug2::Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Enum {
    pub backing: IntType,
    pub fields: Vec<EnumField>,
}

#[derive(Debug, debug2::Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnumField {
    // TODO: Should we allow docs on fields
    pub name: String,
    #[serde(default, skip_serializing_if = "always_some")]
    pub value: Option<i64>,
}

#[derive(Debug, debug2::Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Struct {
    pub fields: Vec<StructField>,
}

#[derive(Debug, debug2::Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StructField {
    // TODO: Should we allow docs on fields
    pub name: String,
    pub ty: Type,
}

fn always_none<T>(x: &Option<T>) -> bool {
    // assert_matches!(x, None);
    assert!(x.is_none());
    true
}

fn always_some<T>(x: &Option<T>) -> bool {
    assert!(x.is_some());
    true
}
