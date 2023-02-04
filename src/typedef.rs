use crate::id::Id;
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

pub fn gen_type() -> Type {
    Type::Var(None)
}

pub fn add_type(id: Id) -> (Id, Type) {
    (id, gen_type())
}
