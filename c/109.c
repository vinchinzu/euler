/* Project Euler Problem 109: Darts */
#include <stdio.h>

int main(void) {
    /* Build all possible shots */
    /* Singles: 1-20, plus S25 (outer bull) = 22 shots */
    /* Doubles: D1-D20 (2-40), plus D25 (bull) = 21 shots */
    /* Trebles: T1-T20 (3-60) = 20 shots */
    /* Zero shot (miss) = 1 shot */
    /* Pre-shots: miss + singles + doubles + trebles + S25 + D25 = 1+20+20+20+1+1 = 63 */
    /* Finishing: doubles + D25 = 21 */

    int pre_scores[64]; /* max 63 pre-shots + zero */
    int pre_count = 0;

    /* Zero shot */
    pre_scores[pre_count++] = 0;
    /* Singles 1-20 */
    for (int i = 1; i <= 20; i++) pre_scores[pre_count++] = i;
    /* Doubles 1-20 */
    for (int i = 1; i <= 20; i++) pre_scores[pre_count++] = 2 * i;
    /* Trebles 1-20 */
    for (int i = 1; i <= 20; i++) pre_scores[pre_count++] = 3 * i;
    /* Outer bull (S25) */
    pre_scores[pre_count++] = 25;
    /* Inner bull (D25) */
    pre_scores[pre_count++] = 50;

    int finish_scores[22];
    int finish_count = 0;
    /* Doubles 1-20 */
    for (int i = 1; i <= 20; i++) finish_scores[finish_count++] = 2 * i;
    /* D25 */
    finish_scores[finish_count++] = 50;

    int limit = 99;
    int total = 0;

    for (int f = 0; f < finish_count; f++) {
        for (int i = 0; i < pre_count; i++) {
            for (int j = i; j < pre_count; j++) {
                int score = pre_scores[i] + pre_scores[j] + finish_scores[f];
                if (score >= 1 && score <= limit)
                    total++;
            }
        }
    }

    printf("%d\n", total);
    return 0;
}
