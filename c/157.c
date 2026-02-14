/*
 * Project Euler 157 - Solving the diophantine equation 1/a + 1/b = p/10^n
 *
 * Count the number of solutions for n = 1..9.
 */
#include <stdio.h>

typedef long long ll;

static int num_divisors(ll num) {
    if (num <= 1) return (int)num;
    int result = 1;
    for (ll i = 2; i * i <= num; i++) {
        if (num % i == 0) {
            int exp = 0;
            while (num % i == 0) { exp++; num /= i; }
            result *= (exp + 1);
        }
    }
    if (num > 1) result *= 2;
    return result;
}

int main(void) {
    ll total = 0;

    for (int n = 1; n <= 9; n++) {
        int count = 0;

        /* Precompute powers */
        ll pow2[10], pow5[10];
        pow2[0] = 1; pow5[0] = 1;
        for (int i = 1; i <= n; i++) {
            pow2[i] = pow2[i-1] * 2;
            pow5[i] = pow5[i-1] * 5;
        }

        /* Case 1: m=1, k = 2^a * 5^b */
        for (int a = 0; a <= n; a++) {
            for (int b = 0; b <= n; b++) {
                ll k = pow2[a] * pow5[b];
                ll s = pow2[n - a] * pow5[n - b];
                ll mk_sum = 1 + k;
                ll s_val = s * mk_sum;
                count += num_divisors(s_val);
            }
        }

        /* Case 2: m = 2^alpha, k = 5^beta, m <= k */
        for (int alpha = 1; alpha <= n; alpha++) {
            for (int beta = 1; beta <= n; beta++) {
                ll m = pow2[alpha];
                ll k = pow5[beta];
                if (m > k) continue;
                ll s = pow2[n - alpha] * pow5[n - beta];
                ll mk_sum = m + k;
                ll s_val = s * mk_sum;
                count += num_divisors(s_val);
            }
        }

        /* Case 3: m = 5^beta, k = 2^alpha, m <= k */
        for (int beta = 1; beta <= n; beta++) {
            for (int alpha = 1; alpha <= n; alpha++) {
                ll m = pow5[beta];
                ll k = pow2[alpha];
                if (m > k) continue;
                ll s = pow2[n - alpha] * pow5[n - beta];
                ll mk_sum = m + k;
                ll s_val = s * mk_sum;
                count += num_divisors(s_val);
            }
        }

        total += count;
    }

    printf("%lld\n", total);
    return 0;
}
