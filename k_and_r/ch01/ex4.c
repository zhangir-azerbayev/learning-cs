#include <stdio.h>

int main()
{
    int fahr, celsius;
    int lower, upper, step;

    lower = -20;
    upper = 150;
    step = 10;

    celsius = lower;

    printf("Conversion Table: Celsius to Fahrenheit\n\n");

    printf("%10s %7s\n", "Celsius", "Fahrenheit");

    while (celsius <= upper) {
        fahr = 32 + (9 * celsius / 5);
        printf("%10d %7d\n", celsius, fahr);
        celsius = celsius + step;
    }
}
