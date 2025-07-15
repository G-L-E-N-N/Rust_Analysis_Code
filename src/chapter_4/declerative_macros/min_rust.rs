fn normal_discount(non_brand: &mut i32) -> i32 {
    *non_brand -= 2;
    *non_brand
}

fn premium_discount(brand: &mut i32) -> i32 {
    *brand -= 5;
    *brand
}

macro_rules! min {
    ($a:expr, $b:expr) => {
        if $a < $b { $a } else { $b }
    };
}

fn main() {
    let mut brand = 20;
    let mut non_brand = 14;

    let cheaper = min!(normal_discount(&mut non_brand), premium_discount(&mut brand));

    println!("The cheaper one will cost: {}", cheaper);
}

