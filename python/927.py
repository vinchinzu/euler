# Project Euler Problem 927
#
# PROBLEM DESCRIPTION:
# <p>A full $k$-ary tree is a tree with a single root node, such that every node is either a leaf or has exactly $k$ ordered children.  The <b>height</b> of a $k$-ary tree is the number of edges in the longest path from the root to a leaf.</p>
# 
# <p>
# For instance, there is one full 3-ary tree of height 0, one full 3-ary tree of height 1, and seven full 3-ary trees of height 2. These seven are shown below.</p>
# 
# <img src="resources/images/0927_PrimeTrees.jpg?1735590785" alt="0927_PrimeTrees.jpg">
# <p>
# For integers $n$ and $k$ with $n\ge 0$ and $k \ge 2$, define $t_k(n)$ to be the number of full $k$-ary trees of height $n$ or less.<br> 
# Thus, $t_3(0) = 1$, $t_3(1) = 2$, and $t_3(2) = 9$. Also, $t_2(0) = 1$, $t_2(1) = 2$, and $t_2(2) = 5$.</p>
# 
# <p>
# Define $S_k$ to be the set of positive integers $m$ such that $m$ divides $t_k(n)$ for some integer $n\ge 0$.  For instance, the above values show that 1, 2, and 5 are in $S_2$ and 1, 2, 3, and 9 are in $S_3$.</p>
# 
# <p>
# Let $S = \bigcap_p S_p$ where the intersection is taken over all primes $p$.  Finally, define $R(N)$ to be the sum of all elements of $S$ not exceeding $N$.  You are given that $R(20) = 18$ and $R(1000) = 2089$.</p>
# 
# <p>
# Find $R(10^7)$.</p>
#

import heapq
import time
import os
import subprocess
import tempfile

# C code for prime checking
C_CODE = r"""
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <stdbool.h>

// Dynamically allocated arrays
int *spf;
int *visited;
int current_generation = 0;
int N_limit;

void sieve(int n) {
    for (int i = 0; i <= n; i++) spf[i] = i;
    for (int i = 2; i * i <= n; i++) {
        if (spf[i] == i) {
            for (int j = i * i; j <= n; j += i)
                if (spf[j] == j)
                    spf[j] = i;
        }
    }
}

unsigned long long power(unsigned long long base, unsigned long long exp, unsigned long long mod) {
    unsigned long long res = 1;
    base %= mod;
    while (exp > 0) {
        if (exp % 2 == 1) res = (res * base) % mod;
        base = (base * base) % mod;
        exp /= 2;
    }
    return res;
}

bool check_reachability(int m, int exponent) {
    current_generation++;
    unsigned long long x = 1;

    while (1) {
        if (visited[x] == current_generation) return false;
        visited[x] = current_generation;
        if (x == 0) return true;
        x = (1 + power(x, exponent, m)) % m;
        if (x == 0) return true;
    }
}

bool is_prime_in_S(int q) {
    if (q == 2) return true;
    int phi = q - 1;

    if (phi % 2 == 0) {
        if (!check_reachability(q, 2)) return false;
        while (phi % 2 == 0) phi /= 2;
    }
    while (phi > 1) {
        int p = spf[phi];
        if (!check_reachability(q, p)) return false;
        while (phi % p == 0) phi /= p;
    }
    return true;
}

int main(int argc, char *argv[]) {
    if (argc < 2) return 1;
    N_limit = atoi(argv[1]);

    spf = (int*)malloc((N_limit + 1) * sizeof(int));
    visited = (int*)calloc((N_limit + 1), sizeof(int));

    sieve(N_limit);

    for (int q = 2; q <= N_limit; q++) {
        if (spf[q] == q) {
            if (is_prime_in_S(q)) {
                printf("%d\n", q);
            }
        }
    }

    free(spf);
    free(visited);
    return 0;
}
"""

def get_prime_factors(n):
    factors = set()
    d = 2
    temp = n
    while d * d <= temp:
        if temp % d == 0:
            factors.add(d)
            while temp % d == 0:
                temp //= d
        d += 1
    if temp > 1:
        factors.add(temp)
    return factors

def check_reachability_py(m, exponent):
    """
    Checks if 0 is reachable in the sequence x_0=1, x_{k+1} = 1 + x_k^exponent mod m.
    """
    seen = set()
    x = 1
    while x not in seen:
        if x == 0: return True
        seen.add(x)
        x = (1 + pow(x, exponent, m)) % m
        if x == 0: return True
    return False

def is_composite_in_S(m):
    """
    Checks if a composite m is in S.
    Condition: For all prime factors p of phi(m), the map x -> 1+x^p mod m reaches 0.
    """
    factors_m = get_prime_factors(m)
    factors_phi = set()
    for p in factors_m:
        factors_phi.update(get_prime_factors(p - 1))
        if m % (p*p) == 0:
            factors_phi.add(p)

    if 2 in factors_phi:
        if not check_reachability_py(m, 2):
            return False
        factors_phi.remove(2)

    for p in factors_phi:
        if not check_reachability_py(m, p):
            return False
    return True

def compile_and_run_c_checker(N):
    # Create temporary C file
    with tempfile.NamedTemporaryFile(suffix='.c', mode='w', delete=False) as c_file:
        c_file.write(C_CODE)
        c_path = c_file.name

    exe_path = c_path + ".exe" # or just no extension

    try:
        # Compile
        subprocess.check_call(['gcc', '-O3', c_path, '-o', exe_path])

        # Run
        output = subprocess.check_output([exe_path, str(N)], text=True)
        primes = [int(line) for line in output.splitlines() if line.strip()]
        return primes

    finally:
        # Cleanup
        if os.path.exists(c_path): os.remove(c_path)
        if os.path.exists(exe_path): os.remove(exe_path)

def solve(N):
    print(f"Solving for R({N})...")

    # 1. Get primes in S from C
    s_primes = compile_and_run_c_checker(N)
    print(f"Primes in S: {s_primes}")

    # 2. Generate numbers in S
    results = []
    heap = [1]
    seen = {1}
    s_primes_sorted = sorted(s_primes)

    while heap:
        curr = heapq.heappop(heap)

        if curr == 1:
            results.append(1)
        elif curr in s_primes:
             results.append(curr)
        else:
            if is_composite_in_S(curr):
                results.append(curr)

        for p in s_primes_sorted:
            nxt = curr * p
            if nxt <= N:
                if nxt not in seen:
                    seen.add(nxt)
                    heapq.heappush(heap, nxt)
            else:
                break

    total_sum = sum(results)
    print(f"Found {len(results)} elements in S.")
    print(f"R({N}) = {total_sum}")
    return total_sum

def main():
    start_time = time.time()
    result = solve(10000000)
    print(f"Execution time: {time.time() - start_time:.2f} seconds")

    # Write answer to file
    with open(os.path.join(os.path.dirname(__file__), 'answer.txt'), 'w') as f:
        f.write(str(result))

if __name__ == "__main__":
    main()
