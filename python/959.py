# Project Euler Problem 959
#
# PROBLEM DESCRIPTION:
# <p>A frog is placed on the number line. Every step the frog jumps either $a$ units to the left or $b$ units to the right, both with $1/2$ probability.</p>
# 
# <p>Define $f(a, b)$ as the limit $\lim_{n \to \infty} \frac{c_n}n$ where $c_n$ is the expected number of unique numbers visited in the first $n$ steps. You are given $f(1, 1) = 0$ and $f(1, 2) \approx 0.427050983$.</p>
# 
# <p>Find $f(89, 97)$. Give your answer rounded to nine digits after the decimal point.</p>
#
# PYTHON IMPLEMENTATION NOTES:
# - Solve the problem described above
# - Implement solve() function
#

from mpmath import mp, mpf, matrix, lu_solve, binomial, nstr

mp.dps = 20

a = 89
b = 97
p = a + b
ymin = -a
ymax = b
z_list = list(range(ymin, ymax + 1))
den = mpf(2) ** p
p_list = []
for y in z_list:
    k = y + a
    if 0 <= k <= p:
        prob = binomial(p, k) / den
    else:
        prob = mpf(0)
    p_list.append(prob)

min_x = -1000
max_x = 1000
states = list(range(min_x, 0)) + list(range(1, max_x + 1))
N = len(states)
A = matrix(N, N)
b = matrix(N, 1)
for i in range(N):
    x = states[i]
    A[i, i] = mpf(1)
    for kk in range(len(z_list)):
        z = z_list[kk]
        pp = p_list[kk]
        next_x = x + z
        if next_x == 0:
            b[i] += pp
        else:
            try:
                j = states.index(next_x)
                A[i, j] -= pp
            except ValueError:
                pass  # h=0

h = lu_solve(A, b)

f = mpf(0)
for kk in range(len(z_list)):
    z = z_list[kk]
    pp = p_list[kk]
    if z != 0:
        try:
            j = states.index(z)
            h_z = h[j]
            f += pp * (1 - h_z)
        except ValueError:
            # if z not in states, but with range should be
            pass

print(nstr(f, 9))
