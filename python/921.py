import time

def get_pisano_period(m):
    # Returns pi(m)
    # For prime p:
    # if p = 1 or 4 mod 5, pi(p) | p-1
    # if p = 2 or 3 mod 5, pi(p) | 2(p+1)
    # We can find it by factoring the candidate period.
    # But for general m, we need prime factorization.
    # Let's assume m is prime for now.
    if m == 2: return 3
    if m == 5: return 20
    
    # Check if prime
    is_prime = True
    if m % 2 == 0: is_prime = False
    else:
        for i in range(3, int(m**0.5)+1, 2):
            if m % i == 0:
                is_prime = False
                break
    
    if is_prime:
        if m % 5 == 1 or m % 5 == 4:
            k = m - 1
        else:
            k = 2 * (m + 1)
        # The period divides k. We can find the smallest divisor.
        # For this problem, we just need a multiple of the period, 
        # but the order of 5 mod pi(m) depends on the exact period.
        # So we should find the exact period.
        return find_period_divisor(k, m)
    else:
        # If not prime, we need to factorize.
        print(f"{m} is not prime!")
        return None

def mat_mul(A, B, m):
    return (
        (A[0]*B[0] + A[1]*B[2]) % m,
        (A[0]*B[1] + A[1]*B[3]) % m,
        (A[2]*B[0] + A[3]*B[2]) % m,
        (A[2]*B[1] + A[3]*B[3]) % m
    )

def mat_pow(A, p, m):
    res = (1, 0, 0, 1)
    while p > 0:
        if p % 2 == 1:
            res = mat_mul(res, A, m)
        A = mat_mul(A, A, m)
        p //= 2
    return res

def get_fib_mod(n, m):
    if n == 0: return 0
    if n == 1: return 1
    M = (1, 1, 1, 0)
    Res = mat_pow(M, n-1, m)
    return Res[0]

def find_period_divisor(k, m):
    # Find smallest d | k such that F_d = 0 mod m and F_{d+1} = 1 mod m
    # We can iterate through divisors of k.
    divs = []
    for i in range(1, int(k**0.5)+1):
        if k % i == 0:
            divs.append(i)
            if i*i != k:
                divs.append(k//i)
    divs.sort()
    
    for d in divs:
        f0 = get_fib_mod(d, m)
        f1 = get_fib_mod(d+1, m)
        if f0 == 0 and f1 == 1:
            return d
    return k

def get_order(a, m):
    # Find smallest k such that a^k = 1 mod m
    # k divides phi(m).
    # Since we don't have phi(m) easily if m is not prime, 
    # we can try to factor m or just use a large multiple?
    # Wait, m here is pi(M). pi(M) is likely even.
    # Let's assume we can factor m or phi(m).
    # For the problem, we can just use Carmichael function or similar.
    # But finding the exact order is safer.
    # Let's just return phi(m) if we can't find order?
    # No, 5^F_i mod pi(M). We need the modulus for F_i.
    # That modulus is the order of 5 mod pi(M).
    pass

def solve():
    M = 398874989
    print(f"M = {M}")
    
    # Check if prime
    is_prime = True
    if M % 2 == 0: is_prime = False
    else:
        for i in range(3, int(M**0.5)+1, 2):
            if M % i == 0:
                is_prime = False
                print(f"Divisible by {i}")
                break
    print(f"Is prime: {is_prime}")
    
    if not is_prime:
        return

    pi_M = get_pisano_period(M)
    print(f"pi(M) = {pi_M}")
    
    # Find order of 5 mod pi_M
    # pi_M might be large.
    # We need to factor pi_M to find phi(pi_M) and then order.
    # Or just iterate? No, too slow.
    # Let's factor pi_M.
    
    temp = pi_M
    factors = {}
    d = 2
    while d*d <= temp:
        while temp % d == 0:
            factors[d] = factors.get(d, 0) + 1
            temp //= d
        d += 1
    if temp > 1:
        factors[temp] = factors.get(temp, 0) + 1
    
    print(f"Factors of pi(M): {factors}")
    
    phi_pi = 1
    for p, e in factors.items():
        phi_pi *= (p-1) * p**(e-1)
    
    print(f"phi(pi(M)) = {phi_pi}")
    
    # Order of 5 mod pi_M divides phi_pi.
    # We can check divisors of phi_pi.
    
    order_5 = phi_pi
    # Try to reduce order
    # For each prime factor of phi_pi, try to divide order_5
    
    phi_factors = {}
    temp = phi_pi
    d = 2
    while d*d <= temp:
        while temp % d == 0:
            phi_factors[d] = phi_factors.get(d, 0) + 1
            temp //= d
        d += 1
    if temp > 1:
        phi_factors[temp] = phi_factors.get(temp, 0) + 1
        
    curr_order = phi_pi
    for p in phi_factors:
        while curr_order % p == 0 and pow(5, curr_order // p, pi_M) == 1:
            curr_order //= p
            
    L = curr_order
    print(f"Order of 5 mod pi(M) is L = {L}")
    
    # Now we need F_i mod L.
    # We iterate i from 2 to m.
    m_limit = 1618034
    
    # Precompute F_i mod L
    # We can just update F_a, F_b
    
    total_S = 0
    
    f_prev = 1 # F_1
    f_curr = 1 # F_2
    # i starts at 2. F_2 = 1.
    
    # Loop
    # We need to sum s(F_i).
    # s(F_i) depends on k = 5^{F_i} mod pi_M.
    # F_i in the exponent is mod L.
    
    # We can optimize: if L is small, F_i mod L is periodic.
    # But L might be large.
    
    # Let's just iterate.
    
    # We need F_k mod M and L_k mod M.
    # k = 5^{F_i mod L} mod pi_M.
    
    # Since we iterate i, we maintain F_i mod L.
    
    start_time = time.time()
    
    # Current F_i mod L
    # i=2: F_2 = 1.
    
    a_fib = 1 # F_1
    b_fib = 1 # F_2
    
    for i in range(2, m_limit + 1):
        # b_fib is F_i mod L
        
        # Calculate K = 3 * 5^b_fib mod pi_M
        # The factor of 3 comes from a_0 = phi^3 = 2 + sqrt(5)
        # Wait, a_0 = sqrt(5) + 2 = phi^3.
        # So a_n = phi^{3 * 5^n}.
        # Let K = 3 * 5^n.
        # p_n = F_K / 2, q_n = L_K / 2.
        # But F_K and L_K might be odd?
        # Actually, for K divisible by 3, F_K and L_K are even.
        # F_3 = 2, L_3 = 4. Both even.
        # So p, q are integers.
        
        K = (3 * pow(5, b_fib, pi_M)) % pi_M
        
        # Calculate F_K, L_K mod M
        M_mat = (1, 1, 1, 0)
        Res = mat_pow(M_mat, K, M)
        F_K = Res[2]
        L_K = (Res[3] + Res[0]) % M
        
        # p = F_K / 2, q = L_K / 2
        inv_2 = (M + 1) // 2
        p = (F_K * inv_2) % M
        q = (L_K * inv_2) % M
        
        term = (pow(p, 5, M) + pow(q, 5, M)) % M
        total_S = (total_S + term) % M
        
        # Update fib
        a_fib, b_fib = b_fib, (a_fib + b_fib) % L
        
        if i % 10000 == 0:
            print(f"Processed {i}, current sum {total_S}, time {time.time()-start_time:.2f}s")

    print(f"Final Result: {total_S}")

if __name__ == "__main__":
    solve()
