use std::io::{self, Write};

// Function to convert Fahrenheit to Celsius
fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

// Function to convert Celsius to Fahrenheit
fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}

// Function to get temperature input from user
fn get_temperature_input(temp_type: &str) -> f64 {
    loop {
        print!("Enter {} temperature: ", temp_type);
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
            
        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Please enter a valid number!"),
        }
    }
}

fn main() {
    println!("Welcome to Temperature Converter!");
    println!("--------------------------------");
    
    loop {
        println!("\nPlease select an option:");
        println!("1. Convert Fahrenheit to Celsius");
        println!("2. Convert Celsius to Fahrenheit");
        println!("3. Exit");
        
        print!("Enter your choice (1-3): ");
        io::stdout().flush().unwrap();
        
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
            
        match choice.trim().parse() {
            Ok(1) => {
                let fahrenheit = get_temperature_input("Fahrenheit");
                let celsius = fahrenheit_to_celsius(fahrenheit);
                println!("{:.1}°F is equal to {:.1}°C", fahrenheit, celsius);
            },
            Ok(2) => {
                let celsius = get_temperature_input("Celsius");
                let fahrenheit = celsius_to_fahrenheit(celsius);
                println!("{:.1}°C is equal to {:.1}°F", celsius, fahrenheit);
            },
            Ok(3) => {
                println!("Thank you for using Temperature Converter!");
                break;
            },
            _ => println!("Invalid choice! Please enter 1, 2, or 3"),
        }
    }
}
