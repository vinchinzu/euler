"""Project Euler Problem 477: Number sequence game.

There is a sequence of N numbers, and 2 players take turns removing either
the first or last number. Find the maximum score of the first player if both
players play optimally.

Uses the reduction algorithm: if a number b is larger than its neighbors a
and c, they can be replaced with a-b+c without changing the optimal score
difference. After reduction, greedy works.

Uses inline C for performance.
"""

from __future__ import annotations

import os
import subprocess
import tempfile


def solve() -> str:
    """Solve Problem 477."""
    c_code = r"""
#include <stdio.h>
#include <stdlib.h>

typedef long long ll;
typedef __int128 lll;

#define M 1000000007LL
#define N 100000000LL

int main() {
    /* Generate the sequence */
    ll *nums = (ll*)malloc(N * sizeof(ll));
    if (!nums) { fprintf(stderr, "malloc failed\n"); return 1; }

    ll s = 0;
    for (ll i = 0; i < N; i++) {
        nums[i] = s;
        s = (s * s + 45) % M;
    }

    /* Reduce: replace peaks a <= b >= c with a-b+c */
    ll *reduced = (ll*)malloc(N * sizeof(ll));
    int idx = 0;
    lll sum = 0;

    for (ll i = 0; i < N; i++) {
        sum += nums[i];
        reduced[idx++] = nums[i];
        while (idx >= 3 && reduced[idx-3] <= reduced[idx-2] && reduced[idx-2] >= reduced[idx-1]) {
            reduced[idx-3] += reduced[idx-1] - reduced[idx-2];
            idx -= 2;
        }
    }

    free(nums);

    /* Greedy on reduced sequence, matching Java:
       for (int start = 0, end = index - 1; start <= end;) {
           long score = reduced[start] > reduced[end] ? reduced[start++] : reduced[end--];
           reducedScore += parity(start + end) * score;
       }
       parity(n) = (n % 2 == 0) ? 1 : -1
       Note: start++ / end-- happens BEFORE parity is evaluated */
    lll reducedScore = 0;
    int start = 0, end = idx - 1;
    while (start <= end) {
        ll score;
        if (reduced[start] > reduced[end])
            score = reduced[start++];
        else
            score = reduced[end--];
        /* parity computed AFTER start/end modification */
        int parity = ((start + end) % 2 == 0) ? 1 : -1;
        reducedScore += (lll)parity * score;
    }

    free(reduced);

    lll ans = (sum + reducedScore) / 2;

    /* Print __int128 */
    ll hi = (ll)(ans / 1000000000000000LL);
    ll lo = (ll)(ans % 1000000000000000LL);
    if (hi > 0)
        printf("%lld%015lld\n", hi, lo);
    else
        printf("%lld\n", lo);
    return 0;
}
"""
    tmpdir = tempfile.mkdtemp()
    c_file = os.path.join(tmpdir, "p477.c")
    exe_file = os.path.join(tmpdir, "p477")

    with open(c_file, "w") as f:
        f.write(c_code)

    subprocess.run(["gcc", "-O2", "-o", exe_file, c_file, "-lm"],
                   check=True, capture_output=True)

    result = subprocess.run([exe_file], capture_output=True, text=True, check=True,
                           timeout=25)
    output = result.stdout.strip()

    os.unlink(c_file)
    os.unlink(exe_file)
    os.rmdir(tmpdir)

    return output


def main():
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
