use crate::id::Id;
use crate::typedef::Type;
use std::vec::Vec;

#[derive(Debug, PartialEq, Clone)]
pub enum Exp {
    Unit,
    Bool(bool),
    Int(i32),
    Float(f64),
    Not(Box<Exp>),
    Neg(Box<Exp>),
    Add(Box<Exp>, Box<Exp>),
    Sub(Box<Exp>, Box<Exp>),
    FNeg(Box<Exp>),
    FAdd(Box<Exp>, Box<Exp>),
    FSub(Box<Exp>, Box<Exp>),
    FMul(Box<Exp>, Box<Exp>),
    FDiv(Box<Exp>, Box<Exp>),
    Eq(Box<Exp>, Box<Exp>),
    LE(Box<Exp>, Box<Exp>),
    If(Box<Exp>, Box<Exp>, Box<Exp>),
    Let((Id, Type), Box<Exp>, Box<Exp>),
    LetTuple(Vec<(Id, Type)>, Box<Exp>, Box<Exp>),
    Var(Id),
    LetRec(FunDef, Box<Exp>),
    App(Box<Exp>, Vec<Box<Exp>>),
    Tuple(Vec<Box<Exp>>),
    Array(Box<Exp>, Box<Exp>),
    Get(Box<Exp>, Box<Exp>),
    Put(Box<Exp>, Box<Exp>, Box<Exp>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunDef {
    pub name: (Id, Type),
    pub args: Vec<(Id, Type)>,
    pub body: Box<Exp>,
}
