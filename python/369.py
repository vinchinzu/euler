"""Project Euler Problem 369: Badugi.

Count n-card hands (n=4..13) from a 52-card deck that contain a Badugi
(4 cards with all different ranks and all different suits).

Uses DP on ranks with Hall's theorem satisfaction bitmask.
For each rank, choose which suits get a card. Track which subsets S of suits
have |N(S)| >= |S| (Hall's condition for perfect matching).
Implemented in C for speed.
"""
import subprocess, os, tempfile

def solve():
    c_code = r"""
#include <stdio.h>
#include <string.h>
#include <stdlib.h>

/*
 * Badugi counting using DP on ranks.
 *
 * 4 suits, 13 ranks, 52 cards. A Badugi = 4 cards, distinct ranks, distinct suits.
 * f(n) = C(52,n) - g(n) where g(n) = hands with no Badugi.
 * A hand has a Badugi iff its suit-rank bipartite graph has a perfect matching.
 * By Hall's theorem: perfect matching exists iff for all S subset {0,1,2,3},
 * |N(S)| >= |S| where N(S) = ranks that have at least one card in some suit in S.
 *
 * DP: process ranks 0..12. For each rank, choose subset T of suits to include.
 * State: for each of 15 nonempty subsets of {0,1,2,3}, track |N(S)|.
 * Since |N(S)| can only increase, and we only need |N(S)| >= |S|, we track
 * a bitmask of which subsets are "satisfied".
 *
 * There are 2^15 = 32768 possible satisfaction states.
 * For each state and each n (0..52), track the count.
 *
 * However, many satisfaction states are unreachable. We use a sparse representation.
 */

/* Enumerate the 15 nonempty subsets of {0,1,2,3} */
/* Subset index: for bitmask b of suits (1..15), assign an index 0..14 */
static int subset_list[15]; /* subset_list[i] = bitmask of suits */
static int subset_size[15]; /* |S| for subset i */
static int num_subsets = 15;

/* For each suit assignment T (0..15) for a rank, which subsets S does it touch?
 * Subset S is "touched" if T & S != 0, meaning this rank is in N(S).
 */
static int touch_mask[16]; /* touch_mask[T] = bitmask over 15 subsets touched by T */

static void init_subsets(void) {
    int idx = 0;
    for (int b = 1; b <= 15; b++) {
        subset_list[idx] = b;
        subset_size[idx] = __builtin_popcount(b);
        idx++;
    }

    for (int T = 0; T < 16; T++) {
        int mask = 0;
        for (int i = 0; i < 15; i++) {
            if (T & subset_list[i])
                mask |= (1 << i);
        }
        touch_mask[T] = mask;
    }
}

/*
 * DP on ranks.
 * After processing r ranks, for each satisfaction bitmask (which of 15 subsets
 * have accumulated enough ranks), and each n (cards chosen so far), store count.
 *
 * Satisfaction tracking: for each subset i, we need |N(subset_list[i])| >= subset_size[i].
 * N(S) grows as we process ranks. We need to track |N(S)| for each S.
 *
 * But tracking all 15 |N(S)| values explicitly: each can be 0..13.
 * State would be a tuple of 15 values: too large.
 *
 * Key insight: We only need to know WHICH subsets have been satisfied.
 * But when a subset S is NOT yet satisfied, we need to know |N(S)| to determine
 * if a future rank's touch could satisfy it. Without knowing |N(S)|, we can't
 * determine the transition.
 *
 * Wait, BUT: when we add a rank with suit assignment T, every subset S with
 * S & T != 0 gets |N(S)| incremented by 1. A subset S becomes satisfied when
 * |N(S)| >= |S|.
 *
 * So we need to track |N(S)| for each unsatisfied S. For satisfied S, we don't care.
 *
 * For 4 suits:
 * - 4 singletons: satisfied when |N({i})| >= 1 (at least one rank in suit i)
 * - 6 pairs: satisfied when |N({i,j})| >= 2
 * - 4 triples: satisfied when |N({i,j,k})| >= 3
 * - 1 quad: satisfied when |N({0,1,2,3})| >= 4
 *
 * After processing r ranks, the max |N(S)| is r for any S. For the singletons,
 * |N({i})| = number of ranks with suit i included. For pairs, |N({i,j})| =
 * number of ranks that include at least one of i,j.
 *
 * This means tracking 15 counters is necessary, each 0..13.
 * Total states: 14^15 ≈ 10^17. WAY too many.
 *
 * Alternative: since N(S) is determined by the union of the T_r's intersected
 * with S, and N(S) = |{r : T_r & S != 0}|, we can note that N(S) is determined
 * by the MULTISET of T_r values.
 *
 * But the multiset has 16^13 / symmetry... still huge.
 *
 * NEW APPROACH: Don't track satisfaction bitmask. Instead, use inclusion-exclusion
 * on which suits are "restricted".
 *
 * g(n) = C(52,n) - f(n) where g(n) = hands with no Badugi (no perfect matching).
 *
 * By inclusion-exclusion on Hall's violations:
 * A hand has NO perfect matching iff there exists S subset suits with |N(S)| < |S|.
 *
 * Use complementary counting:
 * f(n) = sum over injections sigma: suits -> ranks of (-1)^{complement} ...
 * Actually, by permanent formula:
 * f(n) = number of hands containing a transversal (SDR).
 *
 * Using permanent-based inclusion-exclusion:
 * f(n) = sum_{T subset suits} (-1)^{4-|T|} * h(T, n)
 * where h(T, n) = number of n-card hands where for each suit in T, there's a
 * distinct rank assigned to it.
 * Wait, this isn't quite right either.
 *
 * THE CORRECT APPROACH: Use the formula for the permanent.
 * perm(A) = sum_{S subset cols} (-1)^{n-|S|} * product of row sums restricted to S.
 * This is Ryser's formula.
 *
 * But we're not computing a single permanent. We need to sum over all hands.
 *
 * Let me try a COMPLETELY different approach.
 *
 * For each hand H, [H has Badugi] = [exists injection sigma: {0,1,2,3} -> {0..12}
 * such that card(sigma(i), i) in H for all i].
 *
 * f(n) = sum_H [H has Badugi] * [|H|=n]
 *
 * Using inclusion-exclusion (complement):
 * [H has Badugi] = 1 - [H has no Badugi]
 * f(n) = C(52,n) - g(n)
 *
 * For g(n) = hands of size n with no Badugi:
 * By inclusion-exclusion on Hall's condition, using Konig-Egervary:
 *
 * Actually, let me use a different formulation.
 * A hand has NO Badugi iff for every 4-element subset of distinct ranks,
 * there's no suit assignment giving all 4 suits.
 * This is equivalent to: the maximum matching in the bipartite graph < 4.
 *
 * By Hall's theorem: max matching < 4 iff exists S with |N(S)| < |S|.
 *
 * I'll compute g(n) by summing over "deficiency structures".
 *
 * For 4 suits, the possible deficiency is 1, 2, 3, or 4.
 * deficiency = max_S (|S| - |N(S)|).
 * max matching = 4 - deficiency.
 *
 * We compute g(n) = sum over assignments (T_0,...,T_12) with sum|T_r|=n and
 * max matching < 4.
 *
 * Let me use inclusion-exclusion via the COMPLEMENT:
 * f(n) = sum_{sigma} (-1)^{...} ...
 *
 * OK, I think the cleanest way for a small problem (4 suits, 13 ranks) is:
 *
 * f(n) = sum_{sigma: {0,1,2,3} -> {0..12} injective} F(sigma, n)
 *      - sum_{sigma1 != sigma2} F(sigma1 & sigma2, n) + ...
 *
 * where F(sigma, n) = number of n-card hands containing all 4 cards of Badugi sigma.
 * F(sigma, n) = C(52 - 4, n - 4) if all 4 cards distinct (always true for Badugi).
 *
 * But this is the standard inclusion-exclusion over Badugis, and there are C(13,4)*4!=17160.
 * The number of pairs is C(17160,2) ≈ 1.5*10^8 which is too many for direct computation.
 *
 * HOWEVER, we can group Badugis by their intersection type.
 * Two Badugis B1, B2 overlap in k suits (same suit, same rank) and share j common cards.
 * The union size is |B1 cup B2| = 8 - j.
 *
 * Key: the number of pairs of Badugis with a given union size can be computed analytically.
 *
 * For the inclusion-exclusion to work to sufficient order:
 * f(n) = sum_{k=1}^{17160} (-1)^{k+1} * C_k(n)
 * where C_k(n) = sum over k-element subsets of Badugis of C(52-|union|, n-|union|).
 *
 * We need C_k up to some manageable k. For the problem (n <= 13), the maximum Badugi
 * overlap is limited.
 *
 * A Badugi uses 4 specific (rank,suit) pairs. Two Badugis share a card if they
 * assign the same rank to the same suit. Since each Badugi is an injection
 * suits -> ranks, two Badugis B1, B2 share k cards where k = |{s : B1(s) = B2(s)}|.
 *
 * For the union: |B1 union B2| = |B1| + |B2| - |B1 inter B2| = 4 + 4 - k = 8 - k.
 *
 * Similarly for m Badugis, their union size depends on overlaps.
 *
 * I think the BEST approach for this problem is:
 * Use inclusion-exclusion on suits being "blocked". Specifically:
 *
 * For subset S of suits, let B(S, n) = number of n-card hands where ALL ranks
 * that appear are contained in the complement of some "blocking set" for S.
 *
 * Actually, let me use the standard approach via Ryser's formula for permanent.
 *
 * number of SDRs for hand H = permanent of the 4x13 matrix M where M[i][r] = 1 if
 * card (r,i) is in H. By Ryser:
 * perm(M) = (-1)^4 * sum_{S subset {0..12}} (-1)^|S| * prod_{i=0}^3 (sum_{r in S} M[i][r])
 *
 * But we want f(n) = sum over hands H with |H|=n of [perm(M_H) > 0].
 * The indicator [perm > 0] is hard to compute.
 *
 * Let me try yet another approach: the rank profile DP.
 *
 * State: (n0, n1, n2, n3) where n_i = number of ranks assigned to suit i
 * (not cards, but distinct ranks). And also, the number of ranks that touch
 * various suit combinations.
 *
 * Actually, the most tractable approach: for each rank, choose a subset of suits.
 * The choice is one of 2^4 = 16 options. After 13 ranks, we have a sequence of
 * 13 subsets. The hand is determined by this sequence (which cards are included).
 *
 * The number of cards = sum |T_r|.
 * The matching condition depends on whether a transversal exists.
 *
 * For the matching, what we ACTUALLY need to track is the matching number.
 * For 4 suits, we can use Konig's theorem: matching number = 4 - max deficiency.
 *
 * The deficiency can be tracked efficiently. For each S subset suits,
 * |N(S)| = number of ranks r where T_r & S != 0.
 * Deficiency(S) = max(0, |S| - |N(S)|).
 * Max matching = 4 - max_S deficiency(S).
 * Perfect matching iff max_S deficiency(S) = 0 iff all subsets S satisfy |N(S)| >= |S|.
 *
 * Tracking all 15 |N(S)| values: state is (n_S)_{S in 2^{suits}\{empty}} with n_S in 0..13.
 * That's 14^15 states, way too many.
 *
 * BUT: we can note that N(S union T) >= max(N(S), N(T)), so the values are related.
 * Also, N(S) is determined by the counts n(T) for individual suit subsets T.
 *
 * Key simplification: N(S) = |{r : T_r & S != 0}| = 13 - |{r : T_r & S == 0}|
 *                           = 13 - |{r : T_r subset complement(S)}|.
 *
 * So N(S) = 13 - f(complement(S)) where f(U) = number of ranks with T_r subset U.
 *
 * Let g(U) = number of ranks r with T_r = U (frequency of each pattern).
 * Then f(U) = sum_{V subset U} g(V).
 * And N(S) = 13 - sum_{V subset complement(S)} g(V) = sum_{V NOT subset complement(S)} g(V)
 *          = sum_{V & S != 0} g(V).
 *
 * So g(V) for V subset {0,1,2,3} determines everything. g(V) >= 0 and sum_V g(V) = 13.
 * The number of cards = sum_V g(V)*|V|.
 *
 * The state is the frequency vector (g(V))_{V} with sum = 13.
 * V can be any of 2^4 = 16 subsets. g(V) in {0,...,13} with sum = 13.
 * Number of such vectors = C(13+15, 15) = C(28,15) = 3108105. Manageable!
 *
 * But we also need n = sum g(V)*|V|, which varies from 0 to 52.
 * For each state (g vector), n is determined.
 *
 * Wait, the NUMBER of hands corresponding to frequency vector g is the multinomial:
 * 13! / (product_V g(V)!) since we're assigning each of 13 ranks to a pattern V.
 * The number of cards = sum g(V)*|V|.
 * The matching exists iff for all nonempty S, N(S) = sum_{V & S != 0} g(V) >= |S|.
 *
 * So: f(n) = sum over valid g vectors with sum g(V)*|V| = n and Hall condition of
 *     13! / product g(V)!
 *
 * This is the key formula! The number of frequency vectors with sum = 13 is C(28,15) ≈ 3M.
 * Checking Hall's condition and computing the multinomial for each is fast.
 *
 * But C(28,15) = 3108105, and for each we check Hall and compute multinomial.
 * That's about 3M iterations, very fast in C.
 *
 * Actually, we need the multinomial coefficient, which is 13! / product g_V!.
 * And the number of cards n = sum g_V * |V|. We want sum over all valid g with
 * fixed n and Hall condition satisfied.
 *
 * Implementation: enumerate all g vectors with sum = 13 over 16 bins.
 * Use recursive enumeration with pruning.
 */

#define NSUITS 4
#define NRANKS 13
#define NPATTERNS 16  /* 2^4 subsets of suits */

/* Precompute factorials, |V| for each V, and Hall condition check */
static long long fact[NRANKS + 1];
/* C(52, n) */
static long long C52[53];

static int pattern_size[NPATTERNS]; /* |V| for each V */

static void init(void) {
    fact[0] = 1;
    for (int i = 1; i <= NRANKS; i++) fact[i] = fact[i-1] * i;

    /* C(52, n) */
    C52[0] = 1;
    for (int n = 1; n <= 52; n++)
        C52[n] = C52[n-1] * (52 - n + 1) / n;

    for (int v = 0; v < NPATTERNS; v++)
        pattern_size[v] = __builtin_popcount(v);
}

/* Check Hall's condition: for all nonempty S subset {0..3}, N(S) >= |S|.
 * N(S) = sum_{V & S != 0} g[V]. */
static int check_hall(int g[NPATTERNS]) {
    for (int S = 1; S < 16; S++) {
        int ssize = __builtin_popcount(S);
        int NS = 0;
        for (int V = 1; V < 16; V++) {
            if ((V & S) && g[V] > 0) NS += g[V];
        }
        if (NS < ssize) return 0;
    }
    return 1;
}

/* Multinomial coefficient: 13! / product g[V]! */
static long long multinomial(int g[NPATTERNS]) {
    long long denom = 1;
    for (int v = 0; v < NPATTERNS; v++)
        denom *= fact[g[v]];
    return fact[NRANKS] / denom;
}

/* f[n] accumulator for n = 0..52 */
static long long f_count[53]; /* count of hands with Badugi */
static long long total_count[53]; /* total hands per n (should match C(52,n)) */

/* Recursive enumeration of g vectors.
 * g[0..15], processing pattern index 'idx', remaining ranks 'rem'. */
static int g_vec[NPATTERNS];

static void enumerate(int idx, int rem) {
    if (idx == NPATTERNS - 1) {
        /* Last pattern gets all remaining ranks */
        g_vec[idx] = rem;

        /* Compute n = sum g_V * |V| */
        int n = 0;
        for (int v = 0; v < NPATTERNS; v++)
            n += g_vec[v] * pattern_size[v];

        if (n < 4 || n > 13) {
            g_vec[idx] = 0;
            return; /* Only care about n in [4, 13] */
        }

        /* Check Hall's condition */
        if (check_hall(g_vec)) {
            long long coeff = multinomial(g_vec);
            f_count[n] += coeff;
        }

        g_vec[idx] = 0;
        return;
    }

    for (int k = 0; k <= rem; k++) {
        g_vec[idx] = k;
        enumerate(idx + 1, rem - k);
    }
    g_vec[idx] = 0;
}

int main(void) {
    init();
    memset(g_vec, 0, sizeof(g_vec));
    memset(f_count, 0, sizeof(f_count));

    enumerate(0, NRANKS);

    long long total = 0;
    for (int n = 4; n <= 13; n++) {
        total += f_count[n];
    }
    printf("%lld\n", total);
    return 0;
}
""";

    tmpdir = tempfile.mkdtemp()
    src = os.path.join(tmpdir, "sol369.c")
    exe = os.path.join(tmpdir, "sol369")
    with open(src, 'w') as f:
        f.write(c_code)
    subprocess.run(["gcc", "-O3", "-o", exe, src, "-lm"], check=True, capture_output=True)
    result = subprocess.run([exe], capture_output=True, text=True, check=True, timeout=30)
    print(result.stdout.strip())

if __name__ == "__main__":
    solve()
