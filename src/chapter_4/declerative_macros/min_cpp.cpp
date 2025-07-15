#include <iostream>

int normal_discount(int& non_brand) {
    non_brand -= 2;
    return non_brand;
}

int premium_discount(int& brand) {
    brand -= 5;
    return brand;
}

template<typename T>
T min(T a, T b) {
    return (a < b) ? a : b;
}

int main() {
    int brand = 20;
    int non_brand = 14;

    // Die Funktionen werden hier jeweils genau einmal aufgerufen,
    // weil zuerst die Argumente ausgewertet werden
    int cheaper = min(normal_discount(non_brand), premium_discount(brand));

    std::cout << "The cheaper one will cost: " << cheaper << std::endl;

    return 0;
}
