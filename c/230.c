/*
 * Project Euler Problem 230: Fibonacci Words
 *
 * D_A = "14159265358979323846264338327950288419716939937510"
 *       "58209749445923078164062862089986280348253421170679"
 * D_B = "82148086513282306647093844609550582231725359408128"
 *       "48111745028410270193852110555964462294895493038196"
 *
 * f(0)=D_A, f(1)=D_B, f(n)=f(n-2)+f(n-1)
 * D(n) = nth digit of the infinite concatenation
 * Answer = sum_{n=0}^{17} 10^n * D((127+19*n) * 7^n)
 */
#include <stdio.h>
#include <string.h>

static const char *A = "1415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679";
static const char *B = "8214808651328230664709384460955058223172535940812848111745028410270193852110555964462294895493038196";

static int D(long long n) {
    long long lens[200];
    int la = 100, lb = 100; /* both strings are 100 chars */
    lens[0] = la;
    lens[1] = lb;
    int k = 1;
    while (lens[k] < n) {
        k++;
        lens[k] = lens[k - 2] + lens[k - 1];
        if (lens[k] > (long long)2e18) break; /* prevent overflow */
    }

    while (k >= 2) {
        if (n <= lens[k - 2]) {
            k -= 2;
        } else {
            n -= lens[k - 2];
            k -= 1;
        }
    }

    if (k == 0)
        return A[n - 1] - '0';
    else
        return B[n - 1] - '0';
}

int main(void) {
    long long ans = 0;
    long long power10 = 1;
    long long power7 = 1;

    for (int n = 0; n < 18; n++) {
        long long pos = (127 + 19 * n) * power7;
        ans += power10 * D(pos);
        power10 *= 10;
        power7 *= 7;
    }

    printf("%lld\n", ans);
    return 0;
}
