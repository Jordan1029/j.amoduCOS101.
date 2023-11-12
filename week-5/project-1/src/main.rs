// The root of the numbers
use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();


    println!("Enter a");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let mut a:f32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter b");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let mut b:f32 = input2.trim().parse().expect("Not a valid number");

    println!("Enter c");
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let mut c:f32 = input3.trim().parse().expect("Not a valid number");

    let d = b * b - (4.0*a*c);
     if d == 0.0 {
        println!("There is one real root");
    }

    else if d > 0.0 {
        println!("There are two distinct root");
    }

    else if d < 0.0 {
        println!("There are no real roots");
    }

    let root_1:f32 = (-b - d.sqrt())/(2.0 * a);
    let root_2:f32 = (-b - d.sqrt())/(2.0 * a);

    println!("The answer to the question is:{}, {}", root_1,root_2 );

     }
