/*
 * Project Euler 794 - Seventeen Points
 *
 * Choose x_1,...,x_n in [0,1) sequentially such that after step k,
 * each interval [(j-1)/k, j/k) for j=1..k contains exactly one point.
 * F(n) = min sum of x_i. Find F(17) to 12 decimal places.
 *
 * Algorithm: DFS with interval tracking using rational arithmetic
 * (denominator = LCM(1..17) = 12252240).
 *
 * At each step k, each of the k-1 existing points is assigned to a cell
 * of the k-partition. Most points have only 1 possible cell (their interval
 * fits entirely within one cell). Points straddling a cell boundary have 2
 * choices. The new point gets the remaining cell.
 */
#include <stdio.h>
#include <string.h>
#include <stdlib.h>

#define MAXN 18
#define LCM 12252240LL

typedef long long ll;

static int n_target = 17;
static double best_sum;

/* Track intervals per DFS level to avoid global state issues */
typedef struct {
    ll lo[MAXN];
    ll hi[MAXN];
} State;

/* At a given step with given state, determine cell assignments and recurse.
 * ambig[] = indices of points with 2 possible cells
 * fixed_cell[p] = cell for point p if it has 1 choice, or first choice if 2
 */

static void solve(int step, State *st);

static void try_assignment(int step, State *st, int *ambig, int nambig,
                           int *cell_of, int ai, int *cell_used) {
    if (ai == nambig) {
        /* All ambiguous points assigned. Find cell for new point. */
        int free_cell = -1;
        for (int c = 0; c < step; c++) {
            if (!cell_used[c]) { free_cell = c; break; }
        }
        if (free_cell < 0) return;  /* shouldn't happen */

        /* Create new state */
        State ns;
        ll unit = LCM / step;

        for (int p = 0; p < step - 1; p++) {
            int c = cell_of[p];
            ll cl = (ll)c * unit;
            ll ch = (ll)(c + 1) * unit;
            ns.lo[p] = st->lo[p] > cl ? st->lo[p] : cl;
            ns.hi[p] = st->hi[p] < ch ? st->hi[p] : ch;
            if (ns.lo[p] >= ns.hi[p]) return;
        }

        /* New point gets free_cell */
        ll cl = (ll)free_cell * unit;
        ll ch = (ll)(free_cell + 1) * unit;
        ns.lo[step - 1] = cl;
        ns.hi[step - 1] = ch;

        /* Pruning: current lower bound on sum */
        double cur_lo_sum = 0;
        for (int i = 0; i < step; i++)
            cur_lo_sum += (double)ns.lo[i];
        if (cur_lo_sum / (double)LCM >= best_sum)
            return;

        if (step == n_target) {
            double s = cur_lo_sum / (double)LCM;
            if (s < best_sum)
                best_sum = s;
            return;
        }

        solve(step + 1, &ns);
        return;
    }

    int p = ambig[ai];
    /* Try both cells for this ambiguous point */
    for (int opt = 0; opt < 2; opt++) {
        int c;
        ll unit = LCM / step;
        if (opt == 0) {
            /* Lower cell */
            c = (int)(st->lo[p] / unit);
        } else {
            /* Upper cell */
            c = (int)((st->hi[p] - 1) / unit);
        }
        if (c < 0 || c >= step) continue;
        if (cell_used[c]) continue;

        /* Check overlap */
        ll cl = (ll)c * unit;
        ll ch = (ll)(c + 1) * unit;
        ll ol = st->lo[p] > cl ? st->lo[p] : cl;
        ll oh = st->hi[p] < ch ? st->hi[p] : ch;
        if (ol >= oh) continue;

        cell_of[p] = c;
        cell_used[c] = 1;

        try_assignment(step, st, ambig, nambig, cell_of, ai + 1, cell_used);

        cell_used[c] = 0;
    }
}

static void solve(int step, State *st) {
    ll unit = LCM / step;

    int cell_of[MAXN];
    int cell_used[MAXN];
    memset(cell_used, 0, sizeof(cell_used));

    int ambig[MAXN];
    int nambig = 0;

    /* Classify existing points */
    for (int p = 0; p < step - 1; p++) {
        int c_lo = (int)(st->lo[p] / unit);
        int c_hi = (int)((st->hi[p] - 1) / unit);
        if (c_lo < 0) c_lo = 0;
        if (c_hi >= step) c_hi = step - 1;

        if (c_lo == c_hi) {
            /* Unique cell */
            cell_of[p] = c_lo;
            if (cell_used[c_lo]) return;  /* conflict */
            cell_used[c_lo] = 1;
        } else {
            /* Ambiguous - straddles boundary */
            ambig[nambig++] = p;
            cell_of[p] = -1;
        }
    }

    /* Try all combinations for ambiguous points */
    try_assignment(step, st, ambig, nambig, cell_of, 0, cell_used);
}

int main(void) {
    best_sum = 1e18;

    State s0;
    s0.lo[0] = 0;
    s0.hi[0] = LCM;

    solve(2, &s0);

    printf("%.12f\n", best_sum);
    return 0;
}
