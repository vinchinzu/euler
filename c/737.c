/* Project Euler 737: Coin Loops.
 * Simulate coin placement on unit circle and count loops.
 */
#include <stdio.h>
#include <math.h>

#define NLOOPS 2020

int main() {
    double x = 1.0, y = 0.0;
    double last_cy = 0.0;
    int numLoops = 0;

    for (long long k = 2; ; k++) {
        double r2 = x * x + y * y;
        double l = sqrt(1.0 / r2 - 0.25);
        double cx = x / 2.0 - y * l;
        double cy = y / 2.0 + x * l;
        x += (cx - x) / k;
        y += (cy - y) / k;

        if (cy > 0 && last_cy < 0)
            numLoops++;
        last_cy = cy;

        if (numLoops == NLOOPS) {
            printf("%lld\n", k);
            break;
        }
    }
    return 0;
}
