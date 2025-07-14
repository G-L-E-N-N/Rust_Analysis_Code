#include <stdio.h>

int foo(int *z) {
    (*z)++;
    printf("foo called, z now = %d\n", *z);
    return *z;
}

#define min(X, Y)  ((X) < (Y) ? (X) : (Y))

int main() {
    int z = 5;
    int x = 2;
    int y = 3;

    int next = min(x + y, foo(&z));  // foo wird evtl. 2x aufgerufen!

    printf("next = %d\n", next);
    printf("z = %d\n", z);

    return 0;
}
