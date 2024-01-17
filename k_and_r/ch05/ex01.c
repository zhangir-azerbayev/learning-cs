#include <ctype.h>
#include "getch.c"

int getint(int *pn)
{
    int c, sign, buffer;
    int found_digit = 0;

    while (isspace(c=getch()))
        ;

    if (!isdigit(c) && c != EOF && c != '+' && c != '-') {
        ungetch(c);
        return 0;
    }

    sign = (c == '-') ? -1 : 1;

    if (c == '+' || c == '-') {
        c = getch();
        if (!isdigit(c)) {
            ungetch(c);
            ungetch((sign==1) ? '+' : '-');
            return 0;
        }
    }
    for (*pn = 0; isdigit(c); c = getch()) {
        *pn = 10 * *pn +(c - '0');
    }

    *pn *= sign;
    if (c != EOF)
        ungetch(c);

    return c;
}

main()
{
    int c = 1337;

    int i;
    for (i = 0; i < 1; i++) {
        printf("exit code: %d\n", getint(&c));
        printf("*pn: %d\n\n", c);
    }

    printf("%c", getch());
}
