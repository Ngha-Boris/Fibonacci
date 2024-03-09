use std::io;

fn main() {
    println!("Hey there, welcome 🥳😃");
    loop {

        println!("1. Check the Fibonacci number at a particular position");
        println!("2. Check the Fibonacci numbers of integer values of the range 1..23");
        println!("3. Exit the program");
        println!("Choose in the menu what you want either 1, 2, or 3");

        let mut number = String::new();
        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read line");
        
        let number: u64 = match number.trim().parse() {

            Ok(number) => number,
            Err(_) => { 
                println!("Please enter either 1, 2, or 3");
                continue;
            }
            
        };


        match number {
            1 => {
                println!("Input the position(integer) to see the Fibonacci value");
                let mut int = String::new();
                io::stdin()
                    .read_line(&mut int)
                    .expect("Failed to read line");
                let int: u64 = match int.trim().parse() {
                    Ok(int) => int,
                    Err(_) => continue,
                };
                println!("Fibonacci({}) --> {}", int, fibonacci(int as u64));
            }
            2 => {
                println!("Hello there! Let's generate some nth Fibonacci numbers");
                for num in 1..23 {
                    println!("The value at position ({}) --> {}", num, fibonacci(num as u64));
                }
            }
            3 => {  
                println!("Thank you for using our program 👋👋");
                        break;
            }
            _ => continue,
        }


    }
}

fn fibonacci(n: u64) -> u64 {
    if n == 0 {
        return 0
    } else if n == 1 {
        return 1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}
