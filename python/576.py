"""Project Euler Problem 576 — Irrational jumps, embedded C port."""
import subprocess, tempfile, os

C_CODE = r'''
#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <string.h>

/* Sieve primes up to limit */
int sieve(int limit, int *primes) {
    char *is_prime = calloc(limit + 1, 1);
    int count = 0;
    for (int i = 2; i <= limit; i++) is_prime[i] = 1;
    for (int i = 2; (long long)i * i <= limit; i++)
        if (is_prime[i])
            for (int j = i * i; j <= limit; j += i)
                is_prime[j] = 0;
    for (int i = 2; i <= limit; i++)
        if (is_prime[i]) primes[count++] = i;
    free(is_prime);
    return count;
}

typedef struct {
    int prime_idx;    /* index into primes array */
    double total_len; /* i * sqrt(1/p) */
} JumpPos;

static int cmp_frac(const void *a, const void *b) {
    double fa = ((JumpPos*)a)->total_len - floor(((JumpPos*)a)->total_len);
    double fb = ((JumpPos*)b)->total_len - floor(((JumpPos*)b)->total_len);
    if (fa < fb) return -1;
    if (fa > fb) return 1;
    return 0;
}

int main(void) {
    int N = 100;
    double D = 0.00002;

    int primes[30];
    int nprimes = sieve(N, primes);

    /* Generate all jump positions */
    /* Estimate max positions: for each prime, up to ~1/D = 50000 jumps */
    int max_total = nprimes * 60000;
    JumpPos *all_pos = malloc(max_total * sizeof(JumpPos));
    int total_count = 0;

    for (int pi = 0; pi < nprimes; pi++) {
        int p = primes[pi];
        double sqrt_inv_p = sqrt(1.0 / p);

        /* Temp storage for this prime's positions */
        int cap = 1024;
        JumpPos *tmp = malloc(cap * sizeof(JumpPos));
        int cnt = 0;

        int i = 0;
        while (1) {
            if (cnt >= cap) {
                cap *= 2;
                tmp = realloc(tmp, cap * sizeof(JumpPos));
            }
            tmp[cnt].prime_idx = pi;
            tmp[cnt].total_len = i * sqrt_inv_p;
            cnt++;
            i++;

            /* Check if all positions are within D (at power-of-2-minus-1 boundaries) */
            if (i > 1 && (i & (i + 1)) == 0) {
                /* Sort tmp by fractional part */
                qsort(tmp, cnt, sizeof(JumpPos), cmp_frac);
                int all_within = 1;
                for (int j = 1; j < cnt; j++) {
                    double f1 = tmp[j].total_len - floor(tmp[j].total_len);
                    double f0 = tmp[j-1].total_len - floor(tmp[j-1].total_len);
                    if (f1 - f0 > D) {
                        all_within = 0;
                        break;
                    }
                }
                if (all_within) break;
            }
        }

        /* Add to global list */
        if (total_count + cnt > max_total) {
            max_total = (total_count + cnt) * 2;
            all_pos = realloc(all_pos, max_total * sizeof(JumpPos));
        }
        memcpy(all_pos + total_count, tmp, cnt * sizeof(JumpPos));
        total_count += cnt;
        free(tmp);
    }

    /* Sort all positions by fractional part */
    qsort(all_pos, total_count, sizeof(JumpPos), cmp_frac);

    /* Sliding window */
    double ans = 0.0;
    int start = 0;
    int end = nprimes;

    /* For each prime, track the minimum total_len in current window */
    double *min_len = malloc(nprimes * sizeof(double));

    while (end < total_count) {
        double frac_end = all_pos[end].total_len - floor(all_pos[end].total_len);
        double frac_start = all_pos[start].total_len - floor(all_pos[start].total_len);

        while (frac_end - frac_start > D) {
            start++;
            frac_start = all_pos[start].total_len - floor(all_pos[start].total_len);
        }

        /* Compute S for each prime: minimum total_len in window */
        for (int pi = 0; pi < nprimes; pi++) min_len[pi] = 1e18;

        for (int idx = start; idx < end; idx++) {
            int pi = all_pos[idx].prime_idx;
            if (all_pos[idx].total_len < min_len[pi])
                min_len[pi] = all_pos[idx].total_len;
        }

        double total = 0.0;
        for (int pi = 0; pi < nprimes; pi++)
            if (min_len[pi] < 1e17) total += min_len[pi];

        if (total > ans) ans = total;
        end++;
    }

    printf("%.4f\n", ans);

    free(all_pos);
    free(min_len);
    return 0;
}
'''

if __name__ == "__main__":
    with tempfile.NamedTemporaryFile(suffix='.c', delete=False) as f:
        f.write(C_CODE.encode())
        c_file = f.name
    exe = c_file[:-2]
    try:
        subprocess.run(['gcc', '-O2', '-o', exe, c_file, '-lm'], check=True, capture_output=True)
        result = subprocess.run([exe], capture_output=True, text=True, timeout=280)
        print(result.stdout.strip())
    finally:
        os.unlink(c_file)
        if os.path.exists(exe):
            os.unlink(exe)
