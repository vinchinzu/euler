// Project Euler 464 - Mobius function and balanced pairs
//
// Count pairs 1 <= a <= b <= N where the counts of mu(n)=1 and mu(n)=-1
// in [a,b] are approximately balanced.

const NN: usize = 20_000_000;
const K: i32 = 100;


fn mobius(n: usize) -> Vec<i8> {
    let mut mu = vec![0i8; n + 1];
    let mut spf = vec![0u32; n + 1];
    let mut primes = Vec::with_capacity(1_280_000);
    mu[1] = 1;
    for i in 2..=n {
        if spf[i] == 0 {
            spf[i] = i as u32;
            primes.push(i as u32);
            mu[i] = -1;
        }
        let spi = unsafe { *spf.get_unchecked(i) };
        for &p in &primes {
            if p > spi || p as usize > n / i {
                break;
            }
            let v = i * (p as usize);
            unsafe {
                *spf.get_unchecked_mut(v) = p;
                *mu.get_unchecked_mut(v) = if p == spi {
                    0
                } else {
                    -*mu.get_unchecked(i)
                };
            }
        }
    }
    mu
}

fn count_inversions_merge(arr: &mut [i32], temp: &mut [i32]) -> i64 {
    let len = arr.len();
    if len <= 1 {
        return 0;
    }
    if len <= 32 {
        let mut inv = 0i64;
        for i in 1..len {
            let key = arr[i];
            let mut j = i;
            while j > 0 && arr[j - 1] > key {
                arr[j] = arr[j - 1];
                j -= 1;
                inv += 1;
            }
            arr[j] = key;
        }
        return inv;
    }

    let mid = len / 2;
    let (arr_l, arr_r) = arr.split_at_mut(mid);
    let (temp_l, temp_r) = temp.split_at_mut(mid);

    let (inv_l, inv_r) = if len > 16384 {
        rayon::join(
            || count_inversions_merge(arr_l, temp_l),
            || count_inversions_merge(arr_r, temp_r),
        )
    } else {
        (
            count_inversions_merge(arr_l, temp_l),
            count_inversions_merge(arr_r, temp_r),
        )
    };

    let mut cross_inv = 0i64;
    let (mut i, mut j, mut k) = (0, 0, 0);
    while i < mid && j < len - mid {
        if arr_l[i] <= arr_r[j] {
            temp[k] = arr_l[i];
            i += 1;
        } else {
            temp[k] = arr_r[j];
            j += 1;
            cross_inv += (mid - i) as i64;
        }
        k += 1;
    }
    if i < mid {
        temp[k..k + mid - i].copy_from_slice(&arr_l[i..mid]);
    }
    if j < len - mid {
        temp[k..len].copy_from_slice(&arr_r[j..len - mid]);
    }
    arr.copy_from_slice(temp);

    inv_l + inv_r + cross_inv
}

fn count_violations(mu: &[i8], delta: [i32; 3]) -> i64 {
    let mut x = vec![0i32; NN + 1];
    let mut cur = 0i32;
    for b in 1..=NN {
        let m = unsafe { *mu.get_unchecked(b) };
        cur += delta[(m + 1) as usize];
        x[b] = cur;
    }
    let mut temp = vec![0i32; NN + 1];
    count_inversions_merge(&mut x, &mut temp)
}

fn main() {
    let mu = mobius(NN);
    let ans0 = (NN as i64) * (NN as i64 + 1) / 2;
    // sign = +1: mu=1 => +K, mu=-1 => -(K-1); sign = -1 swaps those.
    let d_pos = [-(K - 1), 0, K];
    let d_neg = [K, 0, -(K - 1)];
    let (s1, s2) = rayon::join(
        || count_violations(&mu, d_pos),
        || count_violations(&mu, d_neg),
    );
    println!("{}", ans0 - s1 - s2);
}
