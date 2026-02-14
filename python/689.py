"""Project Euler Problem 689: Binary Series.

Embedded C solution for speed. Enumerate all 2^22 subsets, compute partial sums
of 1/i^2, use erf approximation for tail probability. Output 8 decimal places.
"""
import subprocess, tempfile, os, sys

C_CODE = r"""
#include <stdio.h>
#include <math.h>

static double my_erf(double z) {
    if (z > 5.0) return 1.0;
    if (z < -5.0) return -1.0;
    double az = fabs(z);
    double a1 = 0.254829592;
    double a2 = -0.284496736;
    double a3 = 1.421413741;
    double a4 = -1.453152027;
    double a5 = 1.061405429;
    double p = 0.3275911;
    double t = 1.0 / (1.0 + p * az);
    double val = 1.0 - (((((a5*t + a4)*t) + a3)*t + a2)*t + a1) * t * exp(-z*z);
    return z >= 0 ? val : -val;
}

int main(void) {
    int L = 22;
    double N = 0.5;

    /* Precompute 1/i^2 for i=1..L */
    double inv_sq[23];
    for (int i = 1; i <= L; i++)
        inv_sq[i] = 1.0 / ((double)i * (double)i);

    /* Compute mean and stddev of the tail (i > L) */
    double pi2_6 = M_PI * M_PI / 6.0;
    double pi4_90 = M_PI * M_PI * M_PI * M_PI / 90.0;
    double mean = pi2_6;
    double var = pi4_90;
    for (int i = 1; i <= L; i++) {
        mean -= inv_sq[i];
        var -= 1.0 / ((double)i * (double)i * (double)i * (double)i);
    }
    mean /= 2.0;
    double stddev = sqrt(var / 2.0);

    double ans = 0.0;
    int total = 1 << L;
    for (int subset = 0; subset < total; subset++) {
        double sum_val = 0.0;
        int s = subset;
        while (s) {
            int bit = __builtin_ctz(s);
            sum_val += inv_sq[bit + 1];
            s &= s - 1;
        }
        ans += (1.0 - my_erf((N - mean - sum_val) / stddev)) / 2.0;
    }
    ans /= (double)total;

    printf("%.8f\n", ans);
    return 0;
}
"""

def main():
    with tempfile.NamedTemporaryFile(suffix='.c', mode='w', delete=False) as f:
        f.write(C_CODE)
        c_path = f.name
    bin_path = c_path.replace('.c', '')
    try:
        subprocess.run(['gcc', '-O2', '-o', bin_path, c_path, '-lm'],
                       check=True, capture_output=True)
        result = subprocess.run([bin_path], capture_output=True, text=True,
                                timeout=280)
        print(result.stdout.strip())
    finally:
        for p in [c_path, bin_path]:
            if os.path.exists(p):
                os.unlink(p)

if __name__ == "__main__":
    main()
