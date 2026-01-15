
import sys
import time

def solve():
    # T(1) = 1. Start with r=1.
    rn, rd = 1, 1
    count = 1
    limit = 123456

    # Buffer size for search ahead
    buffer = 5000

    print(f"Starting solve for T({limit})...")
    start_time = time.time()

    while count < limit:
        # Use integer division for start_n to avoid float issues
        # r = rn/rd. start_n = floor(r) + 1.
        start_n = rn // rd + 1
        
        seq = [start_n]
        k = 1
        
        best_cn, best_cd = None, None
        found = False
        
        # Generate sequence and find candidates
        # Optimization: We can reuse k from previous step in the loop?
        # Yes, k is monotonic within the inner loop for a fixed r.
        
        for i in range(buffer):
            an = seq[i] # a[n] where n = start_n + i
            # n = start_n + i
            
            target = an * rd
            
            while True:
                if k <= start_n:
                    ak = k
                else:
                    if k - start_n >= len(seq):
                        # Should not happen if k <= n
                        break
                    ak = seq[k - start_n]

                if rn * ak >= target:
                    break
                k += 1
            
            # Candidate r' = a[n] / a[k-1]
            if k > 1:
                km1 = k - 1
                if km1 <= start_n:
                    akm1 = km1
                else:
                    akm1 = seq[km1 - start_n]
                
                # Check if candidate is better
                # We want smallest candidate > current r.
                # All candidates generated here are derived from the crossing point,
                # so they are potentially valid transitions.
                # However, we need the one that is closest to r (smallest > r).
                
                # Check if candidate > current r
                # an/akm1 > rn/rd <=> an*rd > rn*akm1
                if an * rd > rn * akm1:
                    if best_cn is None:
                        best_cn, best_cd = an, akm1
                        found = True
                    else:
                        # an/akm1 < best_cn/best_cd
                        if an * best_cd < best_cn * akm1:
                            best_cn, best_cd = an, akm1
            
            # Generate next term
            an_next = an + ak
            seq.append(an_next)

        if not found:
            print(f"Warning: Buffer {buffer} insufficient at step {count}, doubling...")
            buffer *= 2
            continue

        rn, rd = best_cn, best_cd
        count += 1
        
        if count % 1000 == 0:
            elapsed = time.time() - start_time
            print(f"T({count}) = {rn/rd:.10f} (Time: {elapsed:.2f}s)")

    result = rn/rd
    print(f"T({limit}) = {result:.10f}")
    
    with open('unsolved/870/answer.txt', 'w') as f:
        f.write(f"{result:.10f}")

if __name__ == "__main__":
    solve()

