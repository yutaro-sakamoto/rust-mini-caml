use crate::typedef::Type;
use std::sync::atomic::{AtomicU64, Ordering};

pub type Id = String;
pub enum GloblId {
    L(String),
}

fn id_of_type(t: Type) -> String {
    match t {
        Type::Unit => "u",
        Type::Bool => "b",
        Type::Int => "i",
        Type::Float => "d",
        Type::Func(_, _) => "f",
        Type::Tuple(_) => "t",
        Type::Array(_) => "a",
        Type::Var(_) => "v",
    }
    .to_string()
}

pub fn gen_tmp(t: Type) -> String {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    format!(
        "T{}{}",
        id_of_type(t),
        COUNTER.fetch_add(1, Ordering::SeqCst)
    )
}
