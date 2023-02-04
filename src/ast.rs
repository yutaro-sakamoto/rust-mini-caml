use crate::id::Id;
use crate::typedef::Type;
use std::fmt::{Debug, Error, Formatter};
use std::vec::Vec;

#[derive(PartialEq, Clone)]
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

impl Debug for Exp {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Exp::Unit => write!(f, "()"),
            Exp::Bool(b) => {
                if *b {
                    write!(f, "true")
                } else {
                    write!(f, "false")
                }
            }
            Exp::Int(i) => write!(f, "{}", i),
            Exp::Float(d) => write!(f, "{}", d),
            Exp::Not(e) => write!(f, "Not {:?}", e),
            Exp::Neg(e) => write!(f, "-{:?}", e),
            Exp::FNeg(e) => write!(f, "-.{:?}", e),
            Exp::Add(a, b) => write!(f, "{:?} + {:?}", a, b),
            Exp::Sub(a, b) => write!(f, "{:?} - {:?}", a, b),
            Exp::FAdd(a, b) => write!(f, "{:?} +. {:?}", a, b),
            Exp::FSub(a, b) => write!(f, "{:?} -. {:?}", a, b),
            Exp::FMul(a, b) => write!(f, "{:?} *. {:?}", a, b),
            Exp::FDiv(a, b) => write!(f, "{:?} /. {:?}", a, b),
            Exp::Eq(a, b) => write!(f, "{:?} = {:?}", a, b),
            Exp::LE(a, b) => write!(f, "{:?} <= {:?}", a, b),
            Exp::If(cond, e1, e2) => write!(f, "if {:?} then {:?} else {:?}", cond, e1, e2),
            Exp::Let((id, _), e1, e2) => write!(f, "let {} = {:?} in {:?}", id, e1, e2),
            Exp::LetTuple(v, e1, e2) => {
                write!(f, "let (").unwrap();
                let mut first = true;
                for e in v.iter() {
                    if !first {
                        write!(f, ", ").unwrap();
                    }
                    first = false;
                    write!(f, "{}", e.0).unwrap();
                }
                write!(f, ") = {:?} in {:?}", e1, e2)
            }
            Exp::LetRec(
                FunDef {
                    name: (id, _),
                    args: formal_args,
                    body: bd,
                },
                e,
            ) => {
                write!(f, "let rec {}", id).unwrap();
                for (arg_name, _) in formal_args.iter() {
                    write!(f, " {}", arg_name).unwrap();
                }
                write!(f, " = {:?} in {:?}", bd, e)
            }
            Exp::Var(s) => write!(f, "{}", s),
            Exp::App(e, v) => {
                write!(f, "{:?}", e).unwrap();
                for x in v.iter() {
                    write!(f, " {:?}", x).unwrap();
                }
                write!(f, "")
            }
            Exp::Tuple(v) => {
                let mut first = true;
                for e in v.iter() {
                    if !first {
                        write!(f, ", ").unwrap();
                    }
                    first = false;
                    write!(f, "{:?}", e).unwrap();
                }
                write!(f, "")
            }
            Exp::Array(e1, e2) => write!(f, "Array.create {:?} {:?}", e1, e2),
            Exp::Get(e1, e2) => write!(f, "{:?}.({:?})", e1, e2),
            Exp::Put(e1, e2, e3) => write!(f, "{:?}.({:?}) <- ({:?})", e1, e2, e3),
        }
    }
}
