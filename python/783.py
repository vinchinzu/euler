#!/usr/bin/env python3
"""Project Euler 783 - Urns Balls Black White.

Start with kn white balls. Over n turns: add k black, remove 2k random.
B_t = black balls removed on turn t. E(n,k) = E[sum B_t^2].

Track E[C_t] and E[C_t^2] where C_t = carried-over black balls before turn t.
B_t | C_t ~ Hypergeometric(N=k*alpha, K=C_t+k, n=2k) where alpha = n-t+2.
"""

import mpmath


def solve(n=10**6, k=10):
    mpmath.mp.dps = 40
    mu = mpmath.mpf(0)   # E[C_t]
    nu = mpmath.mpf(0)   # E[C_t^2]
    total = mpmath.mpf(0)
    kf = mpmath.mpf(k)
    k2 = kf * kf

    for t in range(1, n + 1):
        alpha = mpmath.mpf(n - t + 2)
        M = kf * alpha

        P = mu + kf               # E[C_t + k]
        Q = nu + 2 * kf * mu + k2  # E[(C_t + k)^2]

        # E[B_t^2] = 2/[alpha*(M-1)] * [k*(alpha-2)*P + (2k-1)*Q]
        eb2 = 2 / (alpha * (M - 1)) * (kf * (alpha - 2) * P + (2 * kf - 1) * Q)
        total += eb2

        # Update recurrences
        if alpha > 2:
            mu = P * (alpha - 2) / alpha
            nu = (alpha - 2) / (alpha * (M - 1)) * ((M - 2 * kf - 1) * Q + 2 * kf * P)
        else:
            mu = mpmath.mpf(0)
            nu = mpmath.mpf(0)

    return round(float(total))


if __name__ == "__main__":
    print(solve())
