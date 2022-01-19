use std::io;

fn main() {
    println!("Select 1 or 2 as your temperature unit.");

    println!("1 celsius");
    println!("2 fahrenheit");

    let celsius_or_fahrenheit = get_temperature_unit();

    println!("Nice! Now provide the temperature you want to convert.");

    let original_temperature = get_original_temperature();

    convert_temperature(celsius_or_fahrenheit, original_temperature);
}

fn convert_temperature(celsius_or_fahrenheit: i8, original_temperature: f32) {
    if celsius_or_fahrenheit == 1 {
        println!("{}C is {}F", original_temperature, celsius_to_fahrenheit(original_temperature));
        return;
    }

    if celsius_or_fahrenheit == 2 {
        println!("{}F is {}C", original_temperature, fahrenheit_to_celsius(original_temperature));
        return;
    }
}

fn get_temperature_unit() -> i8 {
    loop {
        let mut from_unit  = String::new();

        io::stdin()
            .read_line(&mut from_unit)
            .expect("Failed to read line");

        let from_unit: i8 = match from_unit.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };

        if from_unit != 1 && from_unit != 2 {
            println!("Please enter 1 or 2");
            continue;
        }

        break from_unit;
    }
}

fn get_original_temperature() -> f32 {
    loop {
        let mut original_temperature = String::new();

        io::stdin()
            .read_line(&mut original_temperature)
            .expect("Failed to read line");

        let original_temp: f32 = match original_temperature.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number: {}", original_temperature);
                continue;
            }
        };

        break original_temp;
    }
}

fn fahrenheit_to_celsius(f: f32) -> f32 {
    (f - 32f32) * 5f32/9f32
}

fn celsius_to_fahrenheit(c: f32) -> f32 {
    (c * 9f32/5f32) + 32f32
}