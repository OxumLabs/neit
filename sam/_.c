#include <stdio.h>
#include <string.h>
int fdi(int a, int b);
double fdf(double a, double b);
int main() {
    char name[337] = "";
    printf("> ");
    fgets(name, sizeof(name) - 1, stdin);
    size_t len = strcspn(name, "\n");
    name[len] = '\0';
    if (strcmp(name,"joy") ==0) {
        printf("Hello joy!\n");
    }
    else {
        printf("Hello %s!\n", name);
        
    }
    return 0;
}
