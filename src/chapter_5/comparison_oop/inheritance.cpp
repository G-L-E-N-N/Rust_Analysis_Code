#include <iostream>
#include <memory>

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
    // Polymorphic usage
    Shape* shape = new Circle(5.0);
    std::cout << "Area: " << shape->area() << std::endl;

    delete shape; // Safe due to virtual destructor

    // Alternative with smart pointers (recommended in modern C++)
    std::unique_ptr<Shape> smartShape = std::make_unique<Circle>(4.0);
    std::cout << "Area (smart pointer): " << smartShape->area() << std::endl;

    return 0;
}
