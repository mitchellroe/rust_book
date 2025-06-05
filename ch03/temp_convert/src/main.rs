use std::io;

fn main() {
    let old_scale = get_scale();
    let old_temp = get_temp();
    let new_temp: f32;
    let new_scale: char;
    if old_scale == 'c' {
        new_scale = 'F';
        new_temp = c_to_f(old_temp);
    } else {
        new_scale = 'C';
        new_temp = f_to_c(old_temp);
    }
    println!("{}°{} is {}°{}",
        old_temp, old_scale.to_uppercase(),
        new_temp, new_scale.to_uppercase());
}

fn get_scale() -> char {
    loop {
        let mut choice = String::new();
        println!("Do you want to provide the temperature in C or F?");
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        let choice = choice.trim().to_lowercase();
        if choice == "c" {
            break 'c';
        } else if choice == "f" {
            break 'f';
        } else {
            println!("Invalid response. Please specify C or F.");
            continue;
        }
    }
}

fn get_temp() -> f32 {
    loop {
        let mut temp = String::new();
        println!("What temperature do you want to convert?");
        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");
        match temp.trim().parse() {
            Ok(num) => { break num; },
            Err(_) => {
                println!("Not a valid number, please try again.");
                continue;
            },
        };
    }
}

fn f_to_c(temp: f32) -> f32 {
    ( temp - 32.0 ) / 1.8
}

fn c_to_f(temp: f32) -> f32 {
    temp * 1.8 + 32.0
}
