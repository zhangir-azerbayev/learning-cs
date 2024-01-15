#include <stdio.h>

int determine_max_char()
{
    unsigned char x;
    unsigned char prev_x;

    x = prev_x = 0;
    while (1) {
        x++;
        if (prev_x > x) {
            return prev_x;
        }
        prev_x = x;
    }
}

void determine_min_float()
{
    float min_float;
    float prev_x = 1.0;
    float x = 1.0;

    while (x != 0) {
        prev_x = x;
        x = x/2;
    }
    min_float = prev_x;
    printf("smallest float is %e", min_float);
}


main() {
    int x = determine_max_char();
    printf("max char is %d\n", x);

    determine_min_float();
}
