#include <stdio.h>

float celsius_of_fahr(float fahr)
{
    return (5.0/9.0) * (fahr - 32.0);
}

int main()
{
    float fahr, celsius;
    int lower, upper, step;

    lower = 0;
    upper = 300;
    step = 20;

    fahr = lower;

    printf("Conversion Table: Fahrenheit to Celsius\n\n");

    printf("%10s %7s\n", "Fahrenheit", "Celsius");

    while (fahr <= upper) {
        celsius = celsius_of_fahr(fahr);
        printf("%10.0f %7.1f\n", fahr, celsius);
        fahr = fahr + step;
    }
}
