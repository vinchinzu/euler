"""Project Euler Problem 730: Shifted Pythagorean Triples.

Find the number of k-shifted Pythagorean triples, which are triples (p,q,r)
where p^2+q^2+k=r^2, GCD(p,q,r)=1, 1<=p<=q<=r, p+q+r<=N, and 0<=k<=K.

Uses inline C for performance with recursive DFS on Barning matrices.
"""

from __future__ import annotations

import os
import subprocess
import tempfile


def solve() -> int:
    """Solve Problem 730."""
    c_code = r"""
#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <stdbool.h>

#define N 100000000
#define K 100
#define L 200

int gcd(int a, int b) {
    while (b) { int t = b; b = a % b; a = t; }
    return a;
}

bool used[K+1][L][L];
long long ans = 0;

void helper(int k, int a, int b, int c) {
    if (a + b + c > N) return;
    if (a > b) { helper(k, b, a, c); return; }
    if (a < L && b < L && k <= K) {
        if (used[k][a][b]) return;
        used[k][a][b] = true;
    }
    ans++;
    helper(k, a - 2*b + 2*c, 2*a - b + 2*c, 2*a - 2*b + 3*c);
    helper(k, a + 2*b + 2*c, 2*a + b + 2*c, 2*a + 2*b + 3*c);
    if (a != b)
        helper(k, -a + 2*b + 2*c, -2*a + b + 2*c, -2*a + 2*b + 3*c);
}

int main() {
    for (int k = 0; k <= K; k++)
        for (int p = 1; p < L; p++)
            for (int q = p; q < L; q++) {
                long long r2 = (long long)p*p + (long long)q*q + k;
                int r = (int)sqrt((double)r2);
                while ((long long)r*r < r2) r++;
                while ((long long)r*r > r2) r--;
                if ((long long)r*r == r2 && p+q+r <= N && gcd(gcd(p,q),r) == 1)
                    helper(k, p, q, r);
            }
    printf("%lld\n", ans);
    return 0;
}
"""
    tmpdir = tempfile.mkdtemp()
    c_file = os.path.join(tmpdir, "p730.c")
    exe_file = os.path.join(tmpdir, "p730")

    with open(c_file, "w") as f:
        f.write(c_code)

    subprocess.run(["gcc", "-O2", "-o", exe_file, c_file, "-lm"],
                   check=True, capture_output=True)

    result = subprocess.run([exe_file], capture_output=True, text=True, check=True)
    ans = int(result.stdout.strip())

    os.unlink(c_file)
    os.unlink(exe_file)
    os.rmdir(tmpdir)

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
