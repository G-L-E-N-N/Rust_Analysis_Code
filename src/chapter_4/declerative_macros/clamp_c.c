#include <stdio.h>

int get_value(int *counter) {
    (*counter) += 1;
    return *counter; // Rückgabe ist aktueller Wert nach Inkrement
}

#define MAX(a, b) ((a) > (b) ? (a) : (b))

int main() {
    int count = 0;
    int result = MAX(get_value(&count), 3);
    printf("Result: %d\n", result);
    printf("Call count: %d\n", count);
    return 0;
}
