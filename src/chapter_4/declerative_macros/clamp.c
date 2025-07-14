#include <stdio.h>

int val = 0;

#define CLAMP(x, low, high) ((x) < (low) ? (low) : ((x) > (high) ? (high) : (x)))

int get_val() {
    val = val + 1;   // Side effect: increment global val
    return val;
}

int main() {
    int result = CLAMP(get_val(), 2, 5);
    printf("Result: %d, val: %d\n", result, val);
    return 0;
}
