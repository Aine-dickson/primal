//! Expressions (04-ir section 6). Each keeps its symbolic structure (MK-10.6).

use crate::units::Unit;
use crate::{Id, Type};
use serde::{Deserialize, Serialize};
use std::ops;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    And,
    Or,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Builtin {
    T,
    T0,
    Elapsed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Func {
    Sin,
    Cos,
    Tan,
    Sqrt,
    Exp,
    Log,
    Abs,
    Min,
    Max,
    Atan2,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Lambda {
    pub params: Vec<Type>,
    pub body: Box<Expr>,
}

/// An expression. Serialized with one distinguishing field per form.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Expr {
    Num {
        num: f64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        unit: Option<Unit>,
    },
    Bool {
        bool: bool,
    },
    Case {
        case: String,
    },
    Ref {
        r#ref: Id,
    },
    Param {
        param: usize,
    },
    Builtin {
        builtin: Builtin,
    },
    Origin {
        origin: Id,
    },
    Der {
        der: Id,
    },
    Neg {
        neg: Box<Expr>,
    },
    Not {
        not: Box<Expr>,
    },
    Norm {
        norm: Box<Expr>,
    },
    Bin {
        bin: BinOp,
        l: Box<Expr>,
        r: Box<Expr>,
    },
    Call {
        call: Func,
        args: Vec<Expr>,
    },
    Apply {
        apply: Box<Expr>,
        args: Vec<Expr>,
    },
    If {
        r#if: Box<Expr>,
        then: Box<Expr>,
        r#else: Box<Expr>,
    },
    Tuple {
        tuple: Vec<Expr>,
    },
    Comp {
        comp: Box<Expr>,
        axis: usize,
    },
    Lambda {
        lambda: Lambda,
    },
    Otherwise {
        otherwise: Box<Expr>,
        default: Box<Expr>,
    },
}

impl Expr {
    pub fn bin(op: BinOp, l: Expr, r: Expr) -> Expr {
        Expr::Bin { bin: op, l: Box::new(l), r: Box::new(r) }
    }

    /// True for a bare numeric literal other than zero (MK-3.8).
    pub fn is_bare_nonzero(&self) -> bool {
        matches!(self, Expr::Num { num, unit: None } if *num != 0.0)
    }

    /// True for the bare literal `0`.
    pub fn is_bare_zero(&self) -> bool {
        matches!(self, Expr::Num { num, unit: None } if *num == 0.0)
    }

    /// Visits every sub-expression, including this one, in pre-order.
    pub fn walk<'a>(&'a self, f: &mut dyn FnMut(&'a Expr)) {
        f(self);
        match self {
            Expr::Neg { neg: e } | Expr::Not { not: e } | Expr::Norm { norm: e } => e.walk(f),
            Expr::Comp { comp: e, .. } => e.walk(f),
            Expr::Bin { l, r, .. } => {
                l.walk(f);
                r.walk(f);
            }
            Expr::Call { args, .. } | Expr::Tuple { tuple: args } => args.iter().for_each(|a| a.walk(f)),
            Expr::Apply { apply, args } => {
                apply.walk(f);
                args.iter().for_each(|a| a.walk(f));
            }
            Expr::If { r#if, then, r#else } => {
                r#if.walk(f);
                then.walk(f);
                r#else.walk(f);
            }
            Expr::Lambda { lambda } => lambda.body.walk(f),
            Expr::Otherwise { otherwise, default } => {
                otherwise.walk(f);
                default.walk(f);
            }
            _ => {}
        }
    }

    /// Identities of the bindings this expression reads directly (not through derived bindings).
    pub fn refs(&self) -> Vec<Id> {
        let mut out = Vec::new();
        self.walk(&mut |e| {
            if let Expr::Ref { r#ref } = e {
                if !out.contains(r#ref) {
                    out.push(r#ref.clone());
                }
            }
        });
        out
    }
}

macro_rules! binop {
    ($tr:ident, $m:ident, $op:expr) => {
        impl ops::$tr for Expr {
            type Output = Expr;
            fn $m(self, r: Expr) -> Expr {
                Expr::bin($op, self, r)
            }
        }
    };
}
binop!(Add, add, BinOp::Add);
binop!(Sub, sub, BinOp::Sub);
binop!(Mul, mul, BinOp::Mul);
binop!(Div, div, BinOp::Div);

impl ops::Neg for Expr {
    type Output = Expr;
    fn neg(self) -> Expr {
        Expr::Neg { neg: Box::new(self) }
    }
}
