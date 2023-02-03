#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Type {
    Unit,
    Bool,
    Int,
    Float,
    Func(Vec<Type>, Box<Type>),
    Tuple(Vec<Type>),
    Array(Box<Type>),
    Var(Option<Box<Type>>),
}

#[allow(dead_code)]
fn gentyp() -> Type {
    Type::Var(None)
}
