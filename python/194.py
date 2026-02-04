"""Project Euler Problem 194 - Coloured Configurations.

A-unit and B-unit are specific graphs of 7 vertices sharing 2 vertices.
Find the number of C-colorings of a chain of 25 A-units and 75 B-units.

ans = C*(C-1) * C(100,25) * (P_A(C)/(C*(C-1)))^25 * (P_B(C)/(C*(C-1)))^75 mod 10^8
"""
import math

def solve():
    A_COUNT = 25
    B_COUNT = 75
    C = 1984
    MOD = 10**8

    # A-unit graph edges (vertices 0-6)
    a_edges = [(0,1),(0,2),(0,5),(1,2),(1,6),(2,3),(3,4),(4,5),(4,6),(5,6)]
    # B-unit graph edges
    b_edges = [(0,1),(0,2),(0,5),(1,2),(1,6),(2,3),(3,4),(4,5),(4,6)]
    nv = 7

    def chromatic_poly_at(edges, c):
        """Count proper c-colorings of graph with nv vertices."""
        if c == 0:
            return 0
        if c == 1:
            return 0 if edges else 1

        adj = [set() for _ in range(nv)]
        for u, v in edges:
            adj[u].add(v)
            adj[v].add(u)

        coloring = [-1] * nv
        count = 0

        def backtrack(v):
            nonlocal count
            if v == nv:
                count += 1
                return
            used = set()
            for u in adj[v]:
                if coloring[u] >= 0:
                    used.add(coloring[u])
            for color in range(c):
                if color not in used:
                    coloring[v] = color
                    backtrack(v + 1)
                    coloring[v] = -1

        backtrack(0)
        return count

    def compute_Q_at_C(edges, C_val):
        """Compute Q(C) = P(C)/(C*(C-1)) where P is the chromatic polynomial.
        Q has degree nv-2 = 5. Use Lagrange interpolation with exact arithmetic."""
        # Evaluate Q at c = 2, 3, ..., nv (6 points for degree-5 polynomial)
        q_vals = []
        x_vals = []
        for c in range(2, nv + 1):
            p = chromatic_poly_at(edges, c)
            q = p // (c * (c - 1))
            q_vals.append(q)
            x_vals.append(c)

        # Lagrange interpolation in exact integer arithmetic
        n = len(x_vals)
        # Compute result as a fraction (numerator, denominator)
        # result = sum over i: q_vals[i] * product_{j!=i} (C_val - x_j) / (x_i - x_j)
        # Compute common denominator
        result_num = 0
        result_den = 1

        for i in range(n):
            num = q_vals[i]
            den = 1
            for j in range(n):
                if i != j:
                    num *= (C_val - x_vals[j])
                    den *= (x_vals[i] - x_vals[j])
            # Add num/den to result_num/result_den
            result_num = result_num * den + num * result_den
            result_den *= den

        # Simplify
        g = math.gcd(abs(result_num), abs(result_den))
        result_num //= g
        result_den //= g
        if result_den < 0:
            result_num = -result_num
            result_den = -result_den

        assert result_den == 1, f"Non-integer Q(C): {result_num}/{result_den}"
        return result_num % MOD

    qa = compute_Q_at_C(a_edges, C)
    qb = compute_Q_at_C(b_edges, C)

    # C(100, 25) mod MOD
    comb = math.comb(A_COUNT + B_COUNT, A_COUNT) % MOD

    # ans = C*(C-1) * comb * qa^25 * qb^75 mod MOD
    ans = C % MOD * ((C - 1) % MOD) % MOD
    ans = ans * comb % MOD
    ans = ans * pow(qa, A_COUNT, MOD) % MOD
    ans = ans * pow(qb, B_COUNT, MOD) % MOD

    return ans

if __name__ == "__main__":
    print(solve())
