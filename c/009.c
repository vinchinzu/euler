#include <stdio.h>
#include <math.h>

int main(void) {
    for (int x = 1; x < 334; x++) {
        for (int y = x + 1; y < 667; y++) {
            unsigned long long c2 = (unsigned long long)x * x + (unsigned long long)y * y;
            unsigned long long c = (unsigned long long)sqrt((double)c2);
            if (c * c == c2 && x + y + (int)c == 1000) {
                printf("%llu\n", (unsigned long long)x * y * c);
                return 0;
            }
        }
    }
    return 0;
}
