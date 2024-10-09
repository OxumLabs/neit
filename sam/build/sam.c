#include <stdio.h>
#include <string.h>
int fdi(int a, int b) {
    if (b == 0) {
        return 0; // Error: Division by zero
    }
    int result = a / b;
    if ((a % b != 0) && ((a < 0) != (b < 0))) {
        result--;
    }
    return result;
}

double fdf(double a, double b) {
    if (b == 0.0) {
        return 0.0; // Error: Division by zero in float
    }
    double result = a / b;
    return (result > 0 && result != (int)result) ? (int)result : (result < 0 && result != (int)result) ? (int)result - 1 : result;
}

int main() {
    const char *name = "joy";
    if (strcmp(name,"joy"),0) {
        printf("hello sir\n");
    }
    else if (strcmp(name,"bilal"),0) {
        printf("hello sir\n");
    }
    else if (strcmp(name,"buj"),0) {
        printf("hello sir\n");
    }
    else {
        printf("hello sir\n");
        
    }
    return 0;
}
