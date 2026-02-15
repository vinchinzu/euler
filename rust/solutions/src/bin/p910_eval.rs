// Evaluate L-expressions numerically for Problem 910

use std::rc::Rc;
use std::cell::Cell;

thread_local! {
    static DEPTH: Cell<u32> = Cell::new(0);
    static MAX_DEPTH: Cell<u32> = Cell::new(5000000);
}

type Fun = Rc<dyn Fn(Val) -> Val>;

#[derive(Clone)]
enum Val {
    Num(u128),
    Func(Fun),
}

impl Val {
    fn apply(self, arg: Val) -> Val {
        DEPTH.with(|d| {
            let cur = d.get();
            let max = MAX_DEPTH.with(|m| m.get());
            if cur > max { panic!("Depth exceeded"); }
            d.set(cur + 1);
            let result = match self {
                Val::Func(f) => f(arg),
                Val::Num(_) => panic!("Cannot apply a number"),
            };
            d.set(cur);
            result
        })
    }
    fn as_num(&self) -> u128 {
        match self { Val::Num(n) => *n, _ => panic!("Expected number") }
    }
}

fn make_a() -> Val { Val::Func(Rc::new(|x: Val| Val::Num(x.as_num() + 1))) }
fn make_z() -> Val { Val::Func(Rc::new(|_u: Val| Val::Func(Rc::new(move |v: Val| v)))) }
fn make_s() -> Val {
    Val::Func(Rc::new(|u: Val| {
        Val::Func(Rc::new(move |v: Val| {
            let u2 = u.clone(); let v2 = v.clone();
            Val::Func(Rc::new(move |w: Val| {
                let inner = u2.clone().apply(v2.clone()).apply(w);
                v2.clone().apply(inner)
            }))
        }))
    }))
}
fn make_c(n: u32) -> Val { if n == 0 { make_z() } else { make_s().apply(make_c(n - 1)) } }
fn make_d(n: u32) -> Val { make_c(n).apply(make_s()).apply(make_s()) }

fn eval_f(a: u32, b: u32, c: u32, d: u32, e: u128) -> Result<u128, String> {
    DEPTH.with(|dep| dep.set(0));
    let result = std::panic::catch_unwind(|| {
        make_d(a).apply(make_d(b)).apply(make_d(c)).apply(make_c(d)).apply(make_a()).apply(Val::Num(e))
    });
    match result {
        Ok(v) => Ok(v.as_num()),
        Err(e) => {
            if let Some(s) = e.downcast_ref::<String>() { Err(s.clone()) }
            else if let Some(s) = e.downcast_ref::<&str>() { Err(s.to_string()) }
            else { Err("Unknown".to_string()) }
        }
    }
}

fn main() {
    eprintln!("=== F(1,b,c,d,0) ===");
    for b in 0..=2 {
        for c in 0..=2 {
            for d in 0..=5 {
                match eval_f(1, b, c, d, 0) {
                    Ok(v) => eprintln!("F(1,{},{},{},0) = {}", b, c, d, v),
                    Err(e) => eprintln!("F(1,{},{},{},0) = ERR: {}", b, c, d, e),
                }
            }
        }
    }

    eprintln!("\n=== F(2,b,c,d,0) ===");
    for b in 0..=2 {
        for c in 0..=1 {
            for d in 0..=3 {
                match eval_f(2, b, c, d, 0) {
                    Ok(v) => eprintln!("F(2,{},{},{},0) = {}", b, c, d, v),
                    Err(e) => eprintln!("F(2,{},{},{},0) = ERR: {}", b, c, d, e),
                }
            }
        }
    }

    // Key question: what is F(a,b,c,d,e) in terms of hyperoperations?
    // Let me check F with e != 0
    eprintln!("\n=== F(1,1,1,d,e) ===");
    for d in 0..=5 {
        for e in [0u128, 1, 90] {
            match eval_f(1, 1, 1, d, e) {
                Ok(v) => eprintln!("F(1,1,1,{},{}) = {}", d, e, v),
                Err(e2) => eprintln!("F(1,1,1,{},{}) = ERR: {}", d, e, e2),
            }
        }
    }

    eprintln!("\n=== F(0,b,c,d,0) with b varying ===");
    for b in 0..=4 {
        for d in 1..=3 {
            match eval_f(0, b, 0, d, 0) {
                Ok(v) => eprintln!("F(0,{},0,{},0) = {}", b, d, v),
                Err(e) => eprintln!("F(0,{},0,{},0) = ERR: {}", b, d, e),
            }
        }
    }
}
