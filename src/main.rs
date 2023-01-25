use maplit::{hashset, hashmap};
use std::collections::{HashMap, HashSet};

mod parse;
mod combine;

#[derive(Hash, Debug, Clone, Eq, PartialEq)]
struct TypeId(String);

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Data(Vec<u8>);

struct ProdDef {
    params: Option<Vec<TypeId>>, // determines order. invariant: no dups
    emit: bool,
}

struct ProdTypes(HashMap<TypeId, ProdDef>);

struct Spec {
    prod_types: ProdTypes,
    inference_exprs: InferenceExprs,
}

struct InferenceExprs {
    aver_exprs: HashSet<Expr>,
    sift_exprs: HashSet<Expr>,
}

#[derive(Hash, Eq, Clone, Debug, PartialEq)]
enum Constant {
    Int(i64),
    Bit(bool),
}

#[derive(Debug)]
struct Values {
    type_id: TypeId,
    datas: HashSet<Data>,
}

#[derive(Debug, Clone, Default)]
struct Kb {
    reals: HashMap<TypeId, HashSet<Data>>,
}

enum Aggregator {
    Any,
    All,
    Num,
}

enum Expr {
    Constant(Constant),
    Aggregate { aggregator: Aggregator, expr: Box<Expr> },
    Take { alias: TypeId, expr: Box<Expr> },
    When { condition: Box<Expr>, result: Box<Expr> },
    Product {
        type_id: TypeId,
        args: Vec<Expr>, // invariant: Type IDs matches params exactly
    }
}

enum EvalError {
    UnexpectedType { got: TypeId, expected: TypeId },
}

//////////////////////

impl TypeId {
    fn int() -> Self {
        Self::new("int")
    }
    fn bit() -> Self {
        Self::new("bit")
    }
    fn new(s: &str) -> Self {
        Self(s.into())
    }   
}

impl Constant {
    fn type_id(&self) -> TypeId {
        match self {
            Constant::Int(_) => TypeId::int(),
            Constant::Bit(_) => TypeId::bit(),
        }
    }
}

impl Expr {
    fn type_id(&self) -> TypeId {
        match self{
            Expr::Constant(constant) => constant.type_id(),
            Expr::Aggregate { aggregator: Aggregator::Num, .. } => TypeId::int(),
            Expr::Aggregate { .. } => TypeId::bit(),
            Expr::Take { expr, .. } => expr.type_id(),
            Expr::When { result, .. } => result.type_id(),
            Expr::Product { type_id, .. } => type_id.clone(),
        }
    }
}
impl ProdTypes {
    fn eval_const(&self, kb: &Kb, expr: &Expr, constant: &Constant) -> Values {
        match constant {
            Constant::Int(i) => Values {
                type_id: TypeId::int(),
                datas: hashset! { Data(i.to_ne_bytes().into_iter().collect()) },
            },
            Constant::Bit(b) => Values {
                type_id: TypeId::bit(),
                datas: hashset! { Data((*b as u8).to_ne_bytes().into_iter().collect()) },
            },
        }
    }
    fn product_instances(&self, args: &[Values]) -> HashSet<Data> {
        let mut c = combine::Combination::new(args);
        let mut x = HashSet::default();
        while let Some(y) = c.next() {
            x.insert(y);
        }
        x
    }
    fn eval(&self, kb: &Kb, expr: &Expr) -> Values {
        match expr {
            Expr::Constant(constant) => self.eval_const(kb, expr, constant),
            Expr::Aggregate { aggregator, expr } => todo!(),
            Expr::Take { alias, expr } => todo!(),
            Expr::When { condition, result } => todo!(),
            Expr::Product { type_id, args } => {
                // let args_type_ids: HashSet<TypeId> = args.iter().map(Expr::type_id).collect();
                // let params_type_ids: HashSet<TypeId> = self.0.get(type_id)
                //     .expect("idk that type").params.as_ref()
                //     .expect("unknown params for this type").iter().cloned().collect();
                // if args_type_ids != params_type_ids {
                //     panic!("params {:?} mismatch fields {:?}", params_type_ids, args_type_ids);
                // }
                let args: Vec<_> = args.iter().map(|arg| self.eval(kb, arg)).collect();
                Values {
                    type_id: type_id.clone(),
                    datas: self.product_instances(&args),
                }
            },
        }
    }
}

impl Spec {
    fn project<'a,'b>(&'a self, type_id: &'a TypeId, param_id: &'a TypeId, data: &'b [u8]) -> Option<&'b [u8]> {
        let mut bytes = 0;
        for p_id in self.prod_types.0.get(type_id)?.params.as_ref()? {
            if param_id == p_id {
                return Some(&data[bytes..]);
            }
            bytes += self.type_bytes(param_id)?;
        }
        None
    }
    fn type_bytes(&self, type_id: &TypeId) -> Option<usize> {
        Some(match &type_id.0 as &str {
            "int" => std::mem::size_of::<i64>(),
            "bit" => std::mem::size_of::<bool>(),
            _ => { 
                let mut bytes = 0;
                for field_id in self.prod_types.0.get(type_id)?.params.as_ref()? {
                    bytes += self.type_bytes(field_id)?;
                }
                bytes
            } 
        })
    }
}

fn main() {
    let kb = Kb::default();
    let expr = Expr::Product {
        type_id : TypeId::new("sale"),
        args: vec![ 
            Expr::Product {
                type_id: TypeId::new("seller"),
                args: vec![
                    Expr::Product {
                        type_id: TypeId::new("person"),
                        args: vec![
                            Expr::Constant(Constant::Bit(true)),
                        ],
                    }
                ],
            },
            Expr::Product {
                type_id: TypeId::new("buyer"),
                args: vec![
                    Expr::Product {
                        type_id: TypeId::new("person"),
                        args: vec![
                            Expr::Constant(Constant::Bit(false)),
                        ],
                    }
                ],
            },
        ],
    };
    let spec = Spec {
        prod_types: ProdTypes(hashmap!{
            TypeId::new("person") => ProdDef {
                emit: false,
                params: Some(vec![TypeId::bit()]),
            },
            TypeId::new("seller") => ProdDef {
                emit: false,
                params: Some(vec![TypeId::new("person")]),
            },
            TypeId::new("buyer") => ProdDef {
                emit: false,
                params: Some(vec![TypeId::new("person")]),
            },
            TypeId::new("sale") => ProdDef {
                emit: false,
                params: Some(vec![TypeId::new("seller"), TypeId::new("buyer")]),
            },
        }),
        inference_exprs: InferenceExprs {
            aver_exprs: Default::default(),
            sift_exprs: Default::default(),
        }
    };
    println!("{:?}", spec.prod_types.eval(&kb, &expr));
}
