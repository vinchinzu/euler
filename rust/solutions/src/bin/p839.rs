// Project Euler 839 - Beans in Bowls
// BBS sequence, merge blocks, compute steps

const N: usize = 10_000_000;

fn main() {
    let mut s = vec![0i64; N];
    let m: i64 = 50515093;
    
    let mut val = 290797i64;
    unsafe { *s.get_unchecked_mut(0) = val; }
    for i in 1..N {
        val = val * val % m;
        unsafe { *s.get_unchecked_mut(i) = val; }
    }

    let mut vals = Vec::with_capacity(N);
    let mut lens = Vec::with_capacity(N);
    
    let s_ptr = s.as_ptr();
    
    for i in 0..N {
        let si = unsafe { *s_ptr.add(i) };
        vals.push(si);
        lens.push(1i64);
        
        let mut len = vals.len();
        unsafe {
            let vals_ptr = vals.as_mut_ptr();
            let lens_ptr = lens.as_mut_ptr();
            
            while len >= 2 {
                let v1 = *vals_ptr.add(len - 2);
                let l1 = *lens_ptr.add(len - 2);
                let v2 = *vals_ptr.add(len - 1);
                let l2 = *lens_ptr.add(len - 1);
                
                if (v1 + l1 - 1) * l2 <= v2 * l1 { break; }
                
                *vals_ptr.add(len - 2) = v1 + v2;
                *lens_ptr.add(len - 2) = l1 + l2;
                
                vals.set_len(len - 1);
                lens.set_len(len - 1);
                len -= 1;
            }
        }
    }

    let mut ans: i64 = 0;
    let mut idx: usize = 0;
    
    unsafe {
        let s_ptr = s.as_mut_ptr();
        
        for block_i in 0..vals.len() {
            let v = *vals.get_unchecked(block_i);
            let len = *lens.get_unchecked(block_i) as usize;
            let len_i64 = len as i64;
            
            let idx_end = idx + len;
            let process_end = idx_end.min(N - 1);
            
            let mut i = 0i64;
            while idx < process_end {
                let t_val = (v + i) / len_i64;
                let s_val = *s_ptr.add(idx);
                let diff = s_val - t_val;
                *s_ptr.add(idx + 1) += diff;
                ans += diff;
                idx += 1;
                i += 1;
            }
            
            if idx < idx_end {
                let t_val = (v + i) / len_i64;
                let s_val = *s_ptr.add(idx);
                ans += s_val - t_val;
                idx += 1;
            }
        }
    }

    println!("{}", ans);
}
