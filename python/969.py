#euler 969
#Starting at zero, a kangaroo hops along the real number line in the positive direction. Each successive hop takes the kangaroo forward a uniformly random distance between $0$ and $1$. Let $H(n)$ be the expected number of hops needed for the kangaroo to pass $n$ on the real line.
#If we write $\alpha = H(1)$, then for all positive integers $n$, $H(n)$ can be expressed as a polynomial function of $\alpha$ with rational coefficients. For example $H(3)=\alpha^3-2\alpha^2+\frac{1}{2}\alpha$. Define $S(n)$ to be the sum of all <b>integer</b> coefficients in this polynomial form of $H(n)$. Therefore $S(1)=1$ and $S(3)=1+(-2)=-1$.<br>
#You are also given $\displaystyle \sum_{n=1}^{10} S(n)=43$.<br>
#Find $\displaystyle\sum_{n=1}^{10^{18}} S(n)$. Give your answer modulo $10^9+7$.

# correct 412543690
import sys
mod = 1000000007
M = 10**18
maxj = 60
primes = [2,3,5,7,11,13,17,19,23,29,31,37,41,43,47,53,59]
stir = [[0 for _ in range(maxj+1)] for _ in range(maxj+1)]
stir[0][0] = 1
for j in range(1, maxj+1):
    stir[j][0] = 0
    for i in range(1, j+1):
        stir[j][i] = (i * stir[j-1][i] + stir[j-1][i-1]) % mod
fact = [1] * (maxj+1)
for i in range(1, maxj+1):
    fact[i] = fact[i-1] * i % mod
total = 0
for j in range(maxj+1):
    P = 1
    for p in primes:
        if p > j:
            break
        P *= p
    if P > M - j + 1:
        continue
    K = (M - j) // P
    if K <= 0:
        continue
    if j == 0:
        s = K % mod
    else:
        s = 0
        for i in range(1, j+1):
            falling = 1
            for t in range(i+1):
                term = (K + 1 - t) % mod
                falling = falling * term % mod
            term = stir[j][i] * falling % mod
            inv = pow(i+1, mod-2, mod)
            term = term * inv % mod
            s = (s + term) % mod
    sign = 1 if j % 2 == 0 else -1
    if sign == -1:
        signs = mod - 1
    else:
        signs = 1
    P_mod = P % mod
    pj = pow(P_mod, j, mod)
    inv_fact = pow(fact[j], mod-2, mod)
    contrib = signs * pj % mod
    contrib = contrib * s % mod
    contrib = contrib * inv_fact % mod
    total = (total + contrib) % mod
print(total)

