mod temperature;
mod fibonacci;

fn main() {

    // Different temperatures [kelvin, celsius, fahrenheit]
    let temp = [0.0, 20.0, 90.0];

    // Temperature conversion
    println!("Kelvin: {}, Celsius: {}, Fahrenheit: {}", temp[0], temperature::kelvin_to_celsius(temp[0]), temperature::kelvin_to_fahrenheit(temp[0]));
    println!("Kelvin: {}, Celsius: {}, Fahrenheit: {}", temperature::celsius_to_kelvin(temp[1]), temp[1], temperature::celsius_to_fahrenheit(temp[1]));
    println!("Kelvin: {}, Celsius: {}, Fahrenheit: {}", temperature::fahrenheit_to_kelvin(temp[2]), temperature::fahrenheit_to_celsius(temp[2]), temp[2]);

    // nth fibonacci number
    println!("1st fibonacci number: {}", fibonacci::fibonacci(1));
    println!("2nd fibonacci number: {}", fibonacci::fibonacci(2));
    println!("6th fibonacci number: {}", fibonacci::fibonacci(6));
    println!("20th fibonacci number: {}", fibonacci::fibonacci(20));
}
