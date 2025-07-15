#include <stdio.h>

extern int add(int a, int b);

int main() {
    int result = add(7, 8);
    printf("7 + 8 = %d\n", result);
    return 0;
}
