use std::io;
fn main() {
    let mut count = 0;
    while count <= 500 {

    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Please enter your name:");
    io::stdin().read_line(&mut input1).expect("Not a valid string");

    println!("Numer of papers published");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let paper:i32 = input2.trim().parse().expect("Not a valid number");

    if paper <= 3{
        println!("Your incentive is N100,000");
    }

    else if paper > 3 && paper < 5{
        println!("Your incentive is N500,000");
    }

    else if paper > 5 && paper < 10{
        println!("Your incentive is N800,000");
    }

    else if paper >= 10{
        println!("Your incentive is N1,000,000");
    }
    count +=1
}
}
