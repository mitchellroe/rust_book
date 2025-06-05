use std::io;

fn main() {
    println!("How many sequences into Fibonacci do you want to go?");
    let dest_num = get_dest_num();
    let final_num = calc_fib(dest_num);
    println!("The {dest_num} number in the Fibonacci sequence is {final_num}");
}

fn get_dest_num() -> u32 {
    loop {
        let mut dest_num = String::new();
        io::stdin()
            .read_line(&mut dest_num)
            .expect("Failed to read line");
        match dest_num.trim().parse() {
            Ok(num) => {
                break num;
            }
            Err(_) => {
                println!("Invalid number, please try again");
                continue;
            }
        }
    }
}

fn calc_fib(target: u32) -> u32 {
    if target == 0 {
        0
    } else {
        let mut prev = 0;
        let mut sum = 1;
        for _ in 1..target {
            let old_sum = sum;
            sum = sum + prev;
            prev = old_sum;
        }
        sum
    }
}
