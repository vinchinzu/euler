"""Project Euler Problem 195 - Inscribed circles of 60-degree triangles.

Find the number of triangles with inradius at most N=1053779 and
exactly one 60-degree angle.

Parametrization: for coprime m > 2n, the primitive inradius is
(m+n)(m-2n) / (2*sqrt(3)). If (m+n) % 3 == 0, the GCD of the
triple is 3, giving a smaller primitive inradius.

Use Stern-Brocot / Farey-like enumeration of coprime pairs via
the mediant property to avoid GCD computation.
"""
import math

def solve():
    N = 1053779
    sqrt3 = math.sqrt(3)
    inv_2sqrt3 = 1.0 / (2.0 * sqrt3)
    limit_3N = 3.0 * N

    ans = 0

    # Use a stack-based enumeration of coprime pairs (m, n) with m > 2n
    # via Stern-Brocot tree approach.
    # Actually, let's just use math.gcd which is C-implemented and fast.
    # The key insight: for large n, m range is tiny (often just 1 value).
    # Total iterations: O(N / sqrt(3)) which is about 600K.

    n = 1
    while True:
        m_start = 2 * n + 1
        # inradius at m_start
        ir_start = (m_start + n) * (m_start - 2 * n) * inv_2sqrt3
        if ir_start > limit_3N:
            break

        m = m_start
        while True:
            ir = (m + n) * (m - 2 * n) * inv_2sqrt3
            if ir > limit_3N:
                break
            if math.gcd(m, n) == 1:
                if (m + n) % 3 == 0:
                    ans += int(N / (ir / 3.0))
                else:
                    ans += int(N / ir)
            m += 1
        n += 1

    return ans

if __name__ == "__main__":
    print(solve())
