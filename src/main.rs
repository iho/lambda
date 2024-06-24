use crate::ast::Expr;
use std::{collections::hash_set::HashSet, collections::HashMap};

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub grammar);
pub mod ast;

fn main() {
    let mut env: HashSet<String> = HashSet::new();
    let mut contex: HashMap<String, ast::Expr> = HashMap::new();
    let mut variable_counter = 1;

    let k = grammar::ExprParser::new()
        .parse("let k = \\ x . \\y . x")
        .unwrap();
    let s = grammar::ExprParser::new()
        .parse("let s = \\x.\\y.\\z.((x_z)_(y_z))")
        .unwrap();
    let skk = grammar::ExprParser::new().parse("((s_k)_k)").unwrap();

    eval(k, &mut env, &mut contex, &mut variable_counter);
    eval(s, &mut env, &mut contex, &mut variable_counter);
    let res = eval(skk.clone(), &mut env, &mut contex, &mut variable_counter);
    println!("{:?}", env);
    println!("{:?}", res);
    let res = normalize(res.clone(), &mut env, &mut contex, &mut variable_counter);
    // println!("{:?}", env);
    println!("{:?}", res);
}
fn eval(
    expr: Expr,
    env: &mut HashSet<String>,
    context: &mut HashMap<String, Expr>,
    mut variable_counter: &mut i32,
) -> Expr {
    match expr {
        Expr::Var(s) => {
            env.insert(s.clone());
            if let Some(e) = context.get(&s) {
                return eval(e.clone(), env, context, variable_counter);
            }

            Expr::Var(s)
        }
        Expr::Alias(name, body) => {
            context.insert(name.clone(), *body.clone());
            env.insert(name.clone());
            Expr::Var(name)
        }
        Expr::Lam(e1, e2) => {
            for (k, v) in context.iter() {
                if *v == *e2 {
                    return eval(v.clone(), env, context, variable_counter);
                }
            }
            let new_var = format!("var_{}", variable_counter);
            *variable_counter += 1;
            context.insert(new_var.clone(), Expr::Var(e1.clone()));
            env.insert(new_var.clone());
            let res = eval(*e2.clone(), env, context, variable_counter);
            Expr::Lam(new_var, Box::new(res))
        }
        Expr::App(s, e) => {
            let res_s = eval(*s.clone(), env, context, variable_counter);
            let res_e = eval(*e.clone(), env, context, variable_counter);

            Expr::App(Box::new(res_s), Box::new(res_e))
        }
    }
}

fn substitute(
    expr: Expr,
    var: String,
    value: Expr,
    env: &mut HashSet<String>,
    context: &mut HashMap<String, Expr>,
    mut variable_counter: &mut i32,
) -> Expr {
    match expr {
        Expr::Var(s) => {
            env.insert(s.clone());
            if s == var {
                return value;
            }
            Expr::Var(s)
        }
        Expr::Lam(e1, e2) => {
            if e1 == var {
                println!("Variable {} is bound", e1);
                // variable is bound, do not substitute
                Expr::Lam(
                    e1,
                    Box::new(substitute(*e2, var, value, env, context, variable_counter)),
                )
            } else if env.contains(&e1) {
                println!("Variable {} is already in the environment", e1);
                let new_var = format!("var_{}", variable_counter);
                *variable_counter += 1;
                context.insert(new_var.clone(), Expr::Var(e1.clone()));
                env.insert(new_var.clone());
                let t1_prime = substitute(
                    ast::Expr::Var(new_var.clone()),
                    e1,
                    *e2.clone(),
                    env,
                    context,
                    variable_counter,
                );
                Expr::Lam(
                    new_var,
                    Box::new(substitute(
                        *e2,
                        var,
                        t1_prime,
                        env,
                        context,
                        variable_counter,
                    )),
                )
            } else {
                Expr::Lam(
                    e1,
                    Box::new(substitute(*e2, var, value, env, context, variable_counter)),
                )
            }
        }
        Expr::App(e1, e2) => Expr::App(
            Box::new(substitute(
                *e1,
                var.clone(),
                value.clone(),
                env,
                context,
                variable_counter,
            )),
            Box::new(substitute(*e2, var, value, env, context, variable_counter)),
        ),
        Expr::Alias(name, body) => Expr::Alias(
            name,
            Box::new(substitute(
                *body,
                var,
                value,
                env,
                context,
                variable_counter,
            )),
        ),
    }
}

fn beta_reduction(
    expr: Expr,
    env: &mut HashSet<String>,
    context: &mut HashMap<String, Expr>,
    mut variable_counter: &mut i32,
) -> Expr {
    match expr {
        Expr::App(e1, e2) => {
            match *e1.clone() {
                Expr::Lam(d1, d2) => {
                    return substitute(*d2, d1, *e2, env, context, variable_counter);
                }
                _ => {}
            }
            let res1 = beta_reduction(*e1, env, context, variable_counter);
            let res2 = beta_reduction(*e2, env, context, variable_counter);
            Expr::App(Box::new(res1), Box::new(res2))
        }
        Expr::Lam(var, body) => {
            env.remove(&var);
            Expr::Lam(
                var,
                Box::new(beta_reduction(*body, env, context, variable_counter)),
            )
        }
        Expr::Var(s) => {
            env.insert(s.clone());
            Expr::Var(s)
        }
        Expr::Alias(name, body) => Expr::Alias(
            name,
            Box::new(beta_reduction(*body, env, context, variable_counter)),
        ),
    }
}
fn normalize(
    expr: Expr,
    env: &mut HashSet<String>,
    context: &mut HashMap<String, Expr>,
    mut variable_counter: &mut i32,
) -> Expr {
    let mut res = expr.clone();
    loop {
        let new_res = beta_reduction(res.clone(), env, context, variable_counter);
        if new_res == res {
            return res;
        }
        res = new_res;
    }
}
