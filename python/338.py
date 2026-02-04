"""Project Euler Problem 338 - Cutting Rectangles.

G(N) = sum_{2<=k<=N} floor(N/k)*floor(N/(k-1))
     - numTripletsWithProductAtMost(N)
     + sumFloorQuotients(N)
all mod 10^8.
"""
from math import isqrt

def solve():
    N = 10**12
    M = 10**8
    L = isqrt(N)

    # Part 1: sum_{k=2}^N floor(N/k)*floor(N/(k-1))
    # Split into k <= L and grouping for k > L
    ans = 0

    # For k = 2 to L:
    for k in range(2, L + 1):
        ans = (ans + (N // k % M) * (N // (k - 1) % M)) % M

    # For k > L, group by t = floor(N/k)
    # When k > L, floor(N/k) < L. Let t = floor(N/k), then k ranges over
    # [N/(t+1)+1, N/t]. But we need floor(N/(k-1)) too.
    # The Java code uses: for t=1 to N/L - 1:
    # ans += ((N/t - N/(t+1) - 1) % M * sq(t, M) + t*(t+1)) % M
    # where sq(t, M) = t*t % M
    for t in range(1, N // L):
        block = N // t - N // (t + 1)
        # In this block, floor(N/k) = t and floor(N/(k-1)) = t or t+1
        # Most have floor(N/(k-1)) = t, except possibly one has t+1
        # The Java formula: (block - 1) * t^2 + t * (t+1) = (block-1)*t^2 + t^2 + t = block*t^2 + t
        # Wait, let me re-derive from Java:
        # ans += ((N/t - N/(t+1) - 1) % M * sq(t, M) + t * (t + 1)) % M
        # = (block - 1) * t^2 + t*(t+1) = (block-1)*t^2 + t^2 + t = block*t^2 + t
        # But that doesn't seem right for all cases. Let me just port the formula exactly.
        val = ((block - 1) % M * (t % M * (t % M) % M) % M + t % M * ((t + 1) % M) % M) % M
        ans = (ans + val) % M

    # Part 2: numTripletsWithProductAtMost(N)
    # Count (a,b,c) with a*b*c <= N, a,b,c >= 1
    # = sum_{a=1}^{cbrt(N)} sum_{b=a}^{sqrt(N/a)} floor(N/(a*b))
    # With careful counting for permutations
    def count_triplets(N):
        """Count ordered triplets (a,b,c) with a*b*c <= N."""
        # = sum_{a=1}^N d(N//a) where d is divisor count sum
        # = sum_{a=1}^N sum_{b=1}^{N//a} floor(N//(a*b))
        # More efficient: O(N^{2/3})
        # Use: sum_{a=1}^N floor(N/a) can be computed in O(sqrt(N))
        # For triplets: sum_{a=1}^N D(floor(N/a)) where D(m) = sum_{b=1}^m floor(m/b)

        cbrt_n = int(round(N ** (1/3)))
        while (cbrt_n + 1) ** 3 <= N:
            cbrt_n += 1
        while cbrt_n ** 3 > N:
            cbrt_n -= 1

        # Method: count all (a,b,c) with 1 <= a <= b <= c and abc <= N
        # Multiply by appropriate permutation factor
        # Total ordered = sum over unordered * multiplicity

        # Direct: sum_{a=1}^{N} sum_{b=1}^{N//a} floor(N//(ab))
        # Use hyperbola method
        total = 0

        # For small a: a = 1 to cbrt_n
        # For each a, compute sum_{b=1}^{N//a} floor(N//(ab)) using O(sqrt(N/a))
        def sum_floor_quot(m):
            """sum_{b=1}^m floor(m/b)"""
            s = isqrt(m)
            result = 0
            for b in range(1, s + 1):
                result += m // b
            result = 2 * result - s * s
            return result

        for a in range(1, cbrt_n + 1):
            ma = N // a
            total += sum_floor_quot(ma)

        # For a > cbrt_n, floor(N/a) < N^{2/3}
        # Group by v = floor(N/a)
        # For each v, the number of a values is floor(N/v) - floor(N/(v+1))
        # And each contributes sum_floor_quot(v)
        # But v ranges from 1 to N // (cbrt_n + 1)
        # We need to enumerate distinct v values

        max_v = N // (cbrt_n + 1)
        # All distinct values of floor(N/a) for a > cbrt_n are <= max_v
        # Enumerate them
        v = 1
        while v <= max_v:
            a_lo = N // (v + 1) + 1 if v < N else 1
            a_hi = N // v
            # a ranges from max(a_lo, cbrt_n+1) to a_hi
            a_lo = max(a_lo, cbrt_n + 1)
            if a_lo <= a_hi:
                count_a = a_hi - a_lo + 1
                total += count_a * sum_floor_quot(v)
            # Next distinct v
            if v < max_v:
                # Next v is floor(N / (floor(N/(v+1))))
                next_a = N // (v + 1)
                if next_a <= cbrt_n:
                    break
                v = N // next_a
            else:
                break
            if v > max_v:
                break

        return total % M

    # Actually the above is getting complex. Let me use a simpler O(N^{2/3}) approach:
    # sum_{a=1}^N sum_{b=1}^{N/a} floor(N/(ab))
    # = sum_{a=1}^N D(floor(N/a)) where D(m) = sum_{k=1}^m floor(m/k)

    def sum_floor_quotients(m):
        """sum_{k=1}^m floor(m/k) in O(sqrt(m))."""
        s = isqrt(m)
        result = 0
        for k in range(1, s + 1):
            result += m // k
        result = 2 * result - s * s
        return result

    def num_triplets_mod(N, M):
        """Count ordered triplets (a,b,c) with a*b*c <= N, mod M."""
        # = sum_{a=1}^N D(floor(N/a))
        # Group by distinct values of floor(N/a)
        total = 0
        a = 1
        while a <= N:
            v = N // a
            # Find range of a with same floor(N/a) = v
            a_end = N // v
            count = a_end - a + 1
            dfq = sum_floor_quotients(v)
            total = (total + count % M * (dfq % M)) % M
            a = a_end + 1
        return total

    triplets = num_triplets_mod(N, M)

    # Part 3: sumFloorQuotients(N) = sum_{k=1}^N floor(N/k)
    sfq = sum_floor_quotients(N) % M

    # Combine: G(N) = ans - triplets + sfq  (mod M)
    # But wait - the Java formula subtracts triplets and adds sfq:
    # ans -= NumberTheory.numTripletsWithProductAtMost(N);
    # ans += NumberTheory.sumFloorQuotients(N);
    # But the triplets function counts ordered triplets, and the Java subtracts it.
    # Actually re-reading Java:
    # ans -= numTripletsWithProductAtMost(N)
    # ans += sumFloorQuotients(N)

    result = (ans - triplets + sfq) % M
    return result

if __name__ == "__main__":
    print(solve())
