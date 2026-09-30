//! Prismal model kernel (docs/spec/01-model-kernel.md): static checking, dependency
//! analysis, compilation and evaluation of expressions.

pub mod check;
pub mod eval;
pub mod model;

pub use check::{check_intervention, check_model, compile_expr, show, Diagnostic};
pub use eval::{distance, Arr, CExpr, Ctx, Status, StatusKind, Value};
pub use model::*;
