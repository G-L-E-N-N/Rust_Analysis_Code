#include <stdio.h>

#define MIN(a, b) ((a) < (b) ? (a) : (b))

int normal_discount(int* non_brand) {
    *non_brand -= 2;
    return *non_brand;
}

int premium_discount(int* brand) {
    *brand -= 5;
    return *brand;
}

int main() {
   int brand = 20;
   int non_brand = 14;
   
    int cheaper = MIN(normal_discount(&non_brand), premium_discount(&brand));
   
    printf("Final price: %d\n", cheaper);
   
    return 0;
}
