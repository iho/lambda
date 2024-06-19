#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Var(String),
    Abs(String, Box<Expr>),
    App(Box<Expr>, Box<Expr>),
    Alias(String, Box<Expr>),
}
