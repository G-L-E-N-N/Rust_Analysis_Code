#include <stdio.h>

#define MIN(a, b) ((a) < (b) ? (a) : (b))

int membership_discount(int* price) {
    *price -= 2;
    return *price;
}

int promotion_discount(int* price) {
    *price -= 5;
    return *price;
}

int main() {
    int price = 20;

    int final_price = MIN(membership_discount(&price), promotion_discount(&price));

    printf("Final price: %d\n", final_price);
    printf("Remaining price: %d\n", price);

    return 0;
}
