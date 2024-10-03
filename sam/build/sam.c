#include <stdio.h>
int fdi(int a, int b) {
    if (b == 0) {
        // Handle error (e.g., return 0 or some error code)
        return 0;
    }
    int result = a / b;
    // Adjust result if a and b have different signs and there's a remainder
    if ((a % b != 0) && ((a < 0) != (b < 0))) {
        result--;
    }
    return result;
}

double fdf(double a, double b) {
    
    if (b == 0.0) {
        return 0.0;
    }
    
    double result = a / b;
    if (result > 0 && result != (int)result) {
        return (int)result; // Truncates towards zero
    }
    if (result < 0 && result != (int)result) {
        return (int)result - 1;
    }
    
    return result;
}

int main() {
    char z[1025] = "joy";
    fgets(z,1025,stdin);
    printf("%s\n", z);
    return 0;
}
