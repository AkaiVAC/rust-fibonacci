use std::io::stdin;
fn main() {
    loop {
        println!("Enter target number:");
        let mut target_number = String::new();
        stdin()
            .read_line(&mut target_number)
            .expect("Failed to read line");
        let target_number: u32 = match target_number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter a number");
                continue;
            }
        };
        println!("Fibonacci result: {}", fibo(target_number));
        println!("Continue? y/n");
        let mut choice = String::new();
        stdin().read_line(&mut choice).expect("Failed to read line");
        match choice.trim().to_ascii_lowercase().as_ref() {
            "y" => continue,
            "n" => break,
            _ => {
                println!("Enter y or n");
                continue;
            }
        }
    }
}

fn fibo(num: u32) -> u32 {
    match num {
        1 => 1,
        _ => num * fibo(num - 1),
    }
}
