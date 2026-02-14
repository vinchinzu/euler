/*
 * Project Euler 040 - Champernowne's Constant
 * Find the product of digits d_1 * d_10 * d_100 * d_1000 * d_10000 * d_100000 * d_1000000
 * of the fractional part of Champernowne's constant.
 */
#include <stdio.h>

int main(void) {
    int positions[] = {1, 10, 100, 1000, 10000, 100000, 1000000};
    int num_positions = 7;
    int result_digits[7];
    int found = 0;

    int current_length = 0;
    int number = 1;
    int next_pos_index = 0;

    while (next_pos_index < num_positions) {
        /* Count digits in number */
        int num_len = 0;
        int tmp = number;
        while (tmp > 0) { num_len++; tmp /= 10; }

        if (current_length + num_len >= positions[next_pos_index]) {
            while (next_pos_index < num_positions &&
                   current_length + num_len >= positions[next_pos_index]) {
                int offset = positions[next_pos_index] - current_length - 1;

                /* Extract digit at position 'offset' in the string representation of number */
                char buf[16];
                sprintf(buf, "%d", number);
                result_digits[next_pos_index] = buf[offset] - '0';
                next_pos_index++;
            }
        }

        current_length += num_len;
        number++;
    }

    int product = 1;
    for (int i = 0; i < num_positions; i++) {
        product *= result_digits[i];
    }

    printf("%d\n", product);
    return 0;
}
