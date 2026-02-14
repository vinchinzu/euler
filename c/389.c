/*
 * Project Euler Problem 389: Platonic Dice
 *
 * Hierarchical dice variance computation.
 * Var(sum of random N i.i.d. dice) = E[N]*Var(X) + E[X]^2*Var(N)
 */
#include <stdio.h>

int main(void) {
    /* T stage: 4-sided die */
    double mu_t = 2.5;      /* (1+4)/2 */
    double var_t = 1.25;    /* (4^2-1)/12 = 15/12 = 1.25 */

    /* C stage: 6-sided dice */
    double mu_6 = 3.5;
    double var_6 = 35.0 / 12.0;  /* (36-1)/12 */
    double e_c = mu_6 * mu_t;
    double var_c = var_6 * mu_t + mu_6 * mu_6 * var_t;

    /* O stage: 8-sided dice */
    double mu_8 = 4.5;
    double var_8 = 63.0 / 12.0;  /* (64-1)/12 */
    double e_o = mu_8 * e_c;
    double var_o = var_8 * e_c + mu_8 * mu_8 * var_c;

    /* D stage: 12-sided dice */
    double mu_12 = 6.5;
    double var_12 = 143.0 / 12.0; /* (144-1)/12 */
    double e_d = mu_12 * e_o;
    double var_d = var_12 * e_d + mu_12 * mu_12 * var_o;

    /* I stage: 20-sided dice */
    double mu_20 = 10.5;
    double var_20 = 399.0 / 12.0; /* (400-1)/12 */
    /* double e_i = mu_20 * e_d; -- not needed */
    double var_i = var_20 * e_d + mu_20 * mu_20 * var_d;

    printf("%.4f\n", var_i);
    return 0;
}
