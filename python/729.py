"""Project Euler Problem 729: Range of Periodic Sequence.

Find S(25) = sum of ranges of all periodic sequences a_{n+1} = a_n - 1/a_n
with period <= 25.

Uses Lyndon word enumeration over binary alphabet for each length.
Ported to C for performance.
"""

import subprocess
import tempfile
import os


def solve():
    c_code = r"""
#include <stdio.h>
#include <math.h>
#include <string.h>

#define MAXN 25

double ans = 0.0;

// For a given binary word of length 'len' (encoded as an integer),
// find the fixed point of the composed inverse map and compute the range.
// The recurrence is a_{n+1} = a_n - 1/a_n.
// Inverse: a_n = (a_{n+1} + sign * sqrt(a_{n+1}^2 + 4)) / 2
// Each bit determines the sign choice.

void process_lyndon(int *w, int len) {
    if (len < 2) return;

    // Convert to bitmask (w[0] is MSB or LSB? Let's use w[0] as step 0)
    unsigned int word = 0;
    for (int i = 0; i < len; i++) {
        if (w[i]) word |= (1u << i);
    }

    // Find fixed point by iterating the composed map
    double d = 1.0;
    for (int iter = 0; iter < 300; iter++) {
        double prev = d;
        for (int i = 0; i < len; i++) {
            int bit = (word >> i) & 1;
            double sign = (bit == 0) ? 1.0 : -1.0;
            d = (d + sign * sqrt(d * d + 4.0)) / 2.0;
        }
        if (fabs(d - prev) < 1e-13) break;
    }

    // Compute range
    double min_val = d, max_val = d;
    for (int i = 0; i < len; i++) {
        int bit = (word >> i) & 1;
        double sign = (bit == 0) ? 1.0 : -1.0;
        d = (d + sign * sqrt(d * d + 4.0)) / 2.0;
        if (d < min_val) min_val = d;
        if (d > max_val) max_val = d;
    }

    // For a Lyndon word of length len, all len rotations are distinct
    ans += len * (max_val - min_val);
}

// Generate all binary Lyndon words of EXACTLY length n using the
// standard recursive algorithm.
// gen(w, t, p, n): w[1..t-1] filled, p = period so far
// When t > n: if n % p == 0 AND p == n, output (this ensures primitive period = n)
// Actually, this standard algorithm generates ALL Lyndon words of length dividing n
// when called with gen(w, 1, 1, n) and the condition is n % p == 0.
// To get ONLY length-n Lyndon words, use condition p == n.

int w_buf[MAXN + 2];

void gen(int t, int p, int n) {
    if (t > n) {
        if (p == n) {
            // w_buf[1..n] is a Lyndon word of length exactly n
            process_lyndon(w_buf + 1, n);
        }
    } else {
        w_buf[t] = w_buf[t - p];
        gen(t + 1, p, n);
        for (int j = w_buf[t - p] + 1; j <= 1; j++) {
            w_buf[t] = j;
            gen(t + 1, t, n);
        }
    }
}

int main() {
    ans = 0.0;

    // Generate Lyndon words of each length from 2 to MAXN
    for (int n = 2; n <= MAXN; n++) {
        memset(w_buf, 0, sizeof(w_buf));
        gen(1, 1, n);
    }

    printf("%.4f\n", ans);
    return 0;
}
"""

    with tempfile.NamedTemporaryFile(mode='w', suffix='.c', delete=False) as f:
        f.write(c_code)
        c_path = f.name

    bin_path = c_path.replace('.c', '')
    try:
        subprocess.run(['gcc', '-O2', '-o', bin_path, c_path, '-lm'],
                      check=True, capture_output=True)
        result = subprocess.run([bin_path], capture_output=True, text=True, check=True,
                              timeout=30)
        return result.stdout.strip()
    finally:
        os.unlink(c_path)
        if os.path.exists(bin_path):
            os.unlink(bin_path)


def main():
    result = solve()
    print(result)


if __name__ == "__main__":
    main()
