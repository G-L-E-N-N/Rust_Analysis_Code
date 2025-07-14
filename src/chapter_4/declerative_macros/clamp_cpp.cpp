#include <iostream>

int foo(int& x) {
    x++;
    std::cout << "foo called, x = " << x << "\n";
    return x;
}

int bar(int& y) {
    y += 2;
    std::cout << "bar called, y = " << y << "\n";
    return y;
}

template<typename T>
T min(T x, T y) {
    return (x < y) ? x : y;
}

int main() {
    int a = 5;
    int b = 5;
    int result = min(foo(a), bar(b));
    std::cout << "Result: " << result << "\n";
    std::cout << "Final a: " << a << ", Final b: " << b << "\n";
}
