//! Prismal runtime (docs/spec/02-runtime-contract.md): runs, solvers, event handling,
//! interventions and observation of results.

pub mod run;
pub mod session;
pub mod solver;

pub use run::{run, Action, Category, Committed, Config, LogEntry, Run, RunDiag, RunStatus, Scheduled, SolverConfig};
pub use session::Session;
