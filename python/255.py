"""Project Euler Problem 255 — Rounded Square Roots."""
import subprocess, tempfile, os

C_CODE = r'''
#include <stdio.h>
#include <stdlib.h>
#include <math.h>

typedef long long ll;

static ll ceil_div(ll a, ll b) {
    return (a + b - 1) / b;
}

static ll bound(ll n, ll low, ll high) {
    if (n < low) return low;
    if (n > high) return high;
    return n;
}

static ll sum_iterations(ll l, ll h, ll x, ll k);

static ll sum_remaining_iterations(ll l, ll h, ll x, ll k) {
    ll total = 0;
    while (l < h) {
        ll next_x = (x + ceil_div(l, x)) / 2;
        ll next_l_val = (next_x * 2 + 1 - x) * x + 1;
        ll next_l = next_l_val < h ? next_l_val : h;
        total += sum_iterations(l, next_l, next_x, k + 1);
        l = next_l;
    }
    return total;
}

static ll sum_iterations(ll l, ll h, ll x, ll k) {
    ll x2l = bound(x * (x - 1) + 1, l, h);
    ll x2h = bound(x * (x + 1) + 1, l, h);
    return sum_remaining_iterations(l, x2l, x, k)
         + (x2h - x2l) * k
         + sum_remaining_iterations(x2h, h, x, k);
}

int main(void) {
    ll L = 10000000000000LL;   /* 10^13 */
    ll H = 100000000000000LL;  /* 10^14 */
    ll X0 = 7000000LL;         /* 7 * 10^6 */

    ll total = sum_iterations(L, H, X0, 1);
    double result = (double)total / (double)(H - L);
    printf("%.10f\n", result);
    return 0;
}
'''

if __name__ == "__main__":
    with tempfile.NamedTemporaryFile(suffix='.c', delete=False) as f:
        f.write(C_CODE.encode())
        c_file = f.name
    exe = c_file[:-2]
    try:
        subprocess.run(['gcc', '-O2', '-o', exe, c_file, '-lm'], check=True, capture_output=True)
        result = subprocess.run([exe], capture_output=True, text=True, timeout=280)
        print(result.stdout.strip())
    finally:
        os.unlink(c_file)
        if os.path.exists(exe):
            os.unlink(exe)
