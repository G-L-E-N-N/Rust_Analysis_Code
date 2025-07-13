#include <iostream>

void use_after_free() {
    int* ptr = new int(42);    // dynamically allocate memory
    delete ptr;                // deallocate memory

    std::cout << *ptr;         // use-after-free: undefined behavior
}

int main() {
    use_after_free();
    return 0;
}
