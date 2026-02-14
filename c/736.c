/* Project Euler 736: Paths to Equality.
 * Iterative deepening DFS with pruning.
 * r(x,y) = (x+1, 2y), s(x,y) = (2x, y+1)
 * Find shortest sequence to reach x == y starting from (45, 90).
 */
#include <stdio.h>
#include <stdint.h>

typedef long long ll;
typedef __int128 lll;

static ll ans = 0;

/* Check feasibility: can we reach x==y in exactly 'depth' more steps,
 * given t remaining 'r' operations? */
static int feasible(ll x, ll y, int depth, int t) {
    /* After t more r-ops and (depth-t) more s-ops:
     * x_final = x * 2^t + something, y_final = y * 2^(depth-t) + something
     * Bounds:
     *   x * 2^t + (depth-t) <= (y + t) * 2^(depth-t)
     *   y * 2^(depth-t) + t <= (x + (depth-t)) * 2^t
     */
    /* Use __int128 to avoid overflow since 2^depth can be large */
    lll pw_t = 1;
    for (int i = 0; i < t; i++) pw_t <<= 1;  /* 2^t */
    lll pw_dt = 1;
    for (int i = 0; i < depth - t; i++) pw_dt <<= 1;  /* 2^(depth-t) */

    lll lhs1 = (lll)x * pw_t + (depth - t);
    lll rhs1 = ((lll)y + t) * pw_dt;
    if (lhs1 > rhs1) return 0;

    lll lhs2 = (lll)y * pw_dt + t;
    lll rhs2 = ((lll)x + (depth - t)) * pw_t;
    if (lhs2 > rhs2) return 0;

    return 1;
}

static void search(ll x, ll y, int depth) {
    if (ans > 0) return;
    if (depth == 0) {
        if (x == y) ans = x;
        return;
    }

    /* Check if any t value is feasible before branching */
    int any_feasible = 0;
    for (int t = 0; t <= depth; t++) {
        if (feasible(x, y, depth, t)) {
            any_feasible = 1;
            break;
        }
    }
    if (!any_feasible) return;

    /* s-operation: (2x, y+1) */
    search(2 * x, y + 1, depth - 1);
    if (ans > 0) return;
    /* r-operation: (x+1, 2y) */
    search(x + 1, 2 * y, depth - 1);
}

int main() {
    ll a = 45, b = 90;

    for (int max_depth = 2; ; max_depth += 2) {
        search(a, b, max_depth);
        if (ans > 0) break;
    }

    printf("%lld\n", ans);
    return 0;
}
