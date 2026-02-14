#include <stdio.h>

#define LIMIT 10000000

int next_number(int n) {
    int sum = 0;
    while (n > 0) { int d = n % 10; sum += d * d; n /= 10; }
    return sum;
}

int main(void) {
    /* Max digit square sum for 7-digit number: 7*81 = 567 */
    int memo[568];
    for (int i = 1; i <= 567; i++) {
        int n = i;
        while (n != 1 && n != 89) n = next_number(n);
        memo[i] = (n == 89);
    }

    int count = 0;
    for (int i = 1; i < LIMIT; i++) {
        int n = next_number(i);  /* reduces to <= 567 in one step */
        if (memo[n]) count++;
    }

    printf("%d\n", count);
    return 0;
}
