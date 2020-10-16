use std::collections::HashMap;
use std::rc::Rc;

pub type Id = String;
pub type Map = HashMap<Id, Expr>;

// Reduce ops
#[derive(Clone, Debug)]
pub enum Unop {
    LogNot,
    Not,
    And,
    Nand,
    Or,
    Nor,
    Xor,
    Xnor,
}

#[derive(Clone, Debug)]
pub enum Binop {
    LogOr,
    LogAnd,
    Add,
    Mul,
    Gt,
    Lt,
    Geq,
    Leq,
    Equal,
    NotEqual,
    IndexBit,
}

#[derive(Clone, Debug)]
pub enum Terop {
    Mux,
    Slice,
    IndexSlice,
}

#[derive(Clone, Debug)]
pub enum Radix {
    Dec,
    Bin,
    Hex,
}

#[derive(Clone, Debug)]
pub struct InstancePath {
    pub path: Vec<Id>,
}

#[derive(Default, Clone, Debug)]
pub struct ExprConcat {
    pub exprs: Vec<Expr>,
}

#[derive(Clone, Debug)]
pub enum Expr {
    Ref(Id),
    Int(i32),
    ULit(u32, Radix, String),
    Str(String),
    Signed(Rc<Expr>),
    IPath(InstancePath, Option<Rc<Expr>>),
    Unop(Unop, Rc<Expr>),
    Binop(Binop, Rc<Expr>, Rc<Expr>),
    Terop(Terop, Rc<Expr>, Rc<Expr>, Rc<Expr>),
    Concat(ExprConcat),
    Call(Id, Vec<Expr>),
}

#[derive(Clone, Debug)]
pub enum AttributeTy {
    Val(String),
    Stmt(Id, String),
}

#[derive(Clone, Debug, Default)]
pub struct Attribute {
    pub attrs: Vec<AttributeTy>,
}

#[derive(Clone, Debug)]
pub enum EventTy {
    Posedge,
    Negedge,
}

#[derive(Clone, Debug)]
pub struct Instance {
    pub id: Id,
    pub prim: Id,
    pub params: Map,
    pub ports: Map,
    pub attr: Attribute,
}

#[derive(Clone, Debug)]
pub enum AssignTy {
    Blocking,
    NonBlocking,
}

// T ~> Sequential type
#[derive(Clone, Debug)]
pub struct GenericCaseBranch<T> {
    pub cond: Expr,
    pub body: Vec<T>,
}

// T ~> Sequential type
#[derive(Clone, Debug)]
pub struct GenericCaseDefault<T> {
    pub body: Vec<T>,
}

// T ~> Sequential type
#[derive(Clone, Debug)]
pub struct GenericCase<T> {
    pub cond: Expr,
    pub branches: Vec<GenericCaseBranch<T>>,
    pub default: Option<GenericCaseDefault<T>>,
}

// T ~> Declaration type
#[derive(Clone, Debug)]
pub enum GenericPort<T> {
    Input(T),
    Output(T),
}

// T ~> Declaration type
// U ~> Sequential type
// V ~> Data Type
#[derive(Clone, Debug)]
pub struct GenericFunction<T, U, V> {
    pub name: Id,
    pub inputs: Vec<GenericPort<T>>,
    pub decls: Vec<T>,
    pub body: Vec<U>,
    pub ret: V,
}

// T ~> Declaration type
// U ~> Parallel type
#[derive(Clone, Debug)]
pub enum GenericStmt<T, U> {
    Decl(T),
    Parallel(U),
}

// T ~> Declaration type
// U ~> Parallel type
#[derive(Clone, Debug)]
pub struct GenericModule<T, U> {
    pub name: String,
    pub params: Vec<T>,
    pub ports: Vec<GenericPort<T>>,
    pub body: Vec<GenericStmt<T, U>>,
    pub attr: Attribute,
}
