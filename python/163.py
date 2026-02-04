"""Project Euler Problem 163 - Cross-hatched triangles.

Count all triangles in a cross-hatched triangular grid of size N=36.

Represent each line by two intersection points on the outer triangle perimeter
numbered 0 to 6N-1. Three lines form a triangle if they pairwise intersect
inside the outer triangle. Subtract over-counted points where 3+ lines meet.
"""

def solve():
    N = 36

    # Generate all lines
    lines = []
    for base in range(3):
        for i in range(N + 1):
            s = (2 * N * base + 2 * i) % (6 * N)
            e = (2 * N * base - 2 * i) % (6 * N)
            lines.append((min(s, e), max(s, e)))
        for i in range(2 * N + 1):
            s = (2 * N * base + 2 * i) % (6 * N)
            e = (2 * N * base - i) % (6 * N)
            lines.append((min(s, e), max(s, e)))

    def intersect(l1, l2):
        s1, e1 = l1
        s2, e2 = l2
        return (s1 <= s2 <= e1 <= e2) or (s2 <= s1 <= e2 <= e1)

    # Count triplets of mutually intersecting lines
    n_lines = len(lines)
    ans = 0
    for i in range(n_lines):
        for j in range(i + 1, n_lines):
            if intersect(lines[i], lines[j]):
                for k in range(j + 1, n_lines):
                    if intersect(lines[i], lines[k]) and intersect(lines[j], lines[k]):
                        ans += 1

    # Subtract over-counted intersection points
    # tr(N+1) interior intersection points where 6 lines meet: C(6,3)=20 each
    # N^2 points where 3 cross-hatched lines meet: counted once extra
    def tr(n):
        return n * (n + 1) // 2

    ans -= 20 * tr(N + 1) + N * N

    return ans

if __name__ == "__main__":
    print(solve())
