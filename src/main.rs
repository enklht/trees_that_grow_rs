// 3.1 extensible ADT declarations
#[derive(Debug, Clone, PartialEq)]
enum Typ {
    Int,
    Fun(Box<Typ>, Box<Typ>),
}

trait Descriptor {
    type Lit;
    type Var;
    type Ann;
    type Abs;
    type App;
    type Exp;
}

enum Exp<X: Descriptor> {
    Lit(X::Lit, i32),
    Var(X::Var, String),
    Ann(X::Ann, Box<Exp<X>>, Typ),
    Abs(X::Abs, String, Box<Exp<X>>),
    App(X::App, Box<Exp<X>>, Box<Exp<X>>),
    Exp(X::Exp),
}

// Note: I think that instead of a void or empty type (as described in the paper) we should use the unit type,
// since products with the void type are isomorphic to void.
// If `Lit` in `ExpUD` were a product type of void and `i32`, it would never be constructible, because creating
// such a value would require an element of the void type, which is impossible.
struct UD;
impl Descriptor for UD {
    type Lit = ();
    type Var = ();
    type Ann = ();
    type Abs = ();
    type App = ();
    type Exp = ();
}
type ExpUD = Exp<UD>;

// 3.3 New field extensions
struct TC;
impl Descriptor for TC {
    type Lit = ();
    type Var = ();
    type Ann = ();
    type Abs = ();
    type App = Typ;
    type Exp = ();
}
type ExpTC = Exp<TC>;

// 3.4 New constructor extensions
struct PE;
impl Descriptor for PE {
    type Lit = ();
    type Var = ();
    type Ann = ();
    type Abs = ();
    type App = ();
    type Exp = i32;
}
type ExpPE = Exp<PE>;

// 3.5 Normal functions on extended data types
use std::collections::HashMap;

fn check(exp: &ExpTC, typ: Typ, env: &HashMap<String, Typ>) -> bool {
    match exp {
        Exp::Lit(_, _) => typ == Typ::Int,
        Exp::Var(_, name) => env.get(name).map(|t| *t == typ).unwrap_or(false),
        Exp::Ann(_, exp, ann_type) => typ == *ann_type && check(exp, typ, env),
        Exp::Abs(_, arg, body) => match typ {
            Typ::Int => false,
            Typ::Fun(src, dst) => {
                let mut env = env.clone();
                env.insert(arg.clone(), *src.clone());
                check(body, *dst, &env)
            }
        },
        Exp::App(arg_typ, func, arg) => {
            check(arg, arg_typ.clone(), env)
                && check(
                    func,
                    Typ::Fun(Box::new(arg_typ.clone()), Box::new(typ.clone())),
                    env,
                )
        }
        Exp::Exp(_) => false,
    }
}

#[cfg(test)]
mod type_check {
    use super::*;

    #[test]
    fn test_check() {
        let env = &HashMap::from([
            ("x".to_string(), Typ::Int),
            (
                "f".to_string(),
                Typ::Fun(
                    Box::new(Typ::Int),
                    Box::new(Typ::Fun(Box::new(Typ::Int), Box::new(Typ::Int))),
                ),
            ),
        ]);

        assert!(check(&Exp::Lit((), 5), Typ::Int, env));
    }
}

// 3.6 generic functions on extensible data types
fn print_type(typ: &Typ) -> String {
    match typ {
        Typ::Int => "int".to_string(),
        Typ::Fun(src, dst) => format!("({}) -> {}", print_type(src), print_type(dst)),
    }
}

// Caution: this might not work; I didn't focus on it because the next example presents a more general solution.
// fn print_exp<X: Descriptor>(exp: &Exp<X>, p: fn(&X::Exp) -> String) -> String {
//     match exp {
//         Exp::Lit(_, i) => format!("{i}"),
//         Exp::Var(_, name) => name.clone(),
//         Exp::Ann(_, exp, typ) => format!("({})::({})", print_exp(exp, p), print_type(&typ)),
//         Exp::Abs(_, arg, body) => format!("lambda {}. {}", arg, print_exp(body, p)),
//         Exp::App(_, func, arg) => format!("({})({})", print_exp(func, p), print_exp(arg, p)),
//         Exp::Exp(exp) => p(exp),
//     }
// }

// 3.7 Type classes for extensible data types
// In Rust, the "constraint kinds" extension in the paper can be realized with traits that take a generic parameter.
trait Printer<X: Descriptor> {
    fn p_lit(data_lit: &X::Lit) -> String;
    fn p_var(data_lit: &X::Var) -> String;
    fn p_ann(data_lit: &X::Ann) -> String;
    fn p_abs(data_lit: &X::Abs) -> String;
    fn p_app(data_lit: &X::App) -> String;
    fn p_exp(data_lit: &X::Exp) -> String;
}

// While the implementation of the `Printer` trait below is faithful to the example in the paper, it may be better to
// define it with methods that accept the related expression data alongside any extra descriptor data, for example:
// trait Printer<X: Descriptor> {
//     fn p_lit(data_lit: &X::Lit, &i32) -> String;
//     fn p_var(data_lit: &X::Var, &String) -> String;
//     fn p_ann(data_lit: &X::Ann, &Box<Expr<X>>, &Typ) -> String;
//     fn p_abs(data_lit: &X::Abs, &String, &Box<Expr<X>>) -> String;
//     fn p_app(data_lit: &X::App, &Box<Expr<X>>, &Box<Expr<X>>) -> String;
//     fn p_exp(data_lit: &X::Exp) -> String;
// }

// 3.7 Type classes for extensible data types
fn print_exp<X: Descriptor, P: Printer<X>>(exp: &Exp<X>) -> String {
    match exp {
        Exp::Lit(data, i) => format!("{i} is {}", P::p_lit(data)),
        Exp::Var(data, name) => format!("{name} is {}", P::p_var(data)),
        Exp::Ann(data, exp, typ) => format!(
            "({})::({}) is {}",
            print_exp::<X, P>(exp),
            print_type(&typ),
            P::p_ann(data)
        ),
        Exp::Abs(data, arg, body) => format!(
            "(lambda {}. {}) is {}",
            arg,
            print_exp::<X, P>(body),
            P::p_abs(data)
        ),
        Exp::App(data, func, arg) => {
            format!(
                "({})({}) is {}",
                print_exp::<X, P>(func),
                print_exp::<X, P>(arg),
                P::p_app(data)
            )
        }
        Exp::Exp(exp) => P::p_exp(exp),
    }
}

struct PrinterPE;

impl Printer<PE> for PrinterPE {
    fn p_lit(_data_lit: &()) -> String {
        "int".into()
    }
    fn p_var(_data_lit: &()) -> String {
        "var".into()
    }
    fn p_ann(_data_lit: &()) -> String {
        "ann".into()
    }
    fn p_abs(_data_lit: &()) -> String {
        "abs".into()
    }
    fn p_app(_data_lit: &()) -> String {
        "app".into()
    }
    fn p_exp(data_lit: &i32) -> String {
        format!("{data_lit} is partially evaluated")
    }
}

// 3.8 Replacing constructors
// Omitted here, since it appears to be a special case of section 3.4 (if I'm correct).

// 3.9 Extensions using type parameters
use std::marker::PhantomData;
struct LE<A> {
    _marker: PhantomData<A>,
}
impl<A> Descriptor for LE<A> {
    type Lit = ();
    type Var = ();
    type Ann = ();
    type Abs = ();
    type App = ();
    type Exp = (A, Exp<LE<A>>, Exp<LE<A>>);
}

// 3.10 Existentials and GADTs
// I couldn't find a straightforward way to realize GADTs in Rust's algebraic data type system.

fn main() {
    // This crate is an example module. The `main` function is intentionally minimal.
}
