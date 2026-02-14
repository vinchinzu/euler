/* Project Euler 066 - Diophantine equation (Pell's equation)
 * Uses big integer arithmetic since x can be very large for some D values */
#include <stdio.h>
#include <string.h>
#include <math.h>

#define MAXDIG 300

typedef struct {
    int d[MAXDIG]; /* digits, little-endian */
    int len;
} BigNum;

static void bn_from_int(BigNum *b, int val) {
    b->len = 0;
    if (val == 0) {
        b->d[0] = 0;
        b->len = 1;
        return;
    }
    while (val > 0) {
        b->d[b->len++] = val % 10;
        val /= 10;
    }
}

/* result = a * scalar + b */
static void bn_mul_scalar_add(BigNum *result, const BigNum *a, int scalar, const BigNum *b) {
    int tmp[MAXDIG + 10];
    int tlen = 0;
    int carry = 0;
    int maxlen = a->len > b->len ? a->len : b->len;

    for (int i = 0; i < a->len || i < b->len || carry; i++) {
        long long val = (long long)carry;
        if (i < a->len) val += (long long)a->d[i] * scalar;
        if (i < b->len) val += b->d[i];
        tmp[tlen++] = (int)(val % 10);
        carry = (int)(val / 10);
    }
    result->len = tlen;
    memcpy(result->d, tmp, tlen * sizeof(int));
}

/* Compare: returns >0 if a>b, 0 if equal, <0 if a<b */
static int bn_cmp(const BigNum *a, const BigNum *b) {
    if (a->len != b->len) return a->len - b->len;
    for (int i = a->len - 1; i >= 0; i--) {
        if (a->d[i] != b->d[i]) return a->d[i] - b->d[i];
    }
    return 0;
}

int main(void) {
    BigNum max_x;
    bn_from_int(&max_x, 0);
    int result_d = 0;

    for (int D = 2; D <= 1000; D++) {
        int sqrt_d = (int)sqrt((double)D);
        if (sqrt_d * sqrt_d == D) continue;

        int a0 = sqrt_d;
        int m = 0, dd = 1, a = a0;

        BigNum p_km2, p_km1, q_km2, q_km1;
        bn_from_int(&p_km2, 0);
        bn_from_int(&p_km1, 1);
        bn_from_int(&q_km2, 1);
        bn_from_int(&q_km1, 0);

        while (1) {
            BigNum cp, cq;
            bn_mul_scalar_add(&cp, &p_km1, a, &p_km2);
            bn_mul_scalar_add(&cq, &q_km1, a, &q_km2);

            /* Check if cp^2 - D * cq^2 == 1 */
            /* Compute cp^2 and D*cq^2 using big integer multiplication */
            /* Since we follow the CF expansion, we know the fundamental solution
             * occurs when a == 2*a0 at the end of a period (for even period)
             * or we just check each convergent. Use the property that for
             * the CF of sqrt(D), convergent p/q satisfies p^2 - D*q^2 = (-1)^k
             * at period boundaries. But simpler: just check periodically. */

            /* For Pell's equation via CF: the solution is the convergent where
             * the period completes with a == 2*a0. We track p^2 - D*q^2 via
             * the recurrence sign. Actually let's just use the sign tracking:
             * h_{-1}=1, h_0=a_0, h_n = a_n*h_{n-1} + h_{n-2}
             * k_{-1}=0, k_0=1, k_n = a_n*k_{n-1} + k_{n-2}
             * and h_n^2 - D*k_n^2 = (-1)^(n+1) * (d_n) ... too complex.
             *
             * The standard approach: convergent at end of period gives solution.
             * If period is odd, need two periods. */

            /* Simpler: compute next CF term, and at each step check if a == 2*a0 */
            int m_next = dd * a - m;
            int d_next = (D - m_next * m_next) / dd;
            int a_next = (a0 + m_next) / d_next;

            m = m_next;
            dd = d_next;

            p_km2 = p_km1;
            p_km1 = cp;
            q_km2 = q_km1;
            q_km1 = cq;

            if (a == 2 * a0) {
                /* cp is numerator of convergent at end of period.
                 * For even period, this is the solution.
                 * For odd period, we need to continue one more period. */
                /* Actually, the Pell solution from CF: the fundamental solution
                 * x = p_k where k is the period length - 1.
                 * If period is odd, we need p_{2*period - 1}. */
                /* Let's just check: cp^2 - D * cq^2 == 1?
                 * We can check using big integers, but it's simpler to use
                 * the fact that for sqrt(D), the sign alternates.
                 * h_n^2 - D*k_n^2 = (-1)^(n+1)
                 * So the solution is found when n+1 is even, i.e., n is odd.
                 * The period boundary is when a == 2*a0. */
                /* Actually, let me just continue the CF until we find x^2 - Dy^2 = 1.
                 * For the first period end, if the sign is +1, we have the solution.
                 * Otherwise, continue for another period. */
                break; /* We'll check below */
            }
            a = a_next;
        }

        /* The CF period of sqrt(D) has length L.
         * p_{L-1} gives p^2 - D*q^2 = (-1)^L
         * If L is even, p_{L-1} is the solution.
         * If L is odd, p_{2L-1} is needed. */

        /* Determine the period length */
        int period = 0;
        {
            int m2 = 0, d2 = 1, a2 = a0;
            do {
                m2 = d2 * a2 - m2;
                d2 = (D - m2 * m2) / d2;
                a2 = (a0 + m2) / d2;
                period++;
            } while (a2 != 2 * a0);
        }

        /* Recompute convergents properly */
        BigNum pk2, pk1, qk2, qk1;
        bn_from_int(&pk2, 0);
        bn_from_int(&pk1, 1);
        bn_from_int(&qk2, 1);
        bn_from_int(&qk1, 0);

        int reps = (period % 2 == 0) ? 1 : 2;
        int m3 = 0, d3 = 1, a3 = a0;

        for (int r = 0; r < reps; r++) {
            /* Process the initial a0 term (only first time) or period terms */
            if (r == 0) {
                /* Process a0 */
                BigNum cp, cq;
                bn_mul_scalar_add(&cp, &pk1, a0, &pk2);
                bn_mul_scalar_add(&cq, &qk1, a0, &qk2);
                pk2 = pk1; pk1 = cp;
                qk2 = qk1; qk1 = cq;
            }
            /* Process one full period */
            m3 = 0; d3 = 1; a3 = a0;
            for (int i = 0; i < period; i++) {
                m3 = d3 * a3 - m3;
                d3 = (D - m3 * m3) / d3;
                a3 = (a0 + m3) / d3;

                BigNum cp, cq;
                bn_mul_scalar_add(&cp, &pk1, a3, &pk2);
                bn_mul_scalar_add(&cq, &qk1, a3, &qk2);
                pk2 = pk1; pk1 = cp;
                qk2 = qk1; qk1 = cq;
            }
        }

        /* pk1 is x, qk1 is y for x^2 - D*y^2 = 1 */
        if (bn_cmp(&pk1, &max_x) > 0) {
            max_x = pk1;
            result_d = D;
        }
    }

    printf("%d\n", result_d);
    return 0;
}
