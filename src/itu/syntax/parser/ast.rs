use std::rc::Rc;

use super::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Block(Vec<Statement>),
    Number(f64),
    Bool(bool),
    Str(Rc<String>),
    Char(char),
    Identifier(Rc<String>),
    Operation(Operation),
    Call(Call),
    Lambda(Lambda),
    Array(Vec<Rc<Expression>>),
    Index(Index),
    EOF,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Operation {
    pub left:  Rc<Expression>,
    pub op:    Operand,
    pub right: Rc<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Call {
    pub callee: Rc<Expression>,
    pub args:   Vec<Rc<Expression>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Lambda {
    pub t:      Rc<Type>,
    pub params: Vec<(Option<Type>, Rc<String>)>,
    pub body:   Rc<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Index {
    pub id:    Rc<Expression>,
    pub index: Rc<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Expression(Rc<Expression>),
    Assignment(Assignment),
    Definition(Definition),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Assignment {
    pub left:  Rc<Expression>,
    pub right: Rc<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Definition {
    pub t:     Option<Type>,
    pub name:  Rc<Expression>,
    pub right: Option<Rc<Expression>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operand {
    Pow,
    Mul, Div, Mod,
    Add, Sub,
    Equal, NEqual,
    Lt, Gt, LtEqual, GtEqual,
}

impl Operand {
    pub fn from_str(v: &str) -> Option<(Operand, u8)> {
        match v {
            "^"   => Some((Operand::Pow, 0)),
            "*"   => Some((Operand::Mul, 1)),
            "/"   => Some((Operand::Div, 1)),
            "%"   => Some((Operand::Mod, 1)),
            "+"   => Some((Operand::Add, 2)),
            "-"   => Some((Operand::Sub, 2)),
            "=="  => Some((Operand::Equal, 3)),
            "!="  => Some((Operand::NEqual, 3)),
            "<"   => Some((Operand::Lt, 4)),
            ">"   => Some((Operand::Gt, 4)),
            "<="  => Some((Operand::LtEqual, 4)),
            ">="  => Some((Operand::GtEqual, 4)),
            _     => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Mut(Option<Rc<Type>>),
    Array(Rc<Type>, Option<Expression>),
    Identifier(Rc<String>),
    I08,
    I16,
    I32,
    I64,
    I128,
    F32,
    F64,
    U08,
    U16,
    U32,
    U64,
    U128,
    Char,
    Str,
    Bool,
    Any,
}

#[allow(unused)]
impl Type {
    pub fn compare(&self, other: &Type) -> bool {
        if self == &Type::Any || other == &Type::Any {
            true
        } else {
            self == other
        }
    }

    pub fn from(v: &Token) -> Option<Type> {
        match v.token_type {
            TokenType::Type => match v.content().as_str() {
                "i08"  => Some(Type::I08),
                "i32"  => Some(Type::I32),
                "i16"  => Some(Type::I16),
                "i64"  => Some(Type::I64),
                "i128" => Some(Type::I128),
                "f32"  => Some(Type::F32),
                "f64"  => Some(Type::F64),
                "u08"  => Some(Type::U08),
                "u16"  => Some(Type::U16),
                "u32"  => Some(Type::U32),
                "u64"  => Some(Type::U64),
                "u128" => Some(Type::U128),
                "char" => Some(Type::Char),
                "str"  => Some(Type::Str),
                "bool" => Some(Type::Bool),
                "any"  => Some(Type::Any),
                _      => None,
            },
            
            TokenType::Identifier => Some(Type::Identifier(Rc::new(v.content().clone()))),
            _ => None,
        }
    }
}
