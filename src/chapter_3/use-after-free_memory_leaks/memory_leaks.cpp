#include <iostream>

void memoryLeak() {
    int* ptr = new int(10); // Dynamically allocate memory
    // No delete --> memory leak
}

int main() {
    memoryLeak(); // Memory leak occurs here
    return 0;
}
