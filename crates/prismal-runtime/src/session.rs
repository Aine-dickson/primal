//! An interactive session over one run: proposals, commits, undo and redo (PK-10.5 to
//! PK-10.10). Undo is a recomputation with the intervention removed from the log (RC-11.2).

use crate::run::{run, Action, Config, Run, RunDiag, Scheduled};
use prismal_ir::Expr;
use prismal_kernel::{CModel, Value};

pub struct Session {
    model: CModel,
    base: Config,
    log: Vec<Scheduled>,
    redo: Vec<Scheduled>,
    pub current: Run,
}

impl Session {
    pub fn new(model: CModel, base: Config) -> Session {
        let current = run(&model, base.clone());
        Session { model, base, log: vec![], redo: vec![], current }
    }

    fn with_log(&self, log: Vec<Scheduled>) -> Run {
        let mut cfg = self.base.clone();
        cfg.log = log;
        run(&self.model, cfg)
    }

    /// Validates an action without committing it (SEM-08: proposed state is not committed).
    /// Returns the previewed run, or the rejection.
    pub fn propose(&self, t: f64, action: Action) -> Result<Run, RunDiag> {
        let mut log = self.log.clone();
        log.push(Scheduled { t, action });
        let r = self.with_log(log);
        match r.rejected.len() > self.current.rejected.len() {
            true => Err(r.rejected.last().unwrap().clone()),
            false => Ok(r),
        }
    }

    /// Commits an action. A rejected action is not logged (RC-10.5, RC-11.5).
    pub fn commit(&mut self, t: f64, action: Action) -> Result<(), RunDiag> {
        let r = self.propose(t, action.clone())?;
        self.log.push(Scheduled { t, action });
        self.redo.clear();
        self.current = r;
        Ok(())
    }

    pub fn undo(&mut self) {
        if let Some(s) = self.log.pop() {
            self.redo.push(s);
            self.current = self.with_log(self.log.clone());
        }
    }

    pub fn redo(&mut self) {
        if let Some(s) = self.redo.pop() {
            self.log.push(s);
            self.current = self.with_log(self.log.clone());
        }
    }

    pub fn log_len(&self) -> usize {
        self.log.len()
    }

    /// The value of an expression on the current final state.
    pub fn value(&self, e: &Expr) -> Value {
        let t = self.current.committed.last().unwrap().t;
        self.current.at(t, e)
    }
}
