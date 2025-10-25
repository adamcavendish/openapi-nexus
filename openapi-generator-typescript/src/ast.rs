//! TypeScript AST definitions

/// TypeScript AST node types
#[derive(Debug, Clone)]
pub enum TsNode {
    Interface(Interface),
    TypeAlias(TypeAlias),
    Enum(Enum),
    Function(Function),
    Class(Class),
}

#[derive(Debug, Clone)]
pub struct Interface {
    pub name: String,
    pub properties: Vec<Property>,
    pub extends: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct TypeAlias {
    pub name: String,
    pub type_expr: TypeExpression,
}

#[derive(Debug, Clone)]
pub struct Enum {
    pub name: String,
    pub variants: Vec<EnumVariant>,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_type: TypeExpression,
}

#[derive(Debug, Clone)]
pub struct Class {
    pub name: String,
    pub properties: Vec<Property>,
    pub methods: Vec<Method>,
}

#[derive(Debug, Clone)]
pub struct Property {
    pub name: String,
    pub type_expr: TypeExpression,
    pub optional: bool,
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub type_expr: TypeExpression,
    pub optional: bool,
}

#[derive(Debug, Clone)]
pub struct Method {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_type: TypeExpression,
}

#[derive(Debug, Clone)]
pub struct EnumVariant {
    pub name: String,
    pub value: Option<String>,
}

#[derive(Debug, Clone)]
pub enum TypeExpression {
    Primitive(String),
    Union(Vec<TypeExpression>),
    Array(Box<TypeExpression>),
    Object(Vec<Property>),
    Reference(String),
    Literal(String),
}
