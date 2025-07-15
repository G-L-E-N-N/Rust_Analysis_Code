#include <iostream>

// Template-Funktion, die zwei Werte vom Typ T vergleicht und den kleineren zurückgibt
template<typename T>
T min_template(T a, T b) {
    return (a < b) ? a : b;
}

int normal_discount(int* non_brand) {
    *non_brand -= 2;
    return *non_brand;
}

int premium_discount(int* brand) {
    *brand -= 5;
    return *brand;
}

int main() {
    int brand = 20;
    int non_brand = 14;

    // Aufruf der Discount-Funktionen mit Zeigern, Rückgabe der veränderten Werte
    int cheaper = min_template(normal_discount(&non_brand), premium_discount(&brand));

    std::cout << "The cheaper one will cost: " << cheaper << std::endl;

    return 0;
}
