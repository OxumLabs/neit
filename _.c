#include "nulibc.h"
#include <stdio.h>
#include <stdlib.h>
int main(){
nstring name = nstr_new("joy");
nprintf(1,"\n",name);
return 0;
}