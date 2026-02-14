/*
 * Project Euler Problem 577: Counting Hexagons
 *
 * Find sum H(n) for n=3..12345 where H(n) = number of regular hexagons
 * with all vertices on triangular grid.
 *
 * H(n) = sum_{r=1}^{n-2} sum_{s=1}^{floor((n-r)/2)} (n-r-2s+1) * (n-2r-s+1) * min(r,s)
 * but we must have n-2r-s+1 > 0 and n-r-2s+1 > 0.
 * Actually, direct formula: H(n) = sum_{a=1}^{floor(n/3)} a * (n-3a+1) * (n-3a+2) / 2
 * Wait, that's for a specific shape. Let me use the known closed-form sum approach.
 *
 * For the hexagon count on a triangular grid of side n:
 * H(n) = sum_{a+2b <= n} a * (n - a - 2b + 1) * min(a, b) -- this doesn't simplify easily.
 *
 * Actually the known formula is:
 * H(n) = sum_{r=1}^{floor(n/3)} (n - 3r + 1) * r * (n - 3r + 2) / 2 doesn't capture all hexagons.
 *
 * Let's use the direct computation from the Eisenstein integer approach, but
 * with the polynomial extrapolation trick since sum H(n) is a polynomial in N.
 *
 * The sum S(N) = sum_{n=3}^{N} H(n) is a degree-5 polynomial in N.
 * Compute S(n) for n = 3..8, then use Lagrange interpolation.
 */
#include <stdio.h>
#include <stdlib.h>

typedef long long ll;

/* Compute H(n) directly: count hexagons on triangular grid of side n.
 * Vertex (a,b) is valid if 0 <= b <= a <= n.
 * Hexagon has center (ca, cb) and radius vector (ra, rb) with 0 < ra, 0 <= rb < ra.
 * The 6 vertices are center + radius * omega^k for k=0..5 where omega rotates 60 deg.
 * In Eisenstein coords, multiplying by (1+omega) = (1,1) rotates by 60 degrees.
 * Rotation: (a,b) -> (a-b, a) [multiply by omega = (-1+omega) -> actually let me be precise]
 * If we represent omega as the primitive cube root rotation:
 * (ra, rb) * (1, 1) = (ra - rb, ra) -- this is multiplication by (1 + omega).
 * Wait, (a+b*omega)(c+d*omega) = ac - bd + (ad + bc - bd)*omega
 * So (ra, rb) * (1, 1) = (ra*1 - rb*1, rb*1 + ra*1 - rb*1) = (ra - rb, ra)
 */

static int check_vertex(int a, int b, int n) {
    return b >= 0 && a >= b && a <= n;
}

static ll H(int n) {
    ll count = 0;
    for (int ca = 0; ca <= n; ca++) {
        for (int cb = 0; cb <= ca; cb++) {
            for (int ra = 1; ra <= n; ra++) {
                for (int rb = 0; rb < ra; rb++) {
                    /* Check all 6 vertices */
                    int va = ra, vb = rb;
                    int ok = 1;
                    for (int k = 0; k < 6; k++) {
                        int px = ca + va;
                        int py = cb + vb;
                        if (!check_vertex(px, py, n)) { ok = 0; break; }
                        /* Rotate: (va, vb) -> (va - vb, va) [multiply by (1,1)] */
                        int nva = va - vb;
                        int nvb = va;
                        va = nva;
                        vb = nvb;
                    }
                    if (ok) count++;
                }
            }
        }
    }
    return count;
}

/* Use Lagrange interpolation to extrapolate.
 * S(N) = sum_{n=3}^{N} H(n) is polynomial of degree d in N.
 * We need d+1 sample points. H(n) should be degree 4 in n, so S(N) is degree 5.
 * We need 7 points for degree 5 (actually 6+1=7 to be safe). */
#define NUM_POINTS 8

int main(void) {
    ll N = 12345;

    /* Compute S(n) = sum_{k=3}^{n} H(k) for n = 3, 4, ..., 3+NUM_POINTS-1 */
    ll S[NUM_POINTS];
    ll cumsum = 0;
    for (int i = 0; i < NUM_POINTS; i++) {
        int n = 3 + i;
        cumsum += H(n);
        S[i] = cumsum;
    }

    /* Lagrange interpolation at x = N using points (3, S[0]), (4, S[1]), ..., (3+NUM_POINTS-1, S[NUM_POINTS-1]) */
    /* Since the values can be large, use __int128 for intermediate calculations */
    /* But the x-values are small, so we can use the standard approach with rational arithmetic */

    /* Use the fact that for a polynomial of degree <= NUM_POINTS-1,
       P(N) = sum_{i=0}^{NUM_POINTS-1} S[i] * prod_{j!=i} (N - x_j) / (x_i - x_j) */

    /* Compute using exact integer arithmetic with common denominator */
    typedef __int128 i128;
    i128 numerator = 0;
    i128 denominator = 1;

    /* First compute common denominator = product of all (x_i - x_j) for i != j ... no, that's different per term.
       Better: use incremental approach. */

    /* Let x_i = 3 + i for i = 0..NUM_POINTS-1 */
    /* L_i(N) = prod_{j!=i} (N - x_j) / (x_i - x_j) */

    /* Compute each L_i as a fraction */
    /* Since x_i - x_j = i - j for our equally spaced points, the denominator of L_i is
       prod_{j!=i} (i - j) = i! * (NUM_POINTS-1-i)! * (-1)^(NUM_POINTS-1-i) */

    /* For equally spaced points with step 1:
       P(N) = sum_{i=0}^{m} (-1)^{m-i} * C(m,i) * S[i] * prod_{j=0}^{m} (N-x_j) / (N - x_i) / m!
       where m = NUM_POINTS - 1.
       Actually it's simpler to just compute directly. */

    /* Direct computation with double intermediate isn't precise enough.
       Use __int128 with careful factoring. */

    /* numerator_parts[i] = S[i] * prod_{j!=i} (N - x_j) */
    /* denom_parts[i] = prod_{j!=i} (x_i - x_j) */
    /* result = sum_i numerator_parts[i] / denom_parts[i] */

    /* For equally spaced x with step 1:
       denom_parts[i] = prod_{j!=i} (i - j) */

    int m = NUM_POINTS - 1;
    i128 result_num = 0;
    /* Common denominator for equally spaced points: factorial(m) * (-1)^? */
    /* Actually compute each term and sum as fractions */

    /* Let's just compute prod_{j!=i} (N - x_j) and prod_{j!=i} (x_i - x_j) for each i */
    for (int i = 0; i <= m; i++) {
        i128 num_prod = 1;
        i128 den_prod = 1;
        for (int j = 0; j <= m; j++) {
            if (j == i) continue;
            num_prod *= (N - (3 + j));
            den_prod *= (i - j);
        }
        result_num += (i128)S[i] * num_prod / den_prod;
    }

    /* result_num should be exact if polynomial degree <= m */
    ll result = (ll)result_num;
    printf("%lld\n", result);
    return 0;
}
