// Project Euler 464 - Mobius function and balanced pairs
//
// Count pairs 1 <= a <= b <= N where the counts of mu(n)=1 and mu(n)=-1
// in [a,b] are approximately balanced.

fn main() {
    const NN: usize = 20_000_000;
    const K: i64 = 100;

    // Compute Mobius function
    let mut mu = vec![1i8; NN + 1];
    let mut is_prime = vec![true; NN + 1];
    is_prime[0] = false;
    if NN >= 1 {
        is_prime[1] = false;
    }

    for i in 2..=NN {
        if is_prime[i] {
            for j in (i..=NN).step_by(i) {
                if j != i {
                    is_prime[j] = false;
                }
                if (j as u64) % ((i as u64) * (i as u64)) == 0 {
                    mu[j] = 0;
                } else {
                    mu[j] = -mu[j];
                }
            }
        }
    }

    let l = K as usize * (NN as f64).sqrt() as usize;

    let mut ans = (NN as i64) * (NN as i64 + 1) / 2;

    for sign in [1i8, -1i8] {
        let tree_size = NN + l + 2;
        let mut bit = vec![0i64; tree_size + 2];

        let bit_add = |bit: &mut Vec<i64>, mut idx: usize, val: i64| {
            idx += 1;
            while idx <= tree_size {
                bit[idx] += val;
                idx += idx & idx.wrapping_neg();
            }
        };

        let bit_sum = |bit: &Vec<i64>, mut idx: usize| -> i64 {
            idx += 1;
            let mut result = 0i64;
            while idx > 0 {
                result += bit[idx];
                idx -= idx & idx.wrapping_neg();
            }
            result
        };

        let mut f: i64 = 0;
        for b in 1..=NN {
            bit_add(&mut bit, (f + l as i64) as usize, 1);
            if mu[b] == sign {
                f += K;
            } else if mu[b] == -sign {
                f -= K - 1;
            }
            ans -= bit_sum(&bit, NN + l) - bit_sum(&bit, (f + l as i64) as usize);
        }
    }

    println!("{}", ans);
}
