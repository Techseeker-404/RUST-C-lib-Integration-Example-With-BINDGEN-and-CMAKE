#include <stdio.h>
#include "libsimplemath/sum.h"
#include "libsimplemath/difer.h"


int main() {
    int a = 20;
    int b = 15;

    printf("sum of a and b %d \n", add(a,b));
    printf("difference of a and b %d \n", subtract(a, b));

    return 0;
}
