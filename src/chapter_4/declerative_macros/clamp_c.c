#include <stdio.h>

#define MIN(a, b) ((a) < (b) ? (a) : (b))

int membership_discount(int price) {
    printf("Calculating membership discount\n");
    price = price - 2;
    return price; // z. B. 2 Euro Rabatt für Mitglieder
}

int promotion_discount(int price) {
    printf("Calculating promotion discount\n");
    price = price - 5;
    return price; // z. B. 5 Euro Rabatt für Aktion
}

int main() {
    int base_price = 20;

    int final_price = MIN(membership_discount(base_price), promotion_discount(base_price));

    printf("Final price: %d\n", final_price);
    return 0;
}
