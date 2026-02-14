/*
 * Project Euler Problem 238: Infinite string tour
 *
 * BBS generator: s_0 = 14025256, s_{n+1} = s_n^2 mod 20300713
 * Concatenate all s_i to form infinite string w.
 * p(k) = position of earliest substring with digit sum k.
 * Answer = sum of p(k) for k = 1 to 2*10^15.
 *
 * The BBS sequence is periodic. We find the period, compute cumulative
 * digit sums, and use the structure to answer all queries efficiently.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define S0 14025256
#define M 20300713
#define MAX_PERIOD 3000000

int main(void) {
    long long N = 2000000000000000LL; /* 2 * 10^15 */

    /* Phase 1: Generate BBS sequence and find period */
    int *seq = (int *)malloc(MAX_PERIOD * sizeof(int));
    long long s = S0;
    int period = 0;
    do {
        seq[period++] = (int)s;
        s = (s * s) % M;
    } while (s != S0 && period < MAX_PERIOD);

    /* Phase 2: Build digit string and cumulative sums */
    /* First compute total length and digit sum of one period */
    long long total_len = 0;
    long long total_dsum = 0;

    /* We need digit counts for each number in the sequence */
    /* Store offsets: offset[i] = starting position of seq[i]'s digits in the period string */
    long long *offset = (long long *)malloc((period + 1) * sizeof(long long));
    offset[0] = 0;

    for (int i = 0; i < period; i++) {
        int n = seq[i];
        int digits = 0;
        int temp = n;
        if (temp == 0) { digits = 1; }
        else { while (temp > 0) { digits++; temp /= 10; } }
        total_len += digits;
        offset[i + 1] = total_len;

        /* Digit sum */
        temp = n;
        while (temp > 0) { total_dsum += temp % 10; temp /= 10; }
    }

    long long L = total_len;
    long long D = total_dsum;

    /* Phase 3: Compute cumulative digit sums and first occurrences */
    /* C[i] = cumulative digit sum from position 0 to position i-1 */
    /* We need C[pos] mod D for each position pos in [0, L) */

    /* Since L can be huge (~17M * 7 digits ~ 120M), we need efficient storage */
    /* Actually let's compute the cumulative digit sum at each digit position */

    /* Allocate array for cumulative sums mod D at each position */
    /* L can be up to ~120M, so we need arrays of that size */

    long long *cumsum = (long long *)malloc((L + 1) * sizeof(long long));
    cumsum[0] = 0;

    long long pos = 0;
    for (int i = 0; i < period; i++) {
        int n = seq[i];
        char buf[12];
        int len = sprintf(buf, "%d", n);
        for (int j = 0; j < len; j++) {
            cumsum[pos + 1] = cumsum[pos] + (buf[j] - '0');
            pos++;
        }
    }

    /* Phase 4: Find first occurrence of each cumulative sum value mod D */
    /* S = set of all C[i] mod D for i in [0, L) */
    /* For each position a (first occurrence), and each s in S:
     * residue r = (s - C[a]) mod D is newly covered with f(r) = a + 1 */

    /* First, compute C_mod[i] = cumsum[i] % D */
    /* Find first occurrence of each C_mod value */

    /* Use a hash: first_occ[v] = first position i where C_mod[i] == v */
    /* D can be large. Let's compute D first. */
    /* D = total_dsum ~ 120M * 4.5 ~ 540M? Actually smaller. */

    /* Build S_bool: a boolean array of size D, S_bool[v] = 1 if v appears in C_mod */
    char *S_bool = (char *)calloc(D, sizeof(char));

    /* first_pos_of_val: first index where C_mod takes a given value */
    /* We process positions in order, recording first occurrences */

    /* For efficiency, collect unique (value, first_position) pairs sorted by position */
    typedef struct { long long val; long long pos; } ValPos;
    ValPos *first_occs = (ValPos *)malloc(D * sizeof(ValPos));
    int num_unique = 0;

    for (long long i = 0; i < L; i++) {
        long long v = cumsum[i] % D;
        if (!S_bool[v]) {
            S_bool[v] = 1;
            first_occs[num_unique].val = v;
            first_occs[num_unique].pos = i;
            num_unique++;
        }
    }

    /* Sort first_occs by position (they should already be in order since we iterate i=0..L-1) */
    /* They are already sorted by position */

    long long Q = N / D;
    long long R = N % D;

    char *covered = (char *)calloc(D, sizeof(char));
    long long total_F = 0;
    long long partial_G = 0;
    long long total_covered = 0;

    /* For each new first occurrence (in order of position), compute which residues
     * are newly covered */
    for (int step = 0; step < num_unique && total_covered < D; step++) {
        long long a_pos = first_occs[step].pos;
        long long v = first_occs[step].val;
        long long f_val = a_pos + 1;

        /* For each s in S, residue r = (s - v + D) % D is covered at cost f_val */
        /* We iterate over all unique values in S */
        for (int si = 0; si < num_unique; si++) {
            long long s_val = first_occs[si].val;
            long long r = (s_val - v % D + D) % D;
            if (!covered[r]) {
                covered[r] = 1;
                total_F += f_val;
                total_covered++;
                if (r >= 1 && r <= R)
                    partial_G += f_val;
            }
        }
    }

    long long answer = Q * total_F + partial_G;
    printf("%lld\n", answer);

    free(seq);
    free(offset);
    free(cumsum);
    free(S_bool);
    free(first_occs);
    free(covered);
    return 0;
}
