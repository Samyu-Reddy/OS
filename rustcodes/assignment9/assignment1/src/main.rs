// Declare a constant for the freezing point of water in Fahrenheit
const FREEZING_POINT_F: f64 = 32.0;

// Function to convert Fahrenheit to Celsius
fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

// Function to convert Celsius to Fahrenheit
fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}

fn main() {
    // Declare a mutable variable with a temperature in Fahrenheit
    let mut temp_f: f64 = FREEZING_POINT_F;

    // Convert the temperature to Celsius and print the result
    let temp_c = fahrenheit_to_celsius(temp_f);
    println!("{:.2}°F is {:.2}°C", temp_f, temp_c);

    // Use a loop to convert and print the next 5 integer temperatures
    for _ in 0..5 {
        temp_f += 1.0; // Increment the temperature
        let temp_c = fahrenheit_to_celsius(temp_f);
        println!("{:.2}°F is {:.2}°C", temp_f, temp_c);
    }
}
