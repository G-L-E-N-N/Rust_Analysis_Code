#include <stdio.h>

#define CLAMP(x, low, high) ((x) < (low) ? (low) : ((x) > (high) ? (high) : (x)))

int get_val(int* x) {
    *x += 1;
    return *x;
}

int main() {
    int val = 0;
    int result = CLAMP(get_val(&val), 2, 5);
    printf("Result: %d, val: %d\n", result, val);
    return 0;
}
