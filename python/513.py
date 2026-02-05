#!/usr/bin/env python3
"""Project Euler Problem 513: Triangles with Integer Median.

Find the number of triangles with integer sides a <= b <= c <= N such that
the median to c also has integer length.
"""

import subprocess
import tempfile
import os


def solve():
    c_code = r'''
#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

typedef long long ll;
typedef __int128 lll;

#define N 100000

int8_t *mobius;

void compute_mobius(int limit) {
    mobius = (int8_t *)calloc(limit + 1, sizeof(int8_t));
    int *spf = (int *)malloc((limit + 1) * sizeof(int));

    mobius[1] = 1;
    for (int i = 0; i <= limit; i++) spf[i] = i;

    for (int i = 2; i <= limit; i++) {
        if (spf[i] == i) {
            mobius[i] = -1;
            for (ll j = (ll)i * i; j <= limit; j += i) {
                if (spf[j] == j) spf[j] = i;
            }
        } else {
            int p = spf[i];
            int q = i / p;
            if (q % p == 0) {
                mobius[i] = 0;
            } else {
                mobius[i] = -mobius[q];
            }
        }
    }
    free(spf);
}

// Directly replicate the Java logic with explicit constraint checking
// Java constraints for case 1 (first loop, k/l both even or no parity check):
//   Constraint(0, -1, -l, false)        => -q <= -l     => q >= l
//   Constraint(3k+l, -(k+l), 0, false)  => (3k+l)p - (k+l)q <= 0  => (3k+l)p <= (k+l)q
//   Constraint(-k, l, n, false)         => -kp + lq <= n => lq - kp <= n
//   Constraint(-l, k, 0, false)         => -lp + kq <= 0 => kq <= lp

// Count (p, q) with p >= 1, satisfying all constraints
ll count_pq_case1(ll k, ll l, ll n) {
    ll count = 0;
    // Iterate over p, compute valid q range
    for (ll p = 1; ; p++) {
        // From q >= l: q_min = l
        ll q_min = l;
        // Also q > p (implicit from p < q in parametrization)
        if (p >= q_min) q_min = p + 1;

        // From (3k+l)p <= (k+l)q: q >= ceil((3k+l)p / (k+l))
        ll q_bc = ((lll)(3*k + l) * p + (k + l) - 1) / (k + l);
        if (q_bc > q_min) q_min = q_bc;

        // From kq <= lp: q <= floor(lp / k)
        ll q_max = (lll)l * p / k;

        // From lq - kp <= n: q <= floor((n + kp) / l)
        ll q_cn = (n + (lll)k * p) / l;
        if (q_cn < q_max) q_max = q_cn;

        if (q_min > q_max) {
            // Check if we should stop: q_max decreases relative to q_min as p grows
            // when the n constraint dominates
            if (q_cn < q_bc) break;
            continue;
        }

        count += q_max - q_min + 1;
    }
    return count;
}

// Case 2: exactly one of k, l even - p, q both even
// Java constraints:
//   Constraint(0, -1, -(l+1)/2, false)  => q' >= ceil(l/2) where q = 2q'
//   Constraint(3k+l, -(k+l), 0, false)  => (3k+l)p' <= (k+l)q' (same constraint scaled)
//   Constraint(-k, l, n/2, false)       => lq' - kp' <= n/2
//   Constraint(-l, k, 0, false)         => kq' <= lp'
ll count_pq_case2(ll k, ll l, ll n) {
    ll l_half = (l + 1) / 2;  // ceiling
    ll n_half = n / 2;
    ll count = 0;

    for (ll p = 1; ; p++) {
        ll q_min = l_half;
        if (p >= q_min) q_min = p + 1;

        ll q_bc = ((lll)(3*k + l) * p + (k + l) - 1) / (k + l);
        if (q_bc > q_min) q_min = q_bc;

        ll q_max = (lll)l * p / k;
        ll q_cn = (n_half + (lll)k * p) / l;
        if (q_cn < q_max) q_max = q_cn;

        if (q_min > q_max) {
            if (q_cn < q_bc) break;
            continue;
        }

        count += q_max - q_min + 1;
    }
    return count;
}

// Case 3: k, l both odd - p, q same parity via substitution p = u-v, q = u+v
// Java constraints (in u, v space):
//   Constraint(-1, -1, -l, false)       => -u - v <= -l => u + v >= l
//   Constraint(2k, -(4k+2l), 0, false)  => 2ku - (4k+2l)v <= 0 => ku <= (2k+l)v
//   Constraint(l-k, k+l, n, false)      => (l-k)u + (k+l)v <= n
//   Constraint(k-l, k+l, 0, false)      => (k-l)u + (k+l)v <= 0 => (l-k)u >= (k+l)v
// Also we need u > v (so q > p) and u >= 1, v >= 1
ll count_pq_case3(ll k, ll l, ll n) {
    ll count = 0;

    // Iterate over v, count valid u
    for (ll v = 1; ; v++) {
        // u > v => u >= v + 1
        ll u_min = v + 1;

        // u + v >= l => u >= l - v
        if (l - v > u_min) u_min = l - v;

        // (l-k)u >= (k+l)v => u >= ceil((k+l)v / (l-k))
        ll u_from_y = ((lll)(k + l) * v + (l - k) - 1) / (l - k);
        if (u_from_y > u_min) u_min = u_from_y;

        // ku <= (2k+l)v => u <= floor((2k+l)v / k)
        ll u_max = (lll)(2*k + l) * v / k;

        // (l-k)u + (k+l)v <= n => u <= floor((n - (k+l)v) / (l-k))
        ll num = n - (lll)(k + l) * v;
        if (num < 0) break;
        ll u_cn = num / (l - k);
        if (u_cn < u_max) u_max = u_cn;

        if (u_min <= u_max) {
            count += u_max - u_min + 1;
        }
    }
    return count;
}

// Second loop: count (k, l) with l > L for given (p, q)
// Java constraints for case 1 (p, q both even or no parity check):
//   Constraint(0, -1, -q, true)         => -l < -q => l > q (STRICT)
//   Constraint(q, -p, 0, false)         => qk - pl <= 0 => qk <= pl
//   Constraint(3p-q, -(q-p), 0, false)  => (3p-q)k - (q-p)l <= 0
//   Constraint(-p, q, n, false)         => -pk + ql <= n => ql - pk <= n
// But wait - there's an issue: the Java second loop iterates l > q, but we want l > L
// Looking at Java: the second loop counts with l > q constraint, but we partition by l <= L vs l > L
// So the second loop should count l > L (not l > q)

// Actually re-reading Java: in second loop, it counts with constraint l > q
// But the split is: first loop has l <= L, second loop has q <= L
// The second loop uses l > q as constraint, which ensures no overlap with first loop
// (since first loop has q >= l)

ll count_kl_case1(ll p, ll q, ll n, ll L) {
    // Count (k, l) with k >= 1, l > q (strict, since first loop has q >= l)
    // Additional: l > L (from our split)
    ll l_base = (q > L) ? q : L;  // l must be > max(q, L)

    ll count = 0;
    for (ll k = 1; ; k++) {
        // l > l_base => l >= l_base + 1
        ll l_min = l_base + 1;
        // l > k => l >= k + 1
        if (k + 1 > l_min) l_min = k + 1;

        // qk <= pl => l >= ceil(qk / p)
        ll l_from_y = ((lll)q * k + p - 1) / p;
        if (l_from_y > l_min) l_min = l_from_y;

        // (3p-q)k <= (q-p)l when 3p > q
        if (3 * p > q) {
            ll l_from_bc = ((lll)(3*p - q) * k + (q - p) - 1) / (q - p);
            if (l_from_bc > l_min) l_min = l_from_bc;
        }

        // ql - pk <= n => l <= floor((n + pk) / q)
        ll l_max = (n + (lll)p * k) / q;

        if (l_min > l_max) break;

        count += l_max - l_min + 1;
    }
    return count;
}

// Case 2: exactly one of p, q even - k, l both even
// k = 2k', l = 2l'
// Java constraints (scaled by 2):
//   Constraint(0, -1, -q/2, true)   => l' > q/2 (strict)
//   others scale similarly with n -> n/2
ll count_kl_case2(ll p, ll q, ll n, ll L) {
    ll q_half = q / 2;  // floor
    ll L_half = L / 2;
    ll l_base = (q_half > L_half) ? q_half : L_half;
    ll n_half = n / 2;

    ll count = 0;
    for (ll k = 1; ; k++) {
        ll l_min = l_base + 1;
        if (k + 1 > l_min) l_min = k + 1;

        ll l_from_y = ((lll)q * k + p - 1) / p;
        if (l_from_y > l_min) l_min = l_from_y;

        if (3 * p > q) {
            ll l_from_bc = ((lll)(3*p - q) * k + (q - p) - 1) / (q - p);
            if (l_from_bc > l_min) l_min = l_from_bc;
        }

        ll l_max = (n_half + (lll)p * k) / q;

        if (l_min > l_max) break;

        count += l_max - l_min + 1;
    }
    return count;
}

// Case 3: p, q both odd - k, l same parity via substitution k = a-b, l = a+b
// Java constraints (in a, b space where k = a-b, l = a+b):
//   Constraint(-1, 1, 0, true)          => -a + b > 0 => b > a (strict), implies l > k
//   Constraint(-1, -1, -q, true)        => -a - b < -q => a + b > q => l > q (strict)
//   Constraint(q-p, -(p+q), 0, false)   => (q-p)a - (p+q)b <= 0
//   Constraint(4p-2q, -2p, 0, false)    => (4p-2q)a - 2pb <= 0 => (2p-q)a <= pb when 2p > q
//   Constraint(q-p, p+q, n, false)      => (q-p)a + (p+q)b <= n
ll count_kl_case3(ll p, ll q, ll n, ll L) {
    ll count = 0;

    // Both k, l even: k = 2k', l = 2l'
    // l' > q/2 and l' > L/2
    count += count_kl_case2(p, q, n, L);  // reuses case2 logic

    // Both k, l odd: k = 2a+1, l = 2b+1 (actually a = (l-k)/2, b = (l+k)/2 - no)
    // Actually the substitution is a for one variable, b for another
    // Let me re-derive: if k = a - b, l = a + b, then a = (k+l)/2, b = (l-k)/2
    // For k, l both odd: k + l even, l - k even, so a, b are integers
    // But we want k = 2a'+1, l = 2b'+1 instead...

    // Let's use: k = 2a+1, l = 2b+1 where b > a >= 0 (so l > k)
    // Constraints:
    //   l > q => 2b+1 > q => b >= q/2 (ceiling)
    //   l > L => 2b+1 > L => b >= L/2 (ceiling)
    //   qk <= pl => q(2a+1) <= p(2b+1) => 2qa + q <= 2pb + p => pb - qa >= (q-p)/2
    //     b >= (2qa + q - p) / (2p)  (ceiling if positive)
    //   (3p-q)k <= (q-p)l when 3p > q
    //     (3p-q)(2a+1) <= (q-p)(2b+1)
    //     2(3p-q)a + (3p-q) <= 2(q-p)b + (q-p)
    //     (q-p)b >= (3p-q)a + (3p-q)/2 - (q-p)/2 = (3p-q)a + (4p-2q)/2 = (3p-q)a + 2p - q
    //     b >= ((3p-q)a + 2p - q) / (q - p)  (ceiling if positive)
    //   ql - pk <= n => q(2b+1) - p(2a+1) <= n => 2qb + q - 2pa - p <= n
    //     b <= (2pa + n - q + p) / (2q)

    ll b_min_q = (q + 1) / 2;  // b > (q-1)/2, so b >= ceil(q/2)
    ll b_min_L = (L + 1) / 2;  // b > (L-1)/2
    ll b_base = (b_min_q > b_min_L) ? b_min_q : b_min_L;

    ll n_adj = n - q + p;
    if (n_adj < 0) return count;

    for (ll a = 0; ; a++) {
        ll b_min = b_base;
        if (a + 1 > b_min) b_min = a + 1;  // b > a

        // b >= ceil((2qa + q - p) / (2p))
        ll b_from_y_num = 2 * (lll)q * a + q - p;
        if (b_from_y_num > 0) {
            ll b_from_y = (b_from_y_num + 2*p - 1) / (2*p);
            if (b_from_y > b_min) b_min = b_from_y;
        }

        // b >= ceil(((3p-q)a + 2p - q) / (q - p)) when 3p > q
        if (3 * p > q) {
            ll b_from_bc_num = (lll)(3*p - q) * a + 2*p - q;
            if (b_from_bc_num > 0) {
                ll b_from_bc = (b_from_bc_num + (q - p) - 1) / (q - p);
                if (b_from_bc > b_min) b_min = b_from_bc;
            }
        }

        // b <= floor((2pa + n - q + p) / (2q))
        ll b_max_num = 2 * (lll)p * a + n_adj;
        if (b_max_num < 0) break;
        ll b_max = b_max_num / (2 * q);

        if (b_min <= b_max) {
            count += b_max - b_min + 1;
        }

        if (b_max < b_base) break;
    }

    return count;
}

ll f(ll n, int checkParity) {
    ll L = (ll)sqrtl(1.5 * n);
    ll result = 0;

    // First loop: iterate over (k, l) with l <= L
    // Count (p, q) with q >= l (and q > p, etc.)
    for (ll l = 1; l <= L; l++) {
        for (ll k = 1; k < l; k++) {
            if (!checkParity || ((k % 2 == 0) && (l % 2 == 0))) {
                result += count_pq_case1(k, l, n);
            } else if ((k % 2 == 0) || (l % 2 == 0)) {
                result += count_pq_case2(k, l, n);
            } else {
                result += count_pq_case3(k, l, n);
            }
        }
    }

    // Second loop: iterate over (p, q) with q <= L
    // Count (k, l) with l > q (to avoid overlap with first loop where q >= l)
    // But we only want l > L here (for the partition), so l > max(q, L)
    for (ll q = 1; q <= L; q++) {
        for (ll p = 1; p < q; p++) {
            if (!checkParity || ((p % 2 == 0) && (q % 2 == 0))) {
                result += count_kl_case1(p, q, n, L);
            } else if ((p % 2 == 0) || (q % 2 == 0)) {
                result += count_kl_case2(p, q, n, L);
            } else {
                result += count_kl_case3(p, q, n, L);
            }
        }
    }

    return result;
}

int main() {
    compute_mobius(N);

    ll ans = 0;
    for (int g = 1; g <= N; g++) {
        if (mobius[g] != 0) {
            ans += mobius[g] * f(N / g, g % 2 == 1);
        }
    }

    printf("%lld\n", ans);

    free(mobius);
    return 0;
}
'''

    with tempfile.NamedTemporaryFile(suffix='.c', delete=False) as f:
        f.write(c_code.encode())
        c_file = f.name
    exe = c_file[:-2]
    try:
        subprocess.run(['gcc', '-O3', '-march=native', '-o', exe, c_file, '-lm'], check=True, capture_output=True)
        result = subprocess.check_output([exe], timeout=300).decode().strip()
        print(result)
    finally:
        os.unlink(c_file)
        if os.path.exists(exe):
            os.unlink(exe)


if __name__ == "__main__":
    solve()
