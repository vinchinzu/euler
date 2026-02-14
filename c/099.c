#include <stdio.h>
#include <math.h>

int main(void) {
    FILE *f = fopen("../data/0099_base_exp.txt", "r");
    if (!f) { fprintf(stderr, "Cannot open file\n"); return 1; }

    double max_value = 0;
    int max_line = 0;
    int line_num = 1;

    int base, exp;
    char comma;

    while (fscanf(f, "%d%c%d", &base, &comma, &exp) == 3) {
        double value = exp * log((double)base);
        if (value > max_value) {
            max_value = value;
            max_line = line_num;
        }
        line_num++;
    }

    fclose(f);
    printf("%d\n", max_line);
    return 0;
}
