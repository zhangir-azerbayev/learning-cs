#include <stdio.h>

#define     IN      0
#define     OUT     1
#define     MAX_LENGTH      10
#define     CELL_WIDTH      4

main ()
{
    int c, state, length; 
    int freq[MAX_LENGTH + 1];

    int i;
    for (i = 1; i <= MAX_LENGTH; ++i) 
        freq[i] = 0;

    state = OUT;
    length = 0;

    while ((c = getchar()) != EOF) {
        if (c == ' ' || c == '\n' || c == '\t') {
            if (state == IN) {
                ++freq[length]; 
            }
            length = 0;
        } else {
            if (length < MAX_LENGTH)
                ++length;
            state = IN;
        }
    }
    
    printf("Word length frequencies\n");
    for (i = 1; i <= MAX_LENGTH; ++i) {
        printf("%d: %d\n", i, freq[i]);
    }
}

/*
    __
__ |  |
  ||  |
  ||  | __    __
__||__||__|__|__|
 */
