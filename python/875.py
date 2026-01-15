"""
Problem 875. Quadruple Congruence
For a positive integer n we define q(n) to be the number of solutions to:
a_1^2+a_2^2+a_3^2+a_4^2 ≡ b_1^2+b_2^2+b_3^2+b_4^2 (mod n)
where 0 ≤ a_i, b_i < n. For example, q(4)= 18432.
Define Q(n)=∑_{i=1}^{n}q(i). You are given Q(10)=18573381.
Find Q(12345678). Give your answer modulo 1001961001.
"""
from typing import List


def power(a: int, b: int, m: int) -> int:
    res = 1
    a %= m
    while b > 0:
        if b % 2 == 1:
            res = (res * a) % m
        a = (a * a) % m
        b //= 2
    return res


def solve(N: int) -> int:
    M = 1001961001

    # Using lists instead of numpy for pure python
    # Pre-allocate lists
    spf: List[int] = [0] * (N + 1)
    pe: List[int] = [0] * (N + 1)
    q: List[int] = [0] * (N + 1)
    primes: List[int] = []

    q[1] = 1

    for i in range(2, N + 1):
        if spf[i] == 0:
            spf[i] = i
            pe[i] = i
            primes.append(i)

            # Compute q(p)
            p = i
            if p == 2:
                q[i] = 128
            else:
                p3 = power(p, 3, M)
                p7 = power(p, 7, M)
                # p^7 + (p-1)p^3
                term2 = ((p - 1) * p3) % M
                q[i] = (p7 + term2) % M

        # Iterate through primes
        for p in primes:
            if p > spf[i] or i * p > N:
                break

            next_val = i * p
            spf[next_val] = p

            if p == spf[i]:
                # p divides i.
                prev_pk = pe[i]
                next_pk = prev_pk * p
                pe[next_val] = next_pk

                # Compute q(next_pk) from q(prev_pk)
                if p == 2:
                    # q(2^{k+1}) = 128 * q(2^k) + 2^{4(k+1)+3}
                    # prev_pk = 2^k
                    term = power(prev_pk, 4, M)
                    term = (term * 128) % M  # 2^7 = 128

                    val = (128 * q[prev_pk]) % M
                    val = (val + term) % M
                    q_next_pk = val
                else:
                    # q(p^{k+1}) = p^7 * q(p^k) + (p-1) * p^{4(k+1)-1}
                    p3 = power(p, 3, M)
                    p7 = power(p, 7, M)

                    term = power(prev_pk, 4, M)
                    term = (term * p3) % M
                    term = (term * (p - 1)) % M

                    val = (p7 * q[prev_pk]) % M
                    val = (val + term) % M
                    q_next_pk = val

                if next_val == next_pk:
                    q[next_val] = q_next_pk
                else:
                    rest = next_val // next_pk
                    q[next_val] = (q_next_pk * q[rest]) % M
            else:
                # p does not divide i
                pe[next_val] = p
                q[next_val] = (q[i] * q[p]) % M

    total_sum = sum(q) % M
    return total_sum


if __name__ == "__main__":
    # Validate Q(10)
    q10 = solve(10)
    if q10 != 18573381:
        print(f"Error: Q(10) = {q10}, expected 18573381")
    else:
        # print(f"Q(10) verified: {q10}")
        # Only print the final answer as per instructions
        print(solve(12345678))
