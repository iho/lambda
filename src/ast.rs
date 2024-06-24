#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Var(String),
    Lam(String, Box<Expr>),
    App(Box<Expr>, Box<Expr>),
    Alias(String, Box<Expr>),
}
