use std::collections::BTreeMap;
use std::collections::{HashMap, HashSet};

mod parse;

#[derive(Hash, Debug, Clone, Eq, PartialEq)]
struct TypeId(String);

struct TypeDef {
    fieldid_to_typeid: Option<HashMap<TypeId, TypeId>>,
    emit: bool,
    seal: bool,
}

#[derive(Hash, Eq, Clone, Debug, PartialEq)]
enum AtomicTerm {
    Int(i64),
    Str(String),
    Bool(bool),
    Unit,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Term {
    alias: TypeId,
    values: TermValue,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
enum TermValue {
    AtomicTerm(AtomicTerm),
    CompositeTerm { type_id: TypeId, fields: BTreeMap<TypeId, Term> },
}

#[derive(Debug, Clone, Default)]
struct Kb {
    reals: HashSet<Term>,
}

enum Aggregator {
    Any,
    All,
    Num,
}

enum Expr {
    AtomicTerm(AtomicTerm),
    Aggregate { aggregator: Aggregator, expr: Box<Expr> },
    Role { alias: TypeId, expr: Box<Expr> },
    Take { alias: TypeId, expr: Box<Expr> },
    When { condition: Box<Expr>, result: Box<Expr> },
}

struct Spec {
    type_defs: HashMap<TypeId, TypeDef>,
    aver_exprs: HashSet<Expr>,
    sift_exprs: HashSet<Expr>,
}

enum EvalError {
    UnexpectedType { got: TypeId, expected: TypeId },
}

//////////////////////

fn retained<T, F: FnMut(&T) -> bool>(f: F, mut s: HashSet<T>) -> HashSet<T> {
    s.retain(f);
    s
}

impl AtomicTerm {
    fn type_id(&self) -> TypeId {
        todo!()
    }
}

impl Term {
    fn singleton(self) -> HashSet<Self> {
        let mut set = HashSet::default();
        set.insert(self);
        set
    }
    fn atomic_bool(b: bool) -> Self {
        Term::AtomicTerm(AtomicTerm::Bool(b))
    }
    fn atomic_bool_eval(&self) -> Result<bool, EvalError> {
        if let Term::AtomicTerm(AtomicTerm::Bool(b)) = self {
            return Ok(*b);
        }
        Err(EvalError::UnexpectedType { got: self.type_id(), expected: TypeId("str".into()) })
    }
    fn type_id(&self) -> TypeId {
        match self {
            Term::AtomicTerm(atomic_term) => atomic_term.type_id(),
            Term::CompositeTerm { type_id, .. } => type_id.clone(),
        }
    }
    fn is_atomic_true(&self) -> bool {
        self.atomic_bool_eval().unwrap_or(false)
    }
    fn atomic_int_eval(&self) -> Result<i64, EvalError> {
        if let Term::AtomicTerm(AtomicTerm::Int(i)) = self {
            return Ok(*i);
        }
        Err(EvalError::UnexpectedType { got: self.type_id(), expected: TypeId("int".into()) })
    }
}

impl Kb {
    fn eval(&self, expr: &Expr) -> Result<HashSet<Term>, EvalError> {
        Ok(match expr {
            Expr::AtomicTerm(atomic_term) => Term::AtomicTerm(atomic_term.clone()).singleton(),
            Expr::Aggregate { aggregator: Aggregator::Any, expr } => Term::AtomicTerm(
                AtomicTerm::Bool(self.eval(expr)?.iter().any(Term::is_atomic_true)),
            )
            .singleton(),
            Expr::Aggregate { aggregator: Aggregator::All, expr } => Term::AtomicTerm(
                AtomicTerm::Bool(self.eval(expr)?.iter().all(Term::is_atomic_true)),
            )
            .singleton(),
            Expr::Aggregate { aggregator: Aggregator::Num, expr } => {
                Term::AtomicTerm(AtomicTerm::Int(self.eval(expr)?.len() as i64)).singleton()
            }
            Expr::Role { alias, expr } => todo!(),
            Expr::Take { alias, expr } => todo!(),
            Expr::When { condition, result } => todo!(),
        })
    }
}

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
