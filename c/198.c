/* Project Euler 198: Ambiguous Numbers.
   Stern-Brocot tree enumeration of fractions p/q < 1/100 with
   2*k1*k2 <= LIMIT/2. */
#include <stdio.h>

#define LIMIT_Q 100000000LL
#define HALF_Q  (LIMIT_Q / 2)

static long long isqrt_ll(long long n) {
    long long r = (long long)__builtin_sqrt((double)n);
    while (r * r > n) r--;
    while ((r + 1) * (r + 1) <= n) r++;
    return r;
}

static long long SQRT_HALF_Q;

/* Stack-based DFS */
#define MAX_STACK 10000000
static long long stack_hl[MAX_STACK], stack_kl[MAX_STACK];
static long long stack_hr[MAX_STACK], stack_kr[MAX_STACK];

static long long count_between(long long hl0, long long kl0, long long hr0, long long kr0) {
    int sp = 0;
    stack_hl[sp] = hl0; stack_kl[sp] = kl0;
    stack_hr[sp] = hr0; stack_kr[sp] = kr0;
    sp++;

    long long total = 0;

    while (sp > 0) {
        sp--;
        long long hl = stack_hl[sp], kl = stack_kl[sp];
        long long hr = stack_hr[sp], kr = stack_kr[sp];

        int initial_branch = (hl == 0 && kl == 1);

        while (1) {
            long long hm = hl + hr;
            long long km = kl + kr;

            if (km > HALF_Q) break;
            if (100 * hm >= km) break;

            long long max_partner = HALF_Q / km;

            if (!(hl == 0 && kl == 1)) {
                if (kl <= max_partner) total++;
            }

            if (kr <= max_partner) total++;

            int left_blocked = kl > max_partner;
            int right_blocked = kr > max_partner;

            if (!right_blocked) {
                if (sp < MAX_STACK) {
                    stack_hl[sp] = hm; stack_kl[sp] = km;
                    stack_hr[sp] = hr; stack_kr[sp] = kr;
                    sp++;
                }
            }

            if (right_blocked && initial_branch) break;
            if (left_blocked) break;

            hr = hm;
            kr = km;
        }
    }

    return total;
}

int main(void) {
    SQRT_HALF_Q = isqrt_ll(HALF_Q);

    long long case1 = HALF_Q - 50; /* k_1 = 51 ... 50_000_000 */
    long long case2 = count_between(0, 1, 1, 100);
    printf("%lld\n", case1 + case2);
    return 0;
}
