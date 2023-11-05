// Rust program to calculate speed of of a car going 80 miles for 2 hours

use std::io;

fn main() {
    {
        println!("for when the distance is 80 miles and the time is 2 hours");
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter the distance in miles");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let m:f32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter the time in hours");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let t:f32 = input2.trim().parse().expect("Not a valid number");

    let k:f32 = m * 1.609344;
    let mut speed:f32 = k/t;

    println!("The speed of the car: {}", speed);
    }

    {
    // Rust program to calculate spped of a car going 120 miles in 4 hours
    println!("For when the distance is 120 miles and the time is 4 hours");
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter the distance in miles");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let m:f32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter the time in hours");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let t:f32 = input2.trim().parse().expect("Not a valid number");

    let k:f32 = m * 1.609344;
    let mut speed:f32 = k/t;

    println!("The speed of the car: {}", speed);

    }

}
