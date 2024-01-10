#include <stdio.h>

int main()
{
    int c;

    while ((c = getchar()) != EOF) {
        printf("The value of getchar() != EOF is %u\n", c != EOF);
    }
    printf("The value of getchar() != EOF is %u\n", c != EOF);
}
