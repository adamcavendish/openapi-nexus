//! TypeScript AST definitions

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// TypeScript AST node types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TsNode {
    Interface(Interface),
    TypeAlias(TypeAlias),
    Enum(Enum),
    Function(Function),
    Class(Class),
    Import(Import),
    Export(Export),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Interface {
    pub name: String,
    pub properties: Vec<Property>,
    pub extends: Vec<String>,
    pub generics: Vec<Generic>,
    pub documentation: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeAlias {
    pub name: String,
    pub type_expr: TypeExpression,
    pub generics: Vec<Generic>,
    pub documentation: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enum {
    pub name: String,
    pub variants: Vec<EnumVariant>,
    pub is_const: bool,
    pub documentation: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<TypeExpression>,
    pub is_async: bool,
    pub is_export: bool,
    pub documentation: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Class {
    pub name: String,
    pub properties: Vec<Property>,
    pub methods: Vec<Method>,
    pub extends: Option<String>,
    pub implements: Vec<String>,
    pub generics: Vec<Generic>,
    pub is_export: bool,
    pub documentation: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Property {
    pub name: String,
    pub type_expr: TypeExpression,
    pub optional: bool,
    pub documentation: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub type_expr: Option<TypeExpression>,
    pub optional: bool,
    pub default_value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Method {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<TypeExpression>,
    pub is_async: bool,
    pub is_static: bool,
    pub visibility: Visibility,
    pub documentation: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumVariant {
    pub name: String,
    pub value: Option<String>,
    pub documentation: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TypeExpression {
    Primitive(PrimitiveType),
    Union(Vec<TypeExpression>),
    Intersection(Vec<TypeExpression>),
    Array(Box<TypeExpression>),
    Object(HashMap<String, TypeExpression>),
    Reference(String),
    Generic(String),
    Function(Box<FunctionSignature>),
    Literal(String),
    IndexSignature(String, Box<TypeExpression>),
    Tuple(Vec<TypeExpression>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PrimitiveType {
    String,
    Number,
    Boolean,
    Null,
    Undefined,
    Any,
    Unknown,
    Void,
    Never,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionSignature {
    pub parameters: Vec<Parameter>,
    pub return_type: Option<Box<TypeExpression>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Generic {
    pub name: String,
    pub constraint: Option<TypeExpression>,
    pub default: Option<TypeExpression>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Visibility {
    Public,
    Private,
    Protected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Import {
    pub module: String,
    pub imports: Vec<ImportSpecifier>,
    pub is_type_only: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportSpecifier {
    pub name: String,
    pub alias: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Export {
    pub specifier: ExportSpecifier,
    pub is_type_only: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExportSpecifier {
    Named(String),
    Default(String),
    All(String),
    From(String, Vec<String>),
}
