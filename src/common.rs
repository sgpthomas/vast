use crate::pretty::PrettyPrinter;
use pretty::RcDoc;
use std::fmt;
use std::rc::Rc;

pub type Id = String;

pub type Width = u64;

impl PrettyPrinter for Width {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            0 => panic!("Error: width must be greater than zero"),
            1 => RcDoc::nil(),
            n => RcDoc::text("[")
                .append(RcDoc::as_string(n - 1))
                .append(RcDoc::text(":"))
                .append(RcDoc::text("0"))
                .append(RcDoc::text("]")),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Unop {
    LogicalNegation,
    BitwiseNegation,
    BitwiseAnd,
    BitwiseNand,
    BitwiseOr,
    BitwiseNor,
    BitwiseXor,
    BitwiseXnor,
}

impl PrettyPrinter for Unop {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Unop::LogicalNegation => RcDoc::text("!"),
            Unop::BitwiseNegation => RcDoc::text("~"),
            Unop::BitwiseAnd => RcDoc::text("&"),
            Unop::BitwiseNand => RcDoc::text("~&"),
            Unop::BitwiseOr => RcDoc::text("|"),
            Unop::BitwiseNor => RcDoc::text("~|"),
            Unop::BitwiseXor => RcDoc::text("^"),
            Unop::BitwiseXnor => RcDoc::text("~^"),
        }
    }
}

impl fmt::Display for Unop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_pretty())
    }
}

#[derive(Clone, Debug)]
pub enum Expr {
    Ref(Id),
    Unop(Unop, Rc<Expr>),
}

impl PrettyPrinter for Expr {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Expr::Ref(name) => RcDoc::as_string(name),
            Expr::Unop(op, name) => op.to_doc().append(name.to_doc()),
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_pretty())
    }
}

#[derive(Clone, Debug)]
pub enum GenericStmt<T, U> {
    Decl(T),
    Par(U),
}

#[derive(Clone, Debug)]
pub struct GenericModule<T, U> {
    pub stmt: Vec<GenericStmt<T, U>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn logical_negation() {
        assert_eq!("!".to_string(), Unop::LogicalNegation.to_string());
    }
}