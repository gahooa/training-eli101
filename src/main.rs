use std::io::Write;

fn main() {

    let name = get_text("What is your name? ");
    let age = get_int("What is your age? ");
    // print the input
    println!("Hello, {}, you are {} years old!", name, age);
    
    if age >= 21 {
        println!("You are old enough to drink in the US");
    } else {
        println!("You are not old enough to drink in the US");
    }


}


fn get_text(prompt: &str) -> String {
    loop {
        let mut input = String::new();
        print!("{}", prompt);
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_string();
        if input.len() > 0 {
            return input;
        }
    }
}

fn get_int(prompt: &str) -> i32 {
    loop {
        let input = get_text(prompt);
        match input.parse::<i32>() {
            Ok(n) => return n,
            Err(_) => println!("Invalid input. Not a number!"),
        }
    }
}