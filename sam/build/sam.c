#include <stdio.h>
#include <string.h>
int main() {
    char name[337] = "";
    printf("> ");
    fgets(name, sizeof(name) - 1, stdin);
    size_t len = strcspn(name, "\n");
    name[len] = '\0';
    return 0;
}
int fdi(int a, int b);
double fdf(double a, double b);
