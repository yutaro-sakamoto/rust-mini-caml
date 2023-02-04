use crate::typedef::Type;

fn deref_type(t: &mut Type) -> Type {
    match t {
        Type::Func(args, ret) => Type::Func(
            args.iter_mut().map(|x| deref_type(x)).collect(),
            Box::new(deref_type(&mut *ret)),
        ),
        Type::Tuple(elems) => Type::Tuple(elems.iter_mut().map(|x| deref_type(x)).collect()),
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
