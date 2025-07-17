// C code: dot_product.c
#include <stddef.h>

float dot_product(const float* a, const float* b, size_t len) {
    float sum = 0.0;
    for (size_t i = 0; i < len; i++) {
        sum += a[i] * b[i];
    }
    return sum;
}
