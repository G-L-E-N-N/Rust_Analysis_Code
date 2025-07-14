// Rust: Reuse via composition and generics
trait Printer {
    fn print(&self);
}

struct PDFPrinter;

impl Printer for PDFPrinter {
    fn print(&self) {
        println!("PDF printing");
    }
}

fn main() {
    let printer = PDFPrinter;
    printer.print(); // Call via concrete instance

    let dynamic_printer: &dyn Printer = &printer;
    dynamic_printer.print(); // Call via trait object (dynamic dispatch)
}
