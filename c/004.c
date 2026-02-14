#include <stdio.h>

int is_palindrome(int n) {
    int rev = 0, orig = n;
    while (n > 0) { rev = rev * 10 + n % 10; n /= 10; }
    return rev == orig;
}

int main(void) {
    int best = 0;
    for (int x = 999; x >= 100; x--)
        for (int y = x; y >= 100; y--) {
            int p = x * y;
            if (p <= best) break;
            if (is_palindrome(p)) best = p;
        }
    printf("%d\n", best);
    return 0;
}
