// Duplicate traits etc. from traits.rs

trait Greet {
    fn greet(&self) -> String {
        String::from("Hi, I'm a generic entity.")
    }
}

trait Identify {
    fn id(&self) -> String;
}

struct Person {
    name: String,
    id: u32,
}

struct Robot {
    serial: String,
}

impl Greet for Person {
    fn greet(&self) -> String {
        format!("Hello, my name is {}", self.name)
    }
}

impl Identify for Person {
    fn id(&self) -> String {
        format!("ID: {}", self.id)
    }
}

impl Greet for Robot {}

// New generics code:

fn print_greeting<T: Greet>(entity: &T) {
    println!("{}", entity.greet());
}

fn print_greeting_dyn(entity: &dyn Greet) {
    println!("{}", entity.greet());
}

fn show_identity_and_greet<T: Greet + Identify>(entity: &T) {
    println!("{}, my ID is {}", entity.greet(), entity.Identify());
}

fn main() {
    let person = Person {
        name: String::from("Bob"),
        id: 99,
    };

    let robot = Robot {
        serial: String::from("T-800"),
    };

    println!("--- Generic Trait Bound ---");
    print_greeting(&person);
    print_greeting(&robot);

    println!("--- Dynamic Dispatch ---");
    print_greeting_dyn(&person);
    print_greeting_dyn(&robot);

    println!("--- Combined Traits ---");
    show_identity_and_greet(&person);
    // show_identity_and_greet(&robot); // Compile error: Robot doesn't implement Identify
}
