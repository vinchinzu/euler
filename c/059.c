#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(void) {
    FILE *f = fopen("../data/cipher1.txt", "r");
    if (!f) {
        fprintf(stderr, "Error: cannot open ../data/cipher1.txt\n");
        return 1;
    }

    /* Read entire file */
    fseek(f, 0, SEEK_END);
    long fsize = ftell(f);
    fseek(f, 0, SEEK_SET);

    char *buf = (char *)malloc(fsize + 1);
    fread(buf, 1, fsize, f);
    buf[fsize] = '\0';
    fclose(f);

    /* Parse comma-separated integers */
    int cipher[2000];
    int cipher_len = 0;
    char *p = buf;
    while (*p) {
        while (*p == ',' || *p == '\n' || *p == '\r' || *p == ' ') p++;
        if (*p == '\0') break;
        cipher[cipher_len++] = atoi(p);
        while (*p && *p != ',') p++;
    }
    free(buf);

    for (int a = 'a'; a <= 'z'; a++) {
        for (int b = 'a'; b <= 'z'; b++) {
            for (int c = 'a'; c <= 'z'; c++) {
                int key[3] = {a, b, c};

                /* Decrypt */
                char text[2000];
                for (int i = 0; i < cipher_len; i++) {
                    text[i] = (char)(cipher[i] ^ key[i % 3]);
                }
                text[cipher_len] = '\0';

                /* Check for common English words */
                if (strstr(text, " the ") && strstr(text, " and ")) {
                    int sum = 0;
                    for (int i = 0; i < cipher_len; i++) {
                        sum += (unsigned char)text[i];
                    }
                    printf("%d\n", sum);
                    return 0;
                }
            }
        }
    }

    return 0;
}
