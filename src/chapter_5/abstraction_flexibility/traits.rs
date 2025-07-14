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

struct Robot {
    serial: String,
}

impl Greet for Robot {} // Uses default implementation


fn main() {
    let alice = Person {
        name: String::from("Alice"),
        id: 1001,
    };

    let r2d2 = Robot {
        serial: String::from("R2D2-7788"),
    };

    // Greet trait
    println!("{}", alice.greet()); // Uses overridden implementation
    println!("{}", r2d2.greet());  // Uses default implementation

    // Identify trait
    println!("{}", alice.id());    // Person implements Identify
    // println!("{}", r2d2.id()); // ERROR: Robot does not implement Identify
}
