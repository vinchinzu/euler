#include <stdio.h>

long long fib(void) {
    long long a = 1;
    long long b = 2;
    long long sum = 0;
    while (b < 4000000) {
        if (b % 2 == 0) {
            sum += b;
        }
        long long temp = a + b;
        a = b;
        b = temp;
    }
    return sum;
}

int main(void) {
    printf("%lld\n", fib());
    return 0;
}
