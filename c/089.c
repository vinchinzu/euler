#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int roman_char_value(char c) {
    switch (c) {
        case 'I': return 1;
        case 'V': return 5;
        case 'X': return 10;
        case 'L': return 50;
        case 'C': return 100;
        case 'D': return 500;
        case 'M': return 1000;
    }
    return 0;
}

int roman_to_int(const char *roman) {
    int total = 0;
    int prev_value = 0;
    int len = (int)strlen(roman);
    for (int i = len - 1; i >= 0; i--) {
        int value = roman_char_value(roman[i]);
        if (value < prev_value)
            total -= value;
        else
            total += value;
        prev_value = value;
    }
    return total;
}

int int_to_roman_len(int num) {
    int values[] = {1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1};
    int symbol_lens[] = {1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1};
    int len = 0;
    for (int i = 0; i < 13; i++) {
        while (num >= values[i]) {
            len += symbol_lens[i];
            num -= values[i];
        }
    }
    return len;
}

int main(void) {
    FILE *f = fopen("../data/0089_roman.txt", "r");
    if (!f) { fprintf(stderr, "Cannot open file\n"); return 1; }

    int total_saved = 0;
    char line[256];

    while (fgets(line, sizeof(line), f)) {
        /* Strip newline/carriage return */
        int len = (int)strlen(line);
        while (len > 0 && (line[len-1] == '\n' || line[len-1] == '\r'))
            line[--len] = '\0';
        if (len == 0) continue;

        int value = roman_to_int(line);
        int minimal_len = int_to_roman_len(value);
        total_saved += len - minimal_len;
    }

    fclose(f);
    printf("%d\n", total_saved);
    return 0;
}
