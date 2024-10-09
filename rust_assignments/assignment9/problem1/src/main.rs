fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}


fn main() {
    
    let mut fahrenheit_temp: f64 = 32.0;

   
    let celsius_temp = fahrenheit_to_celsius(fahrenheit_temp);
    println!("{:.2}°F is {:.2}°C", fahrenheit_temp, celsius_temp);


    for _ in 1..=5 {
        fahrenheit_temp += 1.0;
        let celsius_temp = fahrenheit_to_celsius(fahrenheit_temp);
        println!("{:.2}°F is {:.2}°C", fahrenheit_temp, celsius_temp);
    }
}
