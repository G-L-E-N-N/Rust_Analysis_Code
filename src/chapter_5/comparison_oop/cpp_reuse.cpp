// C++: Inheritance-based reuse
class Printer {
public:
    virtual void print() = 0;
};

class PDFPrinter : public Printer {
public:
    void print() override {
        std::cout << "PDF printing\n";
    }
};
