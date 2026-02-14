#include <stdio.h>
#include <stdbool.h>
#include <string.h>

void get_sorted_digits(int n, char *out) {
    char buf[16];
    sprintf(buf, "%d", n);
    int len = (int)strlen(buf);
    /* Bubble sort */
    for (int i = 0; i < len - 1; i++) {
        for (int j = i + 1; j < len; j++) {
            if (buf[j] < buf[i]) {
                char tmp = buf[i]; buf[i] = buf[j]; buf[j] = tmp;
            }
        }
    }
    strcpy(out, buf);
}

int main(void) {
    for (int x = 1; ; x++) {
        char base[16];
        get_sorted_digits(x, base);

        bool all_same = true;
        for (int mult = 2; mult <= 6; mult++) {
            char other[16];
            get_sorted_digits(x * mult, other);
            if (strcmp(base, other) != 0) {
                all_same = false;
                break;
            }
        }

        if (all_same) {
            printf("%d\n", x);
            return 0;
        }
    }
    return 0;
}
