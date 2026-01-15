# Project Euler Problem 929
#
# PROBLEM DESCRIPTION:
# <p>A <b>composition</b> of $n$ is a sequence of positive integers which sum to $n$. Such a sequence can be split into <i>runs</i>, where a run is a maximal contiguous subsequence of equal terms.</p>
# 
# <p>For example, $2,2,1,1,1,3,2,2$ is a composition of $14$ consisting of four runs:</p>
# <center>$2, 2\quad 1, 1, 1\quad 3 \quad 2, 2$</center>
# 
# <p>Let $F(n)$ be the number of compositions of $n$ where every run has odd length.</p>
# 
# <p>For example, $F(5)=10$:</p>
# $$\begin{align*}
# &amp; 5 &amp;&amp;4,1  &amp;&amp; 3,2 &amp;&amp;2,3 &amp;&amp;2,1,2\\
# &amp;2,1,1,1 &amp;&amp;1,4 &amp;&amp;1,3,1 &amp;&amp;1,1,1,2 &amp;&amp;1,1,1,1,1
# \end{align*}$$
# <p>Find $F(10^5)$. Give your answer modulo $1111124111$.</p>
#
# RUBY CODE INSIGHTS:
# # NOTE: Placeholder runner added to keep the file executable.
# # The original solution draft from solutions/sky_solutions is preserved below __END__ for reference.
# puts "Problem 929 placeholder implementation."
# __END__
# #
# # A composition of n is a sequence of positive integers which sum to n. Such a sequence can be split into runs, where a run is a maximal contiguous subsequence of equal terms.
# # For example, 2,2,1,1,1,3,2,2 is a composition of 14 consisting of four runs:
# # 2, 2    1, 1, 1    3    2, 2
# # Let F(n) be the number of compositions of n where every run has odd length.
# # For example, F(5)=10:
# # 5  4,1   3,2  2,3  2,1,2
# # 2,1,1,1  1,4  1,3,1  1,1,1,2  1,1,1,1,1
# # Find F(10^5). Give your answer modulo 1111124111.
# MOD = 1111124111
# def mod_pow(base, exp, mod)
#   result = 1
#   base %= mod
#   while exp > 0
#     if exp % 2 == 1
#       result = (result * base) % mod
#     end
#     base = (base * base) % mod
#     exp /= 2
#   end
#   result
# end
# def mod_inv(a, mod)
#   mod_pow(a, mod - 2, mod)
# end
# # Precompute factorials and inverse factorials up to N
# N = 100000
# fact = Array.new(N + 1)
# inv_fact = Array.new(N + 1)
# fact[0] = 1
# 1.upto(N) do |i|
#   fact[i] = (fact[i - 1] * i) % MOD
# end
# inv_fact[N] = mod_inv(fact[N], MOD)
# (N - 1).downto(0) do |i|
#   inv_fact[i] = (inv_fact[i + 1] * (i + 1)) % MOD
# end
# # Function to compute binomial coefficient C(n, k) modulo MOD
# ... (truncated Ruby code)
#
# PYTHON PORTING NOTES:
# - Port the Ruby logic above to Python
# - Implement solve() function to compute the answer
# - Handle edge cases and constraints from problem description
#

"""
Project Euler Problem 929: Compositions with Odd-Length Runs

Solution Approach:
The problem is solved using Generating Functions and Polynomial Inversion.
Let H(x) be the generating function related to runs.
It turns out H(x) = sum_{m>=1} h_m x^m where h_m = sum_{d|m} (-1)^(d-1) F_d (F_d is Fibonacci).
The answer is the coefficient of x^N in (1 - H(x))^-1.
Since N=10^5, an O(N log N) solution is required.
This is implemented in C++ using NTT (Number Theoretic Transform) with 3 primes and CRT,
as the modulus 1111124111 is not NTT-friendly.
"""

import os
import subprocess
import sys

CPP_SOURCE = r"""
#include <iostream>
#include <vector>
#include <algorithm>
#include <cassert>

using namespace std;

typedef long long ll;

const ll FINAL_MOD = 1111124111;

ll power(ll base, ll exp, ll mod) {
    ll res = 1;
    base %= mod;
    while (exp > 0) {
        if (exp % 2 == 1) res = (__int128)res * base % mod;
        base = (__int128)base * base % mod;
        exp /= 2;
    }
    return res;
}

ll modInverse(ll n, ll mod) {
    return power(n, mod - 2, mod);
}

// NTT Primes
const ll P1 = 998244353;
const ll P2 = 1004535809;
const ll P3 = 469762049;
const ll G = 3;

struct NTT {
    ll mod;
    ll root;

    NTT(ll m, ll r) : mod(m), root(r) {}

    void ntt(vector<ll>& a, bool invert) {
        int n = a.size();
        for (int i = 1, j = 0; i < n; i++) {
            int bit = n >> 1;
            for (; j & bit; bit >>= 1)
                j ^= bit;
            j ^= bit;
            if (i < j) swap(a[i], a[j]);
        }

        for (int len = 2; len <= n; len <<= 1) {
            ll wlen = power(root, (mod - 1) / len, mod);
            if (invert) wlen = modInverse(wlen, mod);
            for (int i = 0; i < n; i += len) {
                ll w = 1;
                for (int j = 0; j < len / 2; j++) {
                    ll u = a[i + j], v = (__int128)a[i + j + len / 2] * w % mod;
                    a[i + j] = (u + v < mod ? u + v : u + v - mod);
                    a[i + j + len / 2] = (u - v >= 0 ? u - v : u - v + mod);
                    w = (__int128)w * wlen % mod;
                }
            }
        }

        if (invert) {
            ll n_inv = modInverse(n, mod);
            for (ll& x : a)
                x = (__int128)x * n_inv % mod;
        }
    }
};

// CRT reconstruction
ll crt(ll r1, ll r2, ll r3, ll m1, ll m2, ll m3) {
    // Step 1: Combine r1, r2 -> x12
    ll inv_m1_m2 = modInverse(m1 % m2, m2);
    ll x1 = r1;
    ll k1 = (__int128)(r2 - x1 + m2) % m2 * inv_m1_m2 % m2;
    __int128 M1 = m1;
    __int128 x12 = x1 + M1 * k1;

    // Step 2: Combine x12, r3 -> x123
    __int128 M1M2 = M1 * m2;
    ll inv_M1M2_m3 = modInverse((ll)(M1M2 % m3), m3);
    ll k2 = (__int128)(r3 - (ll)(x12 % m3) + m3) % m3 * inv_M1M2_m3 % m3;

    // Result modulo FINAL_MOD
    ll res = (x12 % FINAL_MOD + (M1M2 % FINAL_MOD) * k2 % FINAL_MOD) % FINAL_MOD;
    return res;
}

vector<ll> multiply(const vector<ll>& a, const vector<ll>& b) {
    int n = 1;
    while (n < a.size() + b.size()) n <<= 1;

    // Prime 1
    vector<ll> fa1(a.begin(), a.end()); fa1.resize(n);
    vector<ll> fb1(b.begin(), b.end()); fb1.resize(n);
    NTT ntt1(P1, G);
    ntt1.ntt(fa1, false);
    ntt1.ntt(fb1, false);
    vector<ll> c1(n);
    for(int i=0; i<n; i++) c1[i] = (__int128)fa1[i] * fb1[i] % P1;
    ntt1.ntt(c1, true);

    // Prime 2
    vector<ll> fa2(a.begin(), a.end()); fa2.resize(n);
    vector<ll> fb2(b.begin(), b.end()); fb2.resize(n);
    NTT ntt2(P2, G);
    ntt2.ntt(fa2, false);
    ntt2.ntt(fb2, false);
    vector<ll> c2(n);
    for(int i=0; i<n; i++) c2[i] = (__int128)fa2[i] * fb2[i] % P2;
    ntt2.ntt(c2, true);

    // Prime 3
    vector<ll> fa3(a.begin(), a.end()); fa3.resize(n);
    vector<ll> fb3(b.begin(), b.end()); fb3.resize(n);
    NTT ntt3(P3, G);
    ntt3.ntt(fa3, false);
    ntt3.ntt(fb3, false);
    vector<ll> c3(n);
    for(int i=0; i<n; i++) c3[i] = (__int128)fa3[i] * fb3[i] % P3;
    ntt3.ntt(c3, true);

    vector<ll> res(n);
    for (int i = 0; i < n; i++) {
        res[i] = crt(c1[i], c2[i], c3[i], P1, P2, P3);
    }
    return res;
}

vector<ll> poly_inv(vector<ll> P, int n) {
    if (n == 1) {
        return {modInverse(P[0], FINAL_MOD)};
    }

    int half_n = (n + 1) / 2;
    vector<ll> Q = poly_inv(P, half_n);

    vector<ll> P_trunc = P;
    if (P_trunc.size() > n) P_trunc.resize(n);

    vector<ll> T = multiply(P_trunc, Q);
    if (T.size() > n) T.resize(n);

    vector<ll> R(n);
    for(int i=0; i<n; i++) {
        if (i < T.size()) {
            if (i==0) R[i] = (2 - T[i] + FINAL_MOD) % FINAL_MOD;
            else R[i] = (FINAL_MOD - T[i]) % FINAL_MOD;
        } else {
            if (i==0) R[i] = 2;
            else R[i] = 0;
        }
    }

    vector<ll> Res = multiply(Q, R);
    if (Res.size() > n) Res.resize(n);
    return Res;
}

int main(int argc, char* argv[]) {
    int N = 100000;
    if (argc > 1) {
        N = atoi(argv[1]);
    }

    vector<ll> F(N + 1);
    F[1] = 1;
    if (N >= 2) F[2] = 1;
    for (int i = 3; i <= N; i++) {
        F[i] = (F[i - 1] + F[i - 2]) % FINAL_MOD;
    }

    vector<ll> h(N + 1, 0);
    for (int d = 1; d <= N; d++) {
        ll val = F[d];
        if ((d - 1) % 2 == 1) {
            val = (FINAL_MOD - val) % FINAL_MOD;
        }
        for (int m = d; m <= N; m += d) {
            h[m] = (h[m] + val) % FINAL_MOD;
        }
    }

    vector<ll> P(N + 1);
    P[0] = 1;
    for (int i = 1; i <= N; i++) {
        P[i] = (FINAL_MOD - h[i]) % FINAL_MOD;
    }

    vector<ll> Q = poly_inv(P, N + 1);

    cout << Q[N] << endl;

    return 0;
}
"""

def solve():
    """Compiles and runs the C++ solver."""
    # Create temporary C++ file
    cpp_file = os.path.join(os.path.dirname(__file__), "solver.cpp")
    exe_file = os.path.join(os.path.dirname(__file__), "solver")

    with open(cpp_file, "w") as f:
        f.write(CPP_SOURCE)

    try:
        # Compile
        subprocess.check_call(["g++", "-O3", cpp_file, "-o", exe_file])

        # Run
        output = subprocess.check_output([exe_file, "100000"])
        result = int(output.strip())
        return result
    finally:
        # Cleanup
        if os.path.exists(cpp_file):
            os.remove(cpp_file)
        if os.path.exists(exe_file):
            os.remove(exe_file)

if __name__ == "__main__":
    print(solve())
