use std::io;

fn main() {    
    loop {
        println!("Please select convertions:");
        println!("1 - Fahrenheit to Celcius");
        println!("2 - Celcius to Fahrenheit");
        println!("3 - Quit");

        let mut convertion = String::new();

        io::stdin()
            .read_line(&mut convertion)
            .expect("Failed to read line");

        let convertion: u8 = match convertion.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if convertion == 1 {
            print!("Please enter Fahrenheit degree:");
            let mut degree = String::new();

            io::stdin()
                .read_line(&mut degree)
                .expect("Failed to read line");

            let degree: f32 = match degree.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            let celcius: f32 = (degree - 32:f32) / 1.8000;
            println!("{degree} of Fahrenheit is equal to {celcius} degree in Celcius.");
        }
        else if convertion == 2 {
            print!("Please enter Celcius degree:");
            let mut degree = String::new();

            io::stdin()
                .read_line(&mut degree)
                .expect("Failed to read line");

            let degree: f32 = match degree.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            let fahrenheit: f32 = (degree * 1.8000) + 32;
            println!("{degree} of Fahrenheit is equal to {fahrenheit} degree in Celcius.");
        }
        else if convertion == 3 {
            break;
        }
    }
}
