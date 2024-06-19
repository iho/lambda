use crate::ast::Expr;
use std::{collections::HashMap, env, rc::Rc};

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub grammar);
pub mod ast;
fn main() {
    let mut env: HashMap<String, Expr> = HashMap::new();
    let mut contex: HashMap<String, Expr> = HashMap::new();
    let mut variable_counter = 1;


    let k = grammar::ExprParser::new()
        .parse("let k = \\ x . \\y . x")
        .unwrap();
    let s = grammar::ExprParser::new()
        .parse("let s = \\x.\\y.\\z.((x_z)_(y_z))")
        .unwrap();
    let skk = grammar::ExprParser::new().parse("((s_k)_k)").unwrap();

    eval(k, &mut env, &mut contex,  &mut variable_counter);
    eval(s, &mut env ,&mut contex, &mut variable_counter);
    println!("{:?}", env);
    let res = eval(skk.clone(), &mut env,&mut contex, &mut variable_counter);
    let res = eval(res.clone(), &mut env,&mut contex, &mut variable_counter);

    println!("{:?}", env);
    println!("{:?}", res);
}

fn eval(expr:Expr, env: &mut HashMap<String, Expr>, contex: &mut HashMap<String, Expr>,mut variable_counter: &mut i32) -> Expr {
    match expr {
        Expr::Var(s) => {
            if let Some(e) = env.get(&s) {
               return  eval(e.clone(), env, contex, variable_counter);
            } 
            
            Expr::Var(s)
        }
        Expr::Alias(name, body) => {
            env.insert(name.clone(), *body.clone());
            Expr::Var(name)
        }
        Expr::Abs(e1, e2) => {
            for (k, v) in contex.iter() {
                if *v == *e2 {
                    return eval(v.clone(), env, contex, variable_counter) ;
                }
            }
            let new_var = format!("var_{}", variable_counter);
            *variable_counter += 1;
            contex.insert(new_var.clone(), Expr::Var(e1.clone()));
            let res =  eval(*e2.clone(), env, contex, variable_counter);
            Expr::Abs(new_var, Box::new(res))
        }
        Expr::App(s, e) => {
            let res_s =  eval(*s.clone(), env, contex, variable_counter);
            let res_e =  eval(*e.clone(), env, contex, variable_counter);

            Expr::App(Box::new(res_s), Box::new(res_e))
        }
    } 
}