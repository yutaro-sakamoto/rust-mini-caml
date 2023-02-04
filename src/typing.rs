use crate::ast::Exp;
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
        e => e.clone(),
    }
}
