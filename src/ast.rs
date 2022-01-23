use std::fmt::Display;

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
    pub methods: Vec<Func>,
    pub events: Vec<Func>,
}

pub type Version = (u64, u64, u64);

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
    Signed(SignedIntType),
    Unsigned(UnsignedIntType),
}

#[derive(Debug, debug2::Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UnsignedIntType {
    U8,
    U16,
    U32,
    U64,

    VU8,
    VU16,
    VU32,
    VU64,
}

#[derive(Debug, debug2::Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SignedIntType {
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
    Flags(Flags),
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
    // Because we need to be able to store i64::MIN and u64::MAX
    pub value: Option<i128>,
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

#[derive(Debug, debug2::Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Flags {
    pub backing: IntType,
    pub fields: Vec<FlagField>,
}

#[derive(Debug, debug2::Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FlagField {
    pub name: String,
    pub expr: Expr,
    #[serde(skip_serializing_if = "always_some")]
    pub value: Option<u64>,
}

#[derive(Debug, debug2::Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Expr {
    // These are seperate so we can pritty print them in the docs
    Num(u64),
    BNum(u64),
    BinOp(Box<Expr>, Op, Box<Expr>),
    Ident(String),
}

#[derive(Debug, debug2::Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Op {
    LShift,
    RShift,
    BAnd,
    BOr,
    BXor,
    BClear,
    // TODO: Do we want NXor
}

fn always_none<T>(x: &Option<T>) -> bool {
    // assert_matches!(x, None);
    assert!(x.is_none());
    true
}

fn always_some<T>(x: &Option<T>) -> bool {
    assert!(x.is_some());
    false
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Primitive(p) => p.fmt(f),
            Type::Custom(n) => n.fmt(f),
            Type::Array(t) => write!(f, "[]{}", t),
            Type::Dictionary(d) => write!(f, "[{}]{}", d.key, d.value),
            Type::IntType(i) => i.fmt(f),
        }
    }
}

impl Display for PrimType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            PrimType::String => "string",
            PrimType::Object => "object",
            PrimType::Uuid => "uuid",
            PrimType::Bytes => "bytes",
            PrimType::Bool => "bool",
            PrimType::Matrix4x4 => "matrix4x4",
            PrimType::F32 => "f32",
            PrimType::F64 => "f64",
        })
    }
}

impl Display for IntType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IntType::Signed(i) => i.fmt(f),
            IntType::Unsigned(u) => u.fmt(f),
        }
    }
}

impl Display for UnsignedIntType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            UnsignedIntType::U8 => "u8",
            UnsignedIntType::U16 => "u16",
            UnsignedIntType::U32 => "u32",
            UnsignedIntType::U64 => "u64",
            UnsignedIntType::VU8 => "vu8",
            UnsignedIntType::VU16 => "vu16",
            UnsignedIntType::VU32 => "vu32",
            UnsignedIntType::VU64 => "vu64",
        })
    }
}

impl Display for SignedIntType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            SignedIntType::I8 => "i8",
            SignedIntType::I16 => "i16",
            SignedIntType::I32 => "i32",
            SignedIntType::I64 => "i64",
            SignedIntType::VI8 => "vi8",
            SignedIntType::VI16 => "vi16",
            SignedIntType::VI32 => "vi32",
            SignedIntType::VI64 => "vi64",
        })
    }
}
