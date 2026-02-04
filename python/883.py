import subprocess, os, sys, tempfile

def solve():
    c_code = r"""
#include <stdio.h>
#include <math.h>

int gcd_func(int a, int b) {
    while (b) { int t = b; b = a % b; a = t; }
    return a;
}

int main() {
    double N = 1000000.0;
    long long ans = 0;
    int two_N = 2000000;

    for (int n = 1; n <= two_N; n++) {
        int max_md = two_N / n;
        if (max_md < 1) break;
        for (int md = 1; md <= max_md; md++) {
            int m = n + md;
            int r = m % n;
            if (n > 1 && (r == 0 || gcd_func(r, n % r) != 1)) continue;

            double l = 2.0 * N / ((double)n * md);
            double four_l_sq = 4.0 * l * l;
            int mult = (m + n == 3) ? 2 : 6;
            int mod3_flag = (n + m) % 3;

            for (long long x = 1; 3.0 * x * x <= four_l_sq; x++) {
                double disc_sq_d = four_l_sq - 3.0 * (double)(x * x);
                long long disc_sq_floor = (long long)floor(disc_sq_d);
                long long disc = (long long)sqrt((double)disc_sq_floor);
                while ((disc+1)*(disc+1) <= disc_sq_floor) disc++;
                while (disc*disc > disc_sq_floor) disc--;

                long long xp1h = (x + 1) / 2;
                long long xdh = (x - disc) / 2;
                long long min_y = (xp1h > xdh ? xp1h : xdh) - 1;
                long long max_y_a = 2 * x - 1;
                long long max_y_b = (x + disc) / 2;
                long long max_y = (max_y_a < max_y_b ? max_y_a : max_y_b);

                if (mod3_flag > 0) {
                    min_y = (x + min_y) / 3;
                    max_y = (x + max_y) / 3;
                }
                if (max_y > min_y) {
                    ans += (max_y - min_y) * (long long)mult;
                }
            }
        }
    }
    printf("%lld\n", ans);
    return 0;
}
"""
    d = os.path.dirname(os.path.abspath(__file__))
    src = os.path.join(d, "_883_tmp.c")
    exe = os.path.join(d, "_883_tmp")
    with open(src, "w") as f:
        f.write(c_code)
    subprocess.run(["gcc", "-O2", "-lm", src, "-o", exe], check=True,
                   capture_output=True)
    result = subprocess.run([exe], capture_output=True, text=True, check=True)
    os.unlink(src)
    os.unlink(exe)
    print(result.stdout.strip())

if __name__ == "__main__":
    solve()
