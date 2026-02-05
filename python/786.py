"""Project Euler Problem 786: Billiard Ball Bounces."""

import subprocess
import tempfile
import os

def solve():
    c_code = r'''
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <math.h>

typedef long long ll;

#define N 1000000000LL
#define L ((3 * N + 5) / 2)

ll isqrt_ll(ll n) {
    ll r = (ll)sqrt((double)n);
    while (r * r > n) r--;
    while ((r + 1) * (r + 1) <= n) r++;
    return r;
}

// Compute mobius function
signed char *mobius;

void compute_mobius(ll limit) {
    mobius = calloc(limit + 1, sizeof(signed char));

    for (ll i = 1; i <= limit; i++)
        mobius[i] = 1;

    char *is_prime = calloc(limit + 1, 1);
    for (ll i = 2; i <= limit; i++) {
        if (!is_prime[i]) {
            for (ll j = i; j <= limit; j += i) {
                is_prime[j] = 1;
                if ((j / i) % i == 0)
                    mobius[j] = 0;
                else
                    mobius[j] = -mobius[j];
            }
        }
    }
    free(is_prime);
}

// Count lattice points (x, y) where:
// 1 <= x <= t/5
// y >= 1
// 5x + d*y <= t
ll num_lattice_points(ll t, int d) {
    ll count = 0;
    ll x_max = t / 5;

    for (ll x = 1; x <= x_max; x++) {
        ll remainder = t - 5 * x;
        if (remainder >= d) {
            count += remainder / d;
        }
    }
    return count;
}

int main() {
    ll g_limit = L / 5;

    // Compute mobius
    compute_mobius(g_limit);

    ll ans = 0;

    // Direct sum over all g from 1 to g_limit
    for (ll g = 1; g <= g_limit; g++) {
        if (mobius[g] != 0) {
            ll t = L / g;
            int d = (g % 3 == 0) ? 3 : 9;
            ans += mobius[g] * num_lattice_points(t, d);
        }
    }

    ans *= 4;
    ans += 2;

    printf("%lld\n", ans);

    free(mobius);
    return 0;
}
'''

    with tempfile.NamedTemporaryFile(suffix='.c', delete=False) as f:
        f.write(c_code.encode())
        c_file = f.name
    exe = c_file[:-2]
    subprocess.run(['gcc', '-O3', '-march=native', '-lm', '-o', exe, c_file], check=True)
    result = subprocess.check_output([exe]).decode().strip()
    os.unlink(c_file)
    os.unlink(exe)
    print(result)

if __name__ == "__main__":
    solve()
