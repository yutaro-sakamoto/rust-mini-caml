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
    Var(Id),
    LetRec(Box<FunDef>, Box<Exp>),
    App(Vec<(Box<Exp>, Box<Exp>)>),
    Tpule(Vec<Exp>),
    Array(Box<Exp>, Box<Exp>),
    Get(Box<Exp>, Box<Exp>),
    Put(Box<Exp>, Box<Exp>, Box<Exp>),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct FunDef {
    name: (Id, Type),
    args: Vec<(Id, Type)>,
    body: Type,
}
