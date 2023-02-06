use crate::ast::{Exp, FunDef};
use crate::id::Id;
use crate::typedef::Type;

fn deref_type(t: &mut Type) -> Type {
    match t {
        Type::Func(args, ret) => Type::Func(
            args.iter_mut().map(|x| deref_type(x)).collect(),
            Box::new(deref_type(&mut *ret)),
        ),
        Type::Tuple(elems) => Type::Tuple(elems.iter_mut().map(|x| deref_type(x)).collect()),
        Type::Array(e) => Type::Array(Box::new(deref_type(e))),
        Type::Var(ref mut v) => match v {
            None => {
                eprintln!("uninstantiated type variable detected; assuming int@.");
                *v = Some(Box::new(Type::Int));
                Type::Int
            }
            Some(e) => {
                let ret = deref_type(e);
                *v = Some(Box::new(ret.clone()));
                ret
            }
        },
        other => other.clone(),
    }
}

fn deref_id_type(v: &mut (Id, Type)) -> (Id, Type) {
    (v.0.clone(), deref_type(&mut v.1))
}

fn deref_term(exp: &mut Exp) -> Exp {
    match exp {
        Exp::Not(e) => Exp::Not(Box::new(deref_term(&mut *e))),
        Exp::Neg(e) => Exp::Neg(Box::new(deref_term(&mut *e))),
        Exp::Add(e1, e2) => Exp::Add(
            Box::new(deref_term(&mut *e1)),
            Box::new(deref_term(&mut *e2)),
        ),
        Exp::Sub(e1, e2) => Exp::Sub(
            Box::new(deref_term(&mut *e1)),
            Box::new(deref_term(&mut *e2)),
        ),
        Exp::Eq(e1, e2) => Exp::Eq(
            Box::new(deref_term(&mut *e1)),
            Box::new(deref_term(&mut *e2)),
        ),
        Exp::LE(e1, e2) => Exp::LE(
            Box::new(deref_term(&mut *e1)),
            Box::new(deref_term(&mut *e2)),
        ),
        Exp::FNeg(e) => Exp::FNeg(Box::new(deref_term(&mut *e))),
        Exp::FAdd(e1, e2) => Exp::FAdd(
            Box::new(deref_term(&mut *e1)),
            Box::new(deref_term(&mut *e2)),
        ),
        Exp::FSub(e1, e2) => Exp::FSub(
            Box::new(deref_term(&mut *e1)),
            Box::new(deref_term(&mut *e2)),
        ),
        Exp::FDiv(e1, e2) => Exp::FDiv(
            Box::new(deref_term(&mut *e1)),
            Box::new(deref_term(&mut *e2)),
        ),
        Exp::FMul(e1, e2) => Exp::FMul(
            Box::new(deref_term(&mut *e1)),
            Box::new(deref_term(&mut *e2)),
        ),
        Exp::If(e1, e2, e3) => Exp::If(
            Box::new(deref_term(&mut *e1)),
            Box::new(deref_term(&mut *e2)),
            Box::new(deref_term(&mut *e3)),
        ),
        Exp::Let(xt, e1, e2) => Exp::Let(
            deref_id_type(xt),
            Box::new(deref_term(&mut *e1)),
            Box::new(deref_term(&mut *e2)),
        ),
        Exp::LetRec(
            FunDef {
                name: xt,
                args: yts,
                body: e1,
            },
            e2,
        ) => Exp::LetRec(
            FunDef {
                name: deref_id_type(xt),
                args: yts.iter_mut().map(|x| deref_id_type(x)).collect(),
                body: Box::new(deref_term((e1))),
            },
            Box::new(deref_term(&mut *e2)),
        ),
        Exp::App(e, es) => Exp::App(
            Box::new(deref_term(e)),
            es.iter_mut().map(|x| Box::new(deref_term(x))).collect(),
        ),
        Exp::Tuple(es) => Exp::Tuple(es.iter_mut().map(|x| Box::new(deref_term(x))).collect()),
        Exp::LetTuple(xts, e1, e2) => Exp::LetTuple(
            xts.iter_mut().map(|x| deref_id_type(x)).collect(),
            Box::new(deref_term(&mut *e1)),
            Box::new(deref_term(&mut *e2)),
        ),
        Exp::Array(e1, e2) => Exp::Array(
            Box::new(deref_term(&mut *e1)),
            Box::new(deref_term(&mut *e2)),
        ),
        Exp::Get(e1, e2) => Exp::Get(
            Box::new(deref_term(&mut *e1)),
            Box::new(deref_term(&mut *e2)),
        ),
        Exp::Put(e1, e2, e3) => Exp::Put(
            Box::new(deref_term(&mut *e1)),
            Box::new(deref_term(&mut *e2)),
            Box::new(deref_term(&mut *e3)),
        ),
        /* Unit, Bool, Int, Float and Var*/
        e => e.clone(),
    }
}

fn occur(r1: Option<Box<Type>>, t: &Type) -> bool {
    match t {
        Type::Func(t2s, t2) => t2s.iter().any(|x| occur(r1.clone(), x)) || occur(r1, t2),
        Type::Tuple(t2s) => t2s.iter().any(|x| occur(r1.clone(), x)),
        Type::Array(t2) => occur(r1, t2),
        Type::Var(None) => r1.is_none(),
        Type::Var(Some(t2)) => match &r1 {
            Some(rr1) => (**rr1 == **t2) || occur(r1, t2),
            _ => false,
        },
        _ => false,
    }
}

type Unify = (Type, Type);

fn option_box_type_equals(t1: &Option<Box<Type>>, t2: &Option<Box<Type>>) -> bool {
    match (t1, t2) {
        (Some(r1), Some(r2)) => **r1 == **r2,
        (None, None) => true,
        (_, _) => false,
    }
}

fn unify<'a, 'b>(t1: &'a mut Type, t2: &'b mut Type) -> Result<(), Unify> {
    match (&mut *t1, &mut *t2) {
        (Type::Unit, Type::Unit)
        | (Type::Bool, Type::Bool)
        | (Type::Int, Type::Int)
        | (Type::Float, Type::Float) => Ok(()),
        (Type::Func(t1s, t1_), Type::Func(t2s, t2_)) => {
            if t1s.len() == t2s.len() {
                for (x, y) in t1s.iter_mut().zip(t2s.iter_mut()) {
                    match unify(x, y) {
                        Err(s) => return Err(s),
                        _ => {}
                    }
                }
                unify(t1_, t2_)
            } else {
                Err((t1.clone(), t2.clone()))
            }
        }
        (Type::Tuple(t1s), Type::Tuple(t2s)) => {
            if t1s.len() == t2s.len() {
                for (x, y) in t1s.iter_mut().zip(t2s.iter_mut()) {
                    match unify(x, y) {
                        Err(s) => return Err(s),
                        _ => {}
                    }
                }
                Ok(())
            } else {
                Err((t1.clone(), t2.clone()))
            }
        }
        (Type::Array(r1), Type::Array(r2)) => unify(r1, r2),
        (Type::Var(ref mut r1), Type::Var(ref mut r2)) if option_box_type_equals(r1, r2) => Ok(()),
        (Type::Var(ref mut r1), _) => match r1 {
            Some(rr1) => unify(&mut *rr1, t2),
            None => {
                if occur(None, t2) {
                    Err((t1.clone(), t2.clone()))
                } else {
                    *r1 = Some(Box::new(t2.clone()));
                    Ok(())
                }
            }
        },
        (_, Type::Var(ref mut r2)) => unify(t2, t1),
        (_, _) => Err((t1.clone(), t2.clone())),
    }
}
