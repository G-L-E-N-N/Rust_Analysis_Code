#include <iostream>

int val = 0;

int get_val() {
    val = val + 1;
    return val;
}

template<typename T>
T clamp(T x, T low, T high) {
    return x < low ? low : (x > high ? high : x);
}

int main() {
    int result = clamp(get_val(), 2, 5);
    std::cout << "Result: " << result << ", val: " << val << std::endl;

    // double d = clamp(get_val(), 2.0, 5.0); // Error: conflicting types
}
