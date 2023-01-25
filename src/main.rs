use std::collections::{HashMap, HashSet};

mod parse;

#[derive(Debug, Clone, Eq, PartialEq)]
struct TypeId(String);

struct TypeDef {
    fieldid_to_typeid: Option<HashMap<TypeId, TypeId>>,
    emit: bool,
    seal: bool,
}

#[derive(Debug, Clone)]
struct Term {
    ident: TypeId,
    fields: HashMap<TypeId, Term>,
}

#[derive(Debug, Clone, Default)]
struct Kb {
    reals: HashSet<Term>,
}

enum Aggregator {
    Any,
    All,
    Sum,
    Num,
}

enum Expr {
    Aggregate(Aggregator, Box<Expr>),
    Role { alias: TypeId, expr: Box<Expr> },
    Take(TypeId, Box<Expr>),
    When { condition: Box<Expr>, result: Box<Expr> },
}

struct Spec {
    type_defs: HashMap<TypeId, TypeDef>,
    aver_exprs: HashSet<Expr>,
    sift_exprs: HashSet<Expr>,
}

//////////////////////

impl Spec {
    fn infer(&self) -> Kb {
        let mut kb = Kb::default();
        'main: loop {
            for aver_expr in &self.aver_exprs {}
            return kb;
        }
    }
}

fn main() {
    println!("Hello, world!");
}
