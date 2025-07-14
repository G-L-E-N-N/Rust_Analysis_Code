#include <iostream>

// Abstract base class
class Shape {
public:
    virtual double area() const = 0;
    virtual ~Shape() = default; // Virtual destructor is necessary!
};

// Derived class
class Circle : public Shape {
    double radius;
public:
    Circle(double r) : radius(r) {}
    double area() const override {
        return 3.14 * radius * radius;
    }
};

int main() {
    Circle circle(5.0);
    std::cout << "Circle area: " << std::fixed << std::setprecision(2)
              << circle.area() << std::endl;
    return 0;
}
