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
