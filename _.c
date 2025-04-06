#include "nulibc.h"
#include <stdio.h>
#include <stdlib.h>
int main(){
i8 age = 0;
while((age != 100)) {
age = age+1.0;
nprintf(1,"%d",age);
}
return 0;
}