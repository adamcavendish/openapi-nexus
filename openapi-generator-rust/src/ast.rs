//! Rust AST definitions

/// Rust AST node types
#[derive(Debug, Clone)]
pub enum RustNode {
    Struct(Struct),
    Enum(Enum),
    TypeAlias(TypeAlias),
    Function(Function),
    Trait(Trait),
}

#[derive(Debug, Clone)]
pub struct Struct {
    pub name: String,
    pub fields: Vec<Field>,
    pub derives: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Enum {
    pub name: String,
    pub variants: Vec<EnumVariant>,
    pub derives: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct TypeAlias {
    pub name: String,
    pub type_expr: TypeExpression,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_type: TypeExpression,
    pub visibility: Visibility,
}

#[derive(Debug, Clone)]
pub struct Trait {
    pub name: String,
    pub methods: Vec<Method>,
}

#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub type_expr: TypeExpression,
    pub optional: bool,
    pub visibility: Visibility,
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub type_expr: TypeExpression,
    pub reference: bool,
}

#[derive(Debug, Clone)]
pub struct Method {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_type: TypeExpression,
    pub visibility: Visibility,
}

#[derive(Debug, Clone)]
pub struct EnumVariant {
    pub name: String,
    pub fields: Vec<Field>,
}

#[derive(Debug, Clone)]
pub enum TypeExpression {
    Primitive(String),
    Option(Box<TypeExpression>),
    Vec(Box<TypeExpression>),
    HashMap(Box<TypeExpression>, Box<TypeExpression>),
    Reference(String),
    Tuple(Vec<TypeExpression>),
}

#[derive(Debug, Clone)]
pub enum Visibility {
    Public,
    Private,
}
