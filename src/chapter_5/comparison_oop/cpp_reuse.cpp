#include <iostream>

// C++: Inheritance-based reuse
class Printer {
public:
    virtual void print() = 0;
    virtual ~Printer() = default;
};

class PDFPrinter : public Printer {
public:
    void print() override {
        std::cout << "PDF printing\n";
    }
};

int main() {
    PDFPrinter pdf;
    pdf.print();

    // Polymorph:
    Printer* printer = new PDFPrinter();
    printer->print();
    delete printer;

    return 0;
}
