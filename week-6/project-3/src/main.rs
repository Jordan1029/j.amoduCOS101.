
use std::io;

fn main() {
    
    println!("Enter the value of n: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: u32 = input.trim().parse().expect("Please enter a valid number");

   
    println!("Multiplication Table (1 to {}):", n);

    for i in 1..=n {
        for j in 1..=20 {
            let result = i * j;
            println!("Here is the time table {} x {} = {}", i, j, result);
        }
        println!(); // Add a newline between each multiplication table 
    }
}