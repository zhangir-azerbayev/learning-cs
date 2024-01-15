#include <stdio.h>

main() {
    int c, prev_blank;

    prev_blank = 0;

    while ((c = getchar()) != EOF) {
        if (c == ' ') {
            prev_blank = 1;
        } else {
            if (prev_blank == 1)
                putchar(' ');
            putchar(c);
            prev_blank = 0;
        }
    }
}
