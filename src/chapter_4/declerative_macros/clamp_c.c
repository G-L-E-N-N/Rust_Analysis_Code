#include <stdio.h>

int foo(int* x) {
    (*x)+=1;
    printf("foo called, x is now %d\n", *x);
    return *x;
}

int bar(int* y) {
    (*y) += 2;
    printf("bar called, y is now %d\n", *y);
    return *y;
}

#define min(X, Y) ((X) < (Y) ? (X) : (Y))

int main() {
    int a = 5;
    int b = 5;
    
    int result = min(foo(&a), bar(&b));
    
    printf("Result: %d\n", result);
    printf("Final a: %d\n", a);
    printf("Final b: %d\n", b);
    
    return 0;
}
