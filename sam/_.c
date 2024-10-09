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

void m() {
    const int n = 0;
    char z[334] = "00";
    fgets(z, sizeof(z) - 1, stdin);
    char *newline0 = strchr(z, '\n');
    if (newline0) *newline0 = '\0';
    printf("%s\n", z);
    
}

int main() {
    char name[337] = "joy";
    fgets(name, sizeof(name) - 1, stdin);
    char *newline0 = strchr(name, '\n');
    if (newline0) *newline0 = '\0';
    printf("%s\n", name);
    m();
    return 0;
}
