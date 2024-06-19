use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub grammar);
pub mod ast;
fn main() {
    grammar::ExprParser::new().parse("\\ x .  x").unwrap();
    grammar::ExprParser::new().parse("\\ x . \\ y . x").unwrap();
    grammar::ExprParser::new().parse("\\ x . \\ y . y").unwrap();
    println!("Hello world!") 
}