#include <stdio.h>
#include <string.h>
int fdi(int a, int b) {
    if (b == 0) {
        // ✘ Error: Division by zero
        // → Hint: Handle error (e.g., return 0 or some error code)
        return 0;
    }
    int result = a / b;
    // ℹ Info: Adjust result if a and b have different signs and there's a remainder
    if ((a % b != 0) && ((a < 0) != (b < 0))) {
        result--;
    }
    return result;
}

double fdf(double a, double b) {
    
    if (b == 0.0) {
        // ✘ Error: Division by zero in float
        return 0.0;
    }
    
    double result = a / b;
    if (result > 0 && result != (int)result) {
        return (int)result; // ℹ Info: Truncates towards zero
    }
    if (result < 0 && result != (int)result) {
        return (int)result - 1;
    }
    
    return result;
}

void hi(char *name) {

}

int main() {
    return 0;
}
