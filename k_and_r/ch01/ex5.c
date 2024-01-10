#include <stdio.h>

#define     LOWER   0
#define     UPPER   300
#define     STEP    20

int main()
{
    printf("Conversion Table: Fahrenheit to Celsius\n\n");

    printf("%10s %7s\n", "Fahrenheit", "Celsius");

    int fahr;
    float celsius;

    for (fahr=UPPER; fahr>=LOWER; fahr = fahr - STEP) {
        celsius = (5.0/9.0) * (fahr - 32.0);
        printf("%10d %7.1f\n", fahr, celsius);
    }
}
