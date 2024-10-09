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
    char name[337] = "";
    printf("Hello and welcome to OxumLabs!\n");
    printf("username > ");
    fgets(name, sizeof(name) - 1, stdin);
    for (int i = 0; name[i] != '\0'; i++) {
        if (name[i] == '\n') name[i] = '\0';
    }
    printf("name is %s\n", name);
    if (strcmp(name, "buj")== 0) {
        printf("Hello  %s , shall I fetch the Neit Git repository?\n", name);
    }
    else if (strcmp(name, "bilal")== 0) {
        printf("Hello  %s , shall I fetch the Neit Git repository?\n", name);
    }
    else if (strcmp(name, "buj")==0) {
        printf("Hello  %s , shall I fetch the Neit Git repository?\n", name);
    }
    else if (strcmp(name, "joy")==0) {
        printf("Hello  %s , shall I fetch the Neit Git repository?\n", name);
    }
    else {
        printf("Hello %s, shall I download and run Neit?\n", name);
        
    }
    printf("Thank you for using OxumLabs!\n");
    return 0;
}
