# Project Euler Problem 946
#
# PROBLEM DESCRIPTION:
# <p>Given the representation of a continued fraction
# $$ a_0+ \cfrac 1{a_1+ \cfrac 1{a_2+\cfrac 1{a_3+\ddots }}}= [a_0;a_1,a_2,a_3,\ldots] $$</p>
# 
# <p>
# $\alpha$ is a real number with continued fraction representation:
# $\alpha = [2;1,1,2,1,1,1,2,1,1,1,1,1,2,1,1,1,1,1,1,1,2,1,1,1,1,1,1,1,1,1,1,1,2,\ldots]$<br> where the number of $1$'s between each of the $2$'s are consecutive prime numbers.</p>
# 
# <p>
# $\beta$ is another real number defined as
# $$	\beta = \frac{2\alpha+3}{3\alpha+2} $$</p>
# 
# <p>
# The first ten coefficients of the continued fraction of $\beta$ are $[0;1,5,6,16,9,1,10,16,11]$ with sum $75$.</p>
# 
# <p>
# Find the sum of the first $10^8$ coefficients of the continued fraction of $\beta$.</p>
#

import os
import subprocess
import sys

def solve():
    """
    Computes the sum of the first 10^8 coefficients of the continued fraction of beta.
    Uses an embedded C++ program for performance.
    """
    cpp_source = r"""
#include <iostream>
#include <vector>
#include <cmath>

typedef __int128_t int128;

// Function to calculate floor(a / b)
long long floor_div(int128 a, int128 b) {
    if (b == 0) return 0; // Should not be used for logic if b is 0
    int128 res = a / b;
    if ((a ^ b) < 0 && (a % b != 0)) {
        res--;
    }
    return (long long)res;
}

bool is_prime(int n) {
    if (n < 2) return false;
    // Basic primality test, sufficient for small primes we'll encounter
    if (n % 2 == 0) return n == 2;
    for (int i = 3; i * i <= n; i += 2) {
        if (n % i == 0) return false;
    }
    return true;
}

int main() {
    // Beta = (2 alpha + 3) / (3 alpha + 2)
    int128 A = 2;
    int128 B = 3;
    int128 C = 3;
    int128 D = 2;

    long long sum_beta = 0;
    long long count_beta = 0;
    long long target = 100000000;

    // Alpha generation state
    // 0: Initial 2
    // 1: Generating 1s
    // 2: Generating separator 2, switch to next prime

    int state = 0;
    int current_prime = 2;
    int ones_left = 0;

    while (count_beta < target) {
        bool match = false;
        long long q = 0;

        int128 denom1 = C + D;
        int128 denom_inf = C;

        if (denom1 != 0 && denom_inf != 0) {
            long long q1 = floor_div(A + B, denom1);
            long long q_inf = floor_div(A, denom_inf);
            if (q1 == q_inf) {
                match = true;
                q = q1;
            }
        }

        if (match) {
            sum_beta += q;
            count_beta++;

            // Output step
            // M <- [[0, 1], [1, -q]] * M
            //    = [[C, D], [A - qC, B - qD]]
            int128 next_A = C;
            int128 next_B = D;
            int128 next_C = A - (int128)q * C;
            int128 next_D = B - (int128)q * D;

            A = next_A;
            B = next_B;
            C = next_C;
            D = next_D;
        } else {
            // Input step
            int a = 0;
            if (state == 0) {
                a = 2;
                state = 1;
                ones_left = current_prime;
            } else if (state == 1) {
                if (ones_left > 0) {
                    a = 1;
                    ones_left--;
                } else {
                    // Finished ones, output 2
                    state = 2;
                    a = 2;
                }
            } else if (state == 2) {
                // Was separator 2, now find next prime
                int next_p = current_prime + 1;
                while (!is_prime(next_p)) next_p++;
                current_prime = next_p;

                // Start 1s
                state = 1;
                ones_left = current_prime;
                a = 1;
                ones_left--;
            }

            // Update Matrix with input a
            // M <- M * [[a, 1], [1, 0]]
            //    = [[A*a + B, A], [C*a + D, C]]

            int128 next_A = A * a + B;
            int128 next_B = A;
            int128 next_C = C * a + D;
            int128 next_D = C;

            A = next_A;
            B = next_B;
            C = next_C;
            D = next_D;
        }
    }

    std::cout << sum_beta << std::endl;
    return 0;
}
    """

    # Create temporary C++ file
    cpp_file = os.path.join(os.path.dirname(__file__), 'temp_solve.cpp')
    exe_file = os.path.join(os.path.dirname(__file__), 'temp_solve')

    with open(cpp_file, 'w') as f:
        f.write(cpp_source)

    try:
        # Compile
        subprocess.check_call(['g++', '-O3', cpp_file, '-o', exe_file])

        # Run
        result = subprocess.check_output([exe_file]).decode('utf-8').strip()
        print(result)

    finally:
        # Cleanup
        if os.path.exists(cpp_file):
            os.remove(cpp_file)
        if os.path.exists(exe_file):
            os.remove(exe_file)

if __name__ == "__main__":
    solve()
