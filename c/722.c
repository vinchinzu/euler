/* Project Euler 722: Slowly converging series.
 * Polynomial polylogarithm approach with floating-point polynomials.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#define MAX_POLY 50

typedef struct {
    double c[MAX_POLY];
    int len;
} Poly;

Poly poly_new(int len) {
    Poly p;
    memset(p.c, 0, sizeof(p.c));
    p.len = len;
    return p;
}

void poly_trim(Poly *p) {
    while (p->len > 1 && fabs(p->c[p->len - 1]) < 1e-15)
        p->len--;
}

Poly poly_derivative(Poly *p) {
    if (p->len <= 1) return poly_new(1);
    Poly r = poly_new(p->len - 1);
    for (int i = 1; i < p->len; i++)
        r.c[i - 1] = p->c[i] * i;
    poly_trim(&r);
    return r;
}

Poly poly_multiply(Poly *a, Poly *b) {
    int newlen = a->len + b->len - 1;
    if (newlen > MAX_POLY) newlen = MAX_POLY;
    Poly r = poly_new(newlen);
    for (int i = 0; i < a->len; i++)
        for (int j = 0; j < b->len && i + j < MAX_POLY; j++)
            r.c[i + j] += a->c[i] * b->c[j];
    poly_trim(&r);
    return r;
}

Poly poly_add(Poly *a, Poly *b) {
    int maxlen = a->len > b->len ? a->len : b->len;
    Poly r = poly_new(maxlen);
    for (int i = 0; i < a->len; i++) r.c[i] += a->c[i];
    for (int i = 0; i < b->len; i++) r.c[i] += b->c[i];
    poly_trim(&r);
    return r;
}

Poly poly_shift_up(Poly *p, int n) {
    Poly r = poly_new(p->len + n);
    if (r.len > MAX_POLY) r.len = MAX_POLY;
    for (int i = 0; i < p->len && i + n < MAX_POLY; i++)
        r.c[i + n] = p->c[i];
    return r;
}

Poly poly_scale(Poly *p, double s) {
    Poly r = poly_new(p->len);
    for (int i = 0; i < p->len; i++)
        r.c[i] = p->c[i] * s;
    return r;
}

double poly_eval(Poly *p, double x) {
    double result = 0.0, power = 1.0;
    for (int i = 0; i < p->len; i++) {
        result += p->c[i] * power;
        power *= x;
    }
    return result;
}

int main() {
    double Q = 1.0 - pow(0.5, 25);
    int K = 15;

    /* num = x, den = 1 - x */
    Poly num = poly_new(2);
    num.c[0] = 0.0; num.c[1] = 1.0;
    Poly den = poly_new(2);
    den.c[0] = 1.0; den.c[1] = -1.0;

    /* Apply f(z) -> zf'(z) K times */
    for (int i = 1; i <= K; i++) {
        Poly num_deriv = poly_derivative(&num);
        Poly nd_times_den = poly_multiply(&num_deriv, &den);
        Poly num_times_i = poly_scale(&num, (double)i);
        Poly sum = poly_add(&nd_times_den, &num_times_i);
        num = poly_shift_up(&sum, 1);
    }

    double ans = 0.0, prev_ans = -1.0;
    for (int i = 1; i <= 1000; i++) {
        double z = pow(Q, (double)i);
        double num_val = poly_eval(&num, z);
        double den_val = pow(1.0 - z, K + 1);
        ans += num_val / den_val;
        if (fabs(ans - prev_ans) < 1e-10) break;
        prev_ans = ans;
    }

    /* Format: scientific notation without + in exponent */
    char buf[64];
    sprintf(buf, "%.12e", ans);
    /* Replace "e+" with "e" and "e-0" with "e-" etc. */
    char out[64];
    int j = 0;
    for (int i = 0; buf[i]; i++) {
        if (buf[i] == 'e' && buf[i+1] == '+') {
            out[j++] = 'e';
            i++; /* skip + */
        } else {
            out[j++] = buf[i];
        }
    }
    out[j] = '\0';
    printf("%s\n", out);
    return 0;
}
