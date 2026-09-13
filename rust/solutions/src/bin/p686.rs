// Project Euler 686 - Powers of Two
// Find the Nth j such that 2^j starts with "123".

fn main() {
    const N: i32 = 678910;
    const LOG2: f64 = 0.3010299956639812;
    const LO: f64 = 0.08990511143939793;
    const HI: f64 = 0.09342168516223506;
    
    let mut count = 0;
    let mut j = 0i64;
    
    while count < N {
        j += 1;
        let val = (j as f64) * LOG2;
        let frac = val - val.floor();
        if frac >= LO && frac < HI {
            count += 1;
            if count == N { break; }
        }
        
        j += 1;
        let val = (j as f64) * LOG2;
        let frac = val - val.floor();
        if frac >= LO && frac < HI {
            count += 1;
            if count == N { break; }
        }
        
        j += 1;
        let val = (j as f64) * LOG2;
        let frac = val - val.floor();
        if frac >= LO && frac < HI {
            count += 1;
            if count == N { break; }
        }
        
        j += 1;
        let val = (j as f64) * LOG2;
        let frac = val - val.floor();
        if frac >= LO && frac < HI {
            count += 1;
            if count == N { break; }
        }
    }
    println!("{}", j);
}
