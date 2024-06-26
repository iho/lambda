use crate::ast::Expr;
use std::{collections::HashMap, collections::HashSet};

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub grammar);
pub mod ast;

fn main() {
    let mut env: HashSet<String> = HashSet::new();
    let mut context: HashMap<String, Expr> = HashMap::new();
    let mut variable_counter = 1;

    let k = grammar::ExprParser::new()
        .parse("let k = \\x. \\y. x")
        .unwrap();
    let s = grammar::ExprParser::new()
        .parse("let s = \\x. \\y. \\z. ((x_z)_(y_z))")
        .unwrap();
    let skk = grammar::ExprParser::new().parse("((s_k)_k)").unwrap();
    // let foo = grammar::ExprParser::new().parse("\\x.x").unwrap();
    let two = grammar::ExprParser::new()
        .parse("let two = \\f. \\x. (f_(f_x))")
        .unwrap();
    let three = grammar::ExprParser::new()
        .parse("let three = \\f. \\x. (f_(f_(f_x)))")
        .unwrap();
    let add = grammar::ExprParser::new()
        .parse("let add = \\m.\\n.\\f.\\x. ((m_f)_(n_f))")
        .unwrap();
    let mul = grammar::ExprParser::new()
        .parse("let mul = \\m. \\n. \\f. (m_(n_f))")
        .unwrap();
    let action = grammar::ExprParser::new()
        .parse("((add_two)_three)").unwrap();
    let res = eval(two, &mut env, &mut context, &mut variable_counter);
    let res = eval(three, &mut env, &mut context, &mut variable_counter);
    let res = eval(add, &mut env, &mut context, &mut variable_counter);
    let res = eval(mul, &mut env, &mut context, &mut variable_counter);
    let res = eval(action, &mut env, &mut context, &mut variable_counter);
    eval(k, &mut env, &mut context, &mut variable_counter);
    eval(s, &mut env, &mut context, &mut variable_counter);
    // let res = eval(skk.clone(), &mut env, &mut context, &mut variable_counter);
    // println!("skk {:?}", res);
    // let res = normalize(res.clone(), &mut env, &mut context, &mut variable_counter);
    println!("{:?}", res);
    println!("{:?}", context);
    let res = normalize(res.clone(), &mut env, &mut context, &mut variable_counter);
    println!("add {:?}", res);
}

fn eval(
    expr: Expr,
    env: &mut HashSet<String>,
    context: &mut HashMap<String, Expr>,
    variable_counter: &mut i32,
) -> Expr {
    match expr {
        Expr::Var(s) => {
            if let Some(e) = context.get(&s) {
                eval(e.clone(), env, context, variable_counter)
            } else {
                Expr::Var(s)
            }
        }
        Expr::Alias(name, body) => {
            context.insert(name.clone(), *body.clone());
            Expr::Var(name)
        }
        Expr::Lam(e1, e2) => {
            let res = eval(*e2.clone(), env, context, variable_counter);
            Expr::Lam(e1, Box::new(res))
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
    variable_counter: &mut i32,
) -> Expr {
    match expr {
        Expr::Var(s) => {
            if s == var {
                value
            } else {
                Expr::Var(s)
            }
        }
        Expr::Lam(e1, e2) => {
            if e1 == var {
                Expr::Lam(e1, e2)
            } else {
                let new_var = format!("var_{}", variable_counter);
                *variable_counter += 1;
                let body = substitute(
                    *e2,
                    e1.clone(),
                    Expr::Var(new_var.clone()),
                    env,
                    context,
                    variable_counter,
                );
                Expr::Lam(
                    new_var.clone(),
                    Box::new(substitute(body, var, value, env, context, variable_counter)),
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
    variable_counter: &mut i32,
) -> Expr {
    match expr {
        Expr::App(e1, e2) => match *e1 {
            Expr::Lam(d1, d2) => substitute(*d2, d1, *e2, env, context, variable_counter),
            _ => Expr::App(
                Box::new(beta_reduction(*e1, env, context, variable_counter)),
                Box::new(beta_reduction(*e2, env, context, variable_counter)),
            ),
        },
        Expr::Lam(var, body) => Expr::Lam(
            var,
            Box::new(beta_reduction(*body, env, context, variable_counter)),
        ),
        Expr::Var(s) => Expr::Var(s),
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
    variable_counter: &mut i32,
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
