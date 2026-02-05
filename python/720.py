"""Project Euler Problem 720: Unpredictable Permutations."""

import subprocess
import tempfile
import os

def solve():
    c_code = r'''
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

#define N 25
#define M 1000000007LL
#define L (1 << N)

typedef long long ll;
typedef __int128 lll;

int main() {
    int *elements = malloc(L * sizeof(int));
    int *ranks = malloc(L * sizeof(int));

    elements[0] = 1; elements[1] = 3; elements[2] = 2; elements[3] = 4;
    ranks[0] = 1; ranks[1] = 2; ranks[2] = 2; ranks[3] = 4;

    for (int i = 4; i < L; i *= 2) {
        for (int j = 0; j < i; j++) {
            ranks[i + j] = ranks[j] + elements[j];
            elements[i + j] = 2 * elements[j];
            elements[j] = 2 * elements[j] - 1;
        }
        elements[i - 1] = 2;
        elements[i] = 2 * i - 1;
        ranks[i - 1] = 2;
        ranks[i] = i + 1;
    }

    // Precompute factorials mod M
    ll *factorials = malloc(L * sizeof(ll));
    factorials[0] = 1;
    for (int i = 1; i < L; i++) {
        factorials[i] = (factorials[i-1] * i) % M;
    }

    ll ans = 1;
    for (int i = 0; i < L; i++) {
        ll diff = elements[i] - ranks[i];
        ans = (ans + factorials[L - 1 - i] * diff) % M;
        if (ans < 0) ans += M;
    }

    printf("%lld\n", ans);

    free(elements);
    free(ranks);
    free(factorials);
    return 0;
}
'''
    with tempfile.NamedTemporaryFile(suffix='.c', delete=False) as f:
        f.write(c_code.encode())
        c_file = f.name
    exe = c_file[:-2]
    subprocess.run(['gcc', '-O3', '-march=native', '-o', exe, c_file], check=True)
    result = subprocess.check_output([exe]).decode().strip()
    os.unlink(c_file)
    os.unlink(exe)
    print(result)

if __name__ == "__main__":
    solve()
