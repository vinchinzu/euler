fn do_backward(psums: &[i32], init_c: i32, np: usize) -> bool {
    let mut c = init_c;
    for i in (0..np).rev() {
        let s = psums[i] + c;
        if (s % 10) % 2 == 0 { return false; }
        c = s / 10;
    }
    true
}

fn rec(layer: usize, cin: i32, psums: &mut [i32], np: usize, is_odd: bool) -> i32 {
    if layer == np {
        if !is_odd {
            return if do_backward(psums, cin, np) { 1 } else { 0 };
        } else {
            let mut c = 0;
            let minm = if np == 0 { 1 } else { 0 };
            for m in minm..10 {
                let sm = 2 * m + cin;
                if (sm % 10) % 2 == 1 {
                    let com = sm / 10;
                    if do_backward(psums, com, np) { c += 1; }
                }
            }
            return c;
        }
    }

    let mina = if layer == 0 { 1 } else { 0 };
    let minb = if layer == 0 { 1 } else { 0 };
    let mut cc = 0;
    for a in mina..10 {
        for b in minb..10 {
            let psu = a + b;
            let sf = psu + cin;
            if (sf % 10) % 2 == 1 {
                let cou = sf / 10;
                psums[layer] = psu;
                cc += rec(layer + 1, cou, psums, np, is_odd);
            }
        }
    }
    cc
}

fn count_for_len(length: usize) -> i32 {
    let np = length / 2;
    let is_odd = length % 2 == 1;
    let mut psums = [0i32; 10];
    rec(0, 0, &mut psums, np, is_odd)
}

fn main() {
    let mut total = 0;
    for length in 1..=9 {
        total += count_for_len(length);
    }
    println!("{}", total);
}
