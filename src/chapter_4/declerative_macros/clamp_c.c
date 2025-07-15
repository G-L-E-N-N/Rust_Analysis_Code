#include <stdio.h>

#define MIN(a, b) ((a) < (b) ? (a) : (b))

void apply_membership_discount(int* price) {
    printf("Applying membership discount\n");
    *price -= 2;
}

void apply_promotion_discount(int* price) {
    printf("Applying promotion discount\n");
    *price -= 5;
}

int membership_discount(int* price) {
    apply_membership_discount(price);
    return *price;
}

int promotion_discount(int* price) {
    apply_promotion_discount(price);
    return *price;
}

int main() {
    int base_price1 = 20;
    int base_price2 = 20;

    int final_price = MIN(membership_discount(&base_price1), promotion_discount(&base_price2));

    printf("Final price: %d\n", final_price);
    printf("Aftermath: base_price1 = %d, base_price2 = %d\n", base_price1, base_price2);

    return 0;
}
