# Project Euler Problem 919
#
# PROBLEM DESCRIPTION:
# <p>We call a triangle <i>fortunate</i> if it has integral sides and at least one of its vertices has the property that the distance from it to the triangle's <b>orthocentre</b> is exactly half the distance from the same vertex to the triangle's <b>circumcentre</b>.</p>
# <center><img src="resources/images/0919_remarkablediagram.jpg?1731700434" alt="0919_remarkablediagram.jpg" height="400"></center>
# <p>
# Triangle $ABC$ above is an example of a fortunate triangle with sides $(6,7,8)$. The distance from the vertex $C$ to the circumcentre $O$ is $\approx 4.131182$, while the distance from $C$ to the orthocentre $H$ is half that, at $\approx 2.065591$.
# </p>
# <p>
# Define $S(P)$ to be the sum of $a+b+c$ over all fortunate triangles with sides $a\leq b\leq c$ and perimeter not exceeding $P$.
# </p>
# <p>
# For example $S(10)=24$, arising from three triangles with sides $(1,2,2)$, $(2,3,4)$, and $(2,4,4)$. You are also given $S(100)=3331$.
# </p>
# <p>
# Find $S(10^7)$.
# </p>
#

import math

def gcd(a, b):
    while b:
        a, b = b, a % b
    return a

def solve(limit=10000000):
    """
    Main solution function.
    
    Args:
        limit: Max perimeter
    
    Returns:
        Solution result S(limit)
    """
    seen_primitives = set()
    total_sum = 0

    # Generator 1: u^2 + 15v^2
    # Fundamental solutions derived from: (4c)^2 = (4a +/- b)^2 + 15b^2
    # Parametrization:
    # 4c = k(u^2 + 15v^2)
    # b = 2kuv
    # 4a = k|15v^2 - u^2 +/- 2uv|

    # Limits for u, v
    # 4c approx k u^2 or k 15v^2.
    # Max perimeter P <= limit.
    # We use a safer bound of 2.5 * limit to ensure coverage.

    max_v = int((2.5 * limit / 15)**0.5) + 2
    max_u = int((2.5 * limit)**0.5) + 2

    for v in range(1, max_v):
        for u in range(1, max_u):
            if gcd(u, v) != 1:
                continue

            # Fundamental solution for this (u, v)
            if (u % 2 != 0) and (v % 2 != 0):
                # Both odd. k can be 1.
                c_val = u**2 + 15*v**2
                if c_val % 4 != 0: continue
                c = c_val // 4

                b = 2*u*v

                # Check 1
                val1 = 15*v**2 - u**2 + 2*u*v
                if val1 % 4 == 0:
                    a1 = abs(val1) // 4
                    if a1 > 0:
                        tri1 = tuple(sorted((a1, b, c)))
                        add_primitive(tri1, limit, seen_primitives)

                # Check 2
                val2 = 15*v**2 - u**2 - 2*u*v
                if val2 % 4 == 0:
                    a2 = abs(val2) // 4
                    if a2 > 0:
                        tri2 = tuple(sorted((a2, b, c)))
                        add_primitive(tri2, limit, seen_primitives)

            else:
                # Mixed parity. k must be multiple of 4.
                # Let k=4.
                c = u**2 + 15*v**2
                b = 8*u*v
                a1 = abs(15*v**2 - u**2 + 2*u*v)
                if a1 > 0:
                    tri1 = tuple(sorted((a1, b, c)))
                    add_primitive(tri1, limit, seen_primitives)

                a2 = abs(15*v**2 - u**2 - 2*u*v)
                if a2 > 0:
                    tri2 = tuple(sorted((a2, b, c)))
                    add_primitive(tri2, limit, seen_primitives)

    # Generator 2: 3u^2 + 5v^2

    max_v = int((2.5 * limit / 5)**0.5) + 2
    max_u = int((2.5 * limit / 3)**0.5) + 2

    for v in range(1, max_v):
        for u in range(1, max_u):
            if gcd(u, v) != 1:
                continue

            if (u % 2 != 0) and (v % 2 != 0):
                # Both odd. k=1.
                c_val = 3*u**2 + 5*v**2
                if c_val % 4 != 0: continue
                c = c_val // 4
                b = 2*u*v

                val1 = 5*v**2 - 3*u**2 + 2*u*v
                if val1 % 4 == 0:
                    a1 = abs(val1) // 4
                    if a1 > 0:
                        tri1 = tuple(sorted((a1, b, c)))
                        add_primitive(tri1, limit, seen_primitives)

                val2 = 5*v**2 - 3*u**2 - 2*u*v
                if val2 % 4 == 0:
                    a2 = abs(val2) // 4
                    if a2 > 0:
                        tri2 = tuple(sorted((a2, b, c)))
                        add_primitive(tri2, limit, seen_primitives)
            else:
                # Mixed parity. k=4.
                c = 3*u**2 + 5*v**2
                b = 8*u*v
                a1 = abs(5*v**2 - 3*u**2 + 2*u*v)
                if a1 > 0:
                    tri1 = tuple(sorted((a1, b, c)))
                    add_primitive(tri1, limit, seen_primitives)

                a2 = abs(5*v**2 - 3*u**2 - 2*u*v)
                if a2 > 0:
                    tri2 = tuple(sorted((a2, b, c)))
                    add_primitive(tri2, limit, seen_primitives)

    # Sum up
    ans = 0
    for tri in seen_primitives:
        p = sum(tri)
        if p > limit: continue
        count = limit // p
        ans += p * count * (count + 1) // 2

    return ans

def add_primitive(tri, limit, seen):
    # tri is sorted
    a, b, c = tri
    if a + b <= c: return # Invalid triangle
    if a <= 0: return
    # Reduce to primitive
    g = gcd(a, gcd(b, c))
    prim = (a//g, b//g, c//g)
    if sum(prim) <= limit:
        seen.add(prim)

def main():
    """Main entry point"""
    import sys
    
    if len(sys.argv) > 1:
        n = int(sys.argv[1])
        result = solve(n)
    else:
        result = solve()
    
    print(result)


if __name__ == "__main__":
    main()
