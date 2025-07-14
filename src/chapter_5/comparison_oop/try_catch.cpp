// C++: Exception-based error handling
#include <iostream>
#include <stdexcept>

double divide(double a, double b) {
    if (b == 0) throw std::runtime_error("Divide by zero");
    return a / b;
}

int main() {
    try {
        double result = divide(10, 0);
        std::cout << "Result: " << result << "\n";
    } catch (const std::runtime_error& e) {
        std::cerr << "Error: " << e.what() << "\n";
    }
}
