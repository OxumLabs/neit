#include <stdio.h>
#include <math.h>


int fdi(int a, int b) {
    return a / b - ((a % b) < 0);
}

double fdf(double a, double b) {
    return floor(a / b);
}
void welcome(char *name, int age) {
    printf("Hello nice to meet you %s! , i see you are %d years old!\nyou will be   %d years old next year!\n", name, age, age+1);
    printf("%d\n", fdf(2, 2));
    
}

int main() {
    char *name = "jay";
    int age = 16;
    welcome(name,16);
    return 0;
}
