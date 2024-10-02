#include <stdio.h>
#include <math.h>


int fdi(int a, int b) {
    return a / b - ((a % b) < 0);
}

double fdf(double a, double b) {
    return floor(a / b);
}
int main() {
    int z = 100;
    printf("%f\n", z/3);
    return 0;
}
