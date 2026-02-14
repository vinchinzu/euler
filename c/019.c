#include <stdio.h>
#include <stdbool.h>

bool is_leap(int year) {
    return (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;
}

int days_in_month(int year, int month) {
    int days[] = {31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31};
    if (month == 2 && is_leap(year)) return 29;
    return days[month - 1];
}

int main(void) {
    int dow = 2; /* Jan 1, 1901 is Tuesday (0=Sunday) */
    /* Starting from Jan 1 1900 which is Monday (dow=1), advance through 1900 */
    /* The C++ code starts with dow=2 for Jan 1 1901, so we match that */
    int count = 0;
    for (int year = 1901; year <= 2000; year++) {
        for (int month = 1; month <= 12; month++) {
            if (dow == 0) count++;
            dow = (dow + days_in_month(year, month)) % 7;
        }
    }
    printf("%d\n", count);
    return 0;
}
