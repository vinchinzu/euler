// Trace L-expression evaluation for Problem 910
// This is a helper to understand the mathematical structure.

use std::fmt;

#[derive(Clone, Debug)]
enum Expr {
    Num(u64),
    A,
    Z,
    S,
    App(Box<Expr>, Box<Expr>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Num(n) => write!(f, "{}", n),
            Expr::A => write!(f, "A"),
            Expr::Z => write!(f, "Z"),
            Expr::S => write!(f, "S"),
            Expr::App(func, arg) => {
                write!(f, "{}({})", func, arg)
            }
        }
    }
}

fn app(f: Expr, x: Expr) -> Expr {
    Expr::App(Box::new(f), Box::new(x))
}

fn reduce(expr: &Expr, depth: usize) -> Option<Expr> {
    if depth > 1000 { return None; }
    match expr {
        Expr::App(f, x) => {
            // Try A(n) -> n+1
            if let Expr::A = f.as_ref() {
                if let Expr::Num(n) = x.as_ref() {
                    return Some(Expr::Num(n + 1));
                }
            }
            // Try Z(u)(v) -> v
            if let Expr::App(f2, _u) = f.as_ref() {
                if let Expr::Z = f2.as_ref() {
                    return Some(x.as_ref().clone());
                }
            }
            // Try S(u)(v)(w) -> v(u(v)(w))
            if let Expr::App(f2, v) = f.as_ref() {
                if let Expr::App(f3, u) = f2.as_ref() {
                    if let Expr::S = f3.as_ref() {
                        let w = x;
                        // S(u)(v)(w) = v(u(v)(w))
                        let inner = app(u.as_ref().clone(), app(v.as_ref().clone(), w.as_ref().clone()));
                        let result = app(v.as_ref().clone(), inner);
                        return Some(result);
                    }
                }
            }
            // Try reducing f first, then x
            if let Some(rf) = reduce(f.as_ref(), depth + 1) {
                return Some(app(rf, x.as_ref().clone()));
            }
            if let Some(rx) = reduce(x.as_ref(), depth + 1) {
                return Some(app(f.as_ref().clone(), rx));
            }
            None
        }
        _ => None,
    }
}

fn fully_reduce(mut expr: Expr) -> Option<u64> {
    for _step in 0..100000 {
        match reduce(&expr, 0) {
            Some(next) => expr = next,
            None => {
                if let Expr::Num(n) = expr {
                    return Some(n);
                }
                eprintln!("Stuck at: {}", expr);
                return None;
            }
        }
    }
    eprintln!("Too many steps");
    None
}

fn make_c(n: u32) -> Expr {
    let mut e = Expr::Z;
    for _ in 0..n {
        e = app(Expr::S, e);
    }
    e
}

fn make_d(n: u32) -> Expr {
    let c = make_c(n);
    app(app(c, Expr::S), Expr::S)
}

fn main() {
    // Verify examples
    // S(Z)(A)(0) = 1
    let e1 = app(app(app(Expr::S, Expr::Z), Expr::A), Expr::Num(0));
    eprintln!("S(Z)(A)(0) = {:?}", fully_reduce(e1));

    // S(S)(S(S))(S(Z))(A)(0) = 6
    let e2 = app(app(app(app(app(Expr::S, Expr::S), app(Expr::S, Expr::S)), app(Expr::S, Expr::Z)), Expr::A), Expr::Num(0));
    eprintln!("S(S)(S(S))(S(Z))(A)(0) = {:?}", fully_reduce(e2));

    // Now compute D_a(D_b)(D_c)(C_d)(A)(e) for small values
    for a in 0..=4 {
        for b in 0..=3 {
            for c in 0..=3 {
                for d in 0..=3 {
                    for e_val in [0u64, 1] {
                        let expr = app(
                            app(
                                app(
                                    app(
                                        app(make_d(a), make_d(b)),
                                        make_d(c)
                                    ),
                                    make_c(d)
                                ),
                                Expr::A
                            ),
                            Expr::Num(e_val)
                        );
                        match fully_reduce(expr) {
                            Some(n) => eprintln!("F({},{},{},{},{}) = {}", a, b, c, d, e_val, n),
                            None => eprintln!("F({},{},{},{},{}) = FAILED", a, b, c, d, e_val),
                        }
                    }
                }
            }
        }
    }
}
