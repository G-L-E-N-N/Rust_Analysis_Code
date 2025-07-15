#include <iostream>

// C++: Using inheritance for abstraction
class Shape {
public:
    virtual double area() const = 0;
    virtual ~Shape() = default;
};

class Circle : public Shape {
    double radius;
public:
    Circle(double r) : radius(r) {}
    double area() const override {
        return 3.14 * radius * radius;
    }
};

// C++: Polymorphism with virtual methods
void print_area(Shape* s) {
    std::cout << "Area: " << s->area() << std::endl;
}

int main() {
    Circle c(5.0);
    print_area(&c); 
    return 0;
}
