"""Project Euler Problem 629: Scatterstone Nim.

Compute g(200) = sum_{k=2}^{200} f(200,k) mod 10^9+7,
where f(n,k) counts winning positions (non-zero Grundy value partitions).

Nimbers: k=2 gives alternating 0,1; k=3 computed directly; k>=4 gives n-1.
Uses C for the partition DP with XOR tracking.
"""

import subprocess, tempfile, os

def solve():
    N = 200

    # Compute nimbers for k=2
    nimbers2 = [0] * (N + 1)
    for n in range(2, N + 1):
        used = set()
        for i in range(1, n):
            used.add(nimbers2[i] ^ nimbers2[n - i])
        mex = 0
        while mex in used:
            mex += 1
        nimbers2[n] = mex

    # Compute nimbers for k=3
    nimbers3 = [0] * (N + 1)
    for n in range(2, N + 1):
        used = set()
        for i in range(1, n):
            used.add(nimbers3[i] ^ nimbers3[n - i])
        for i in range(1, n):
            for j in range(i, n - i):
                k_val = n - i - j
                if k_val >= j:
                    used.add(nimbers3[i] ^ nimbers3[j] ^ nimbers3[k_val])
        mex = 0
        while mex in used:
            mex += 1
        nimbers3[n] = mex

    # For k>=4, nimbers are 0,0,1,2,...,N-1
    nimbers4 = [0] * (N + 1)
    for n in range(2, N + 1):
        nimbers4[n] = n - 1

    xor_size = 256  # Enough for max nimber < 200

    n2_str = ','.join(str(x) for x in nimbers2)
    n3_str = ','.join(str(x) for x in nimbers3)
    n4_str = ','.join(str(x) for x in nimbers4)

    c_code = f"""
#include <stdio.h>
#include <string.h>

#define NN 200
#define MOD 1000000007LL
#define XS {xor_size}

static int nimbers2[NN+1] = {{{n2_str}}};
static int nimbers3[NN+1] = {{{n3_str}}};
static int nimbers4[NN+1] = {{{n4_str}}};

/* dp[a][x] = # partitions of a stones using parts of size <= current_d,
   with XOR of nimbers = x */
static long long dp[NN+1][XS];
static long long tmp[NN+1][XS];

static long long count_winning(int *nimbers) {{
    memset(dp, 0, sizeof(dp));
    dp[0][0] = 1;

    for (int d = 1; d <= NN; d++) {{
        int g = nimbers[d];
        /* Add 0 or more copies of pile size d.
           tmp[a][x] = # ways to use >= 1 copy of size d to fill a stones
                       with XOR = x (given dp has contributions from sizes < d).
           Recurrence: tmp[a][x] = dp[a-d][x^g] + tmp[a-d][x^g]
           (one copy from dp, or one more copy from tmp)
        */
        memset(tmp, 0, sizeof(tmp));
        for (int a = d; a <= NN; a++) {{
            for (int x = 0; x < XS; x++) {{
                int px = x ^ g;
                tmp[a][x] = (dp[a - d][px] + tmp[a - d][px]) % MOD;
            }}
        }}

        /* dp += tmp */
        for (int a = 0; a <= NN; a++) {{
            for (int x = 0; x < XS; x++) {{
                dp[a][x] = (dp[a][x] + tmp[a][x]) % MOD;
            }}
        }}
    }}

    long long result = 0;
    for (int x = 1; x < XS; x++) {{
        result = (result + dp[NN][x]) % MOD;
    }}
    return result;
}}

int main(void) {{
    long long f2 = count_winning(nimbers2);
    long long f3 = count_winning(nimbers3);
    long long f4 = count_winning(nimbers4);

    /* g(N) = f2 + f3 + (N-3)*f4 since f(N,k) = f4 for all k >= 4 */
    long long ans = (f2 + f3 + (long long)(NN - 3) % MOD * (f4 % MOD)) % MOD;
    printf("%lld\\n", ans);
    return 0;
}}
"""

    with tempfile.NamedTemporaryFile(suffix='.c', mode='w', delete=False) as f:
        f.write(c_code)
        c_path = f.name

    bin_path = c_path.replace('.c', '')
    try:
        subprocess.run(['gcc', '-O2', '-o', bin_path, c_path], check=True,
                       capture_output=True, text=True)
        result = subprocess.run([bin_path], capture_output=True, text=True, timeout=30)
        print(result.stdout.strip())
    finally:
        os.unlink(c_path)
        if os.path.exists(bin_path):
            os.unlink(bin_path)

if __name__ == "__main__":
    solve()
