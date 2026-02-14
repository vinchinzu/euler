"""Project Euler Problem 570: Snowflakes.

GCD(2*4^{n-2} - 3^{n-2}, 7n+3) summed for n=3..10^7, times 6.
Embedded C for speed.
"""
import subprocess, tempfile, os

def solve():
    c_code = r"""
#include <stdio.h>

typedef long long ll;

ll mod_pow(ll base, ll exp, ll mod) {
    ll result = 1;
    base %= mod;
    if (base < 0) base += mod;
    while (exp > 0) {
        if (exp & 1) result = (unsigned __int128)result * base % mod;
        base = (unsigned __int128)base * base % mod;
        exp >>= 1;
    }
    return result;
}

ll gcd_ll(ll a, ll b) {
    if (a < 0) a = -a;
    if (b < 0) b = -b;
    while (b) { ll t = b; b = a % b; a = t; }
    return a;
}

int main() {
    ll N = 10000000;
    ll ans = 0;
    for (ll n = 3; n <= N; n++) {
        ll mod = 7 * n + 3;
        ll t1 = mod_pow(4, n - 2, mod);
        ll t2 = mod_pow(3, n - 2, mod);
        ll term = (2 * t1 - t2 + mod) % mod;
        ll g = gcd_ll(term, mod);
        ans += 6 * g;
    }
    printf("%lld\n", ans);
    return 0;
}
"""
    with tempfile.NamedTemporaryFile(suffix='.c', mode='w', delete=False) as f:
        f.write(c_code)
        c_path = f.name
    exe_path = c_path.replace('.c', '')
    try:
        subprocess.run(['gcc', '-O2', '-o', exe_path, c_path],
                       check=True, capture_output=True)
        result = subprocess.run([exe_path], capture_output=True, text=True, timeout=280)
        return result.stdout.strip()
    finally:
        for p in [c_path, exe_path]:
            if os.path.exists(p):
                os.unlink(p)

if __name__ == "__main__":
    print(solve())
