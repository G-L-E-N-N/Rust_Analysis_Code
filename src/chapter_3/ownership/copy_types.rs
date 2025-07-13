pub fn example_copy_simple() {
    let x = 4; // 'x' is stored on the stack with the value 4
    let y = x; // 'x' is copied into 'y', both have their own copy of 4

    println!("x: {}, y: {}", x, y); // Both are still valid
}

pub fn example_copy_temperature() {
    let living_room_temp = 22; // Initial temperature in living room, Copy trait (i32)
    let bedroom_temp = living_room_temp; // Copy living room temperature to bedroom

    println!("Initial living room temperature: {}°C", living_room_temp);
    println!("Initial bedroom temperature: {}°C", bedroom_temp);

    // Turn heater on in bedroom: increase temperature
    let bedroom_temp = heater_on(bedroom_temp);
    println!("Bedroom temperature after heater on: {}°C", bedroom_temp);

    // Turn AC on in living room: decrease temperature
    let living_room_temp = ac_on(living_room_temp);
    println!("Living room temperature after AC on: {}°C", living_room_temp);
}

fn heater_on(temp: i32) -> i32 {
    temp + 3
}

fn ac_on(temp: i32) -> i32 {
    temp - 2
}

fn main() {
    example_copy_simple();
    example_copy_temperature();
}
