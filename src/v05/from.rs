use crate::v05::ast::*;

impl From<Instance> for Parallel {
    fn from(inst: Instance) -> Self {
        Parallel::Inst(inst)
    }
}

impl From<Decl> for Stmt {
    fn from(decl: Decl) -> Self {
        Stmt::Decl(decl)
    }
}

impl From<SequentialIfElse> for Sequential {
    fn from(seq: SequentialIfElse) -> Self {
        Sequential::IfElse(seq)
    }
}

impl From<ParallelAlways> for Parallel {
    fn from(always: ParallelAlways) -> Self {
        Parallel::Always(always)
    }
}

impl From<ParallelAlways> for Stmt {
    fn from(always: ParallelAlways) -> Self {
        Stmt::from(Parallel::from(always))
    }
}

impl From<Parallel> for Stmt {
    fn from(par: Parallel) -> Self {
        Stmt::Parallel(par)
    }
}

impl From<Instance> for Stmt {
    fn from(inst: Instance) -> Self {
        Stmt::from(Parallel::from(inst))
    }
}
