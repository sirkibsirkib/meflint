use crate::{Expr, Spec};

type Got<'a, T> = Option<(T, &'a [u8])>;

fn expr(s: &[u8]) -> Got<Expr> {
    todo!()
}

fn spec(s: &[u8]) -> Got<Spec> {
    todo!()
}
