use std::io;

fn trapezium(){
    println!("AREA OF TRAPEZIUM");
    let mut input1= String::new();
    let mut input2= String::new();
    let mut input3= String::new();

println!("\nInput trapezium's height (in cm)");
io::stdin().read_line(&mut input1).expect("Failed to read input");
let height:f32 = input1.trim().parse().expect("Invalid input");

println!("\nInput trapezium's first base (in cm)");
io::stdin().read_line(&mut input2).expect("Failed to read input");
let first_base:f32 = input2.trim().parse().expect("Invalid input");

println!("\nInput trapezium's second base (in cm)");
io::stdin().read_line(&mut input3).expect("Failed to read input");
let second_base:f32 = input3.trim().parse().expect("Invalid input");

let trap_area:f32 = (0.5 * height) * (first_base * second_base);

println!("Thea area of the trapezium is{:.2}",trap_area);
}

fn rhombus(){
    println!("AREA OF RHOMBUS");
    let mut input1= String::new();
    let mut input2= String::new();

println!("\nInput rhombus  first diagonal (in cm)");
io::stdin().read_line(&mut input1).expect("Failed to read input");
let d1:f32 = input1.trim().parse().expect("Invalid input");

println!("\nInput rhombus second diagonal(in cm)");
io::stdin().read_line(&mut input2).expect("Failed to read input");
let d2:f32 = input2.trim().parse().expect("Invalid input");

let rhom_area:f32 = 0.5 * d1 *d2;

println!("Thea area of the rhombus is{:.2}",rhom_area);
}

fn Parallelogram(){
    println!("AREA OF PARALLELOGRAM");
    let mut input1= String::new();
    let mut input2= String::new();

println!("\nInput base(in cm)");
io::stdin().read_line(&mut input1).expect("Failed to read input");
let base:f32 = input1.trim().parse().expect("Invalid input");

println!("\nInput altitude (in cm)");
io::stdin().read_line(&mut input2).expect("Failed to read input");
let altitude:f32 = input2.trim().parse().expect("Invalid input");
let para_area:f32 = base * altitude;
println!("Thea area of the parallelogram is{:.2}",para_area)
}

fn cube(){
    println!("AREA OF A CUBE");
    let mut input1= String::new();
    

println!("\nInput length (in cm)");
io::stdin().read_line(&mut input1).expect("Failed to read input");
let length:f32 = input1.trim().parse().expect("Invalid input");


let cube:f32 = 6.0 * (length.powf(2.0));

println!("Thea area of the cube is{:.2}",cube);
}

fn cylinder(){
    println!("VOLUME OF CYLINDER");
    let mut input1= String::new();
    let mut input2= String::new();
    let mut input3= String::new();

println!("\nInput radius (in cm)");
io::stdin().read_line(&mut input1).expect("Failed to read input");
let radius:f32 = input1.trim().parse().expect("Invalid input");

println!("\nInput height (in cm)");
io::stdin().read_line(&mut input2).expect("Failed to read input");
let height:f32 = input2.trim().parse().expect("Invalid input");

let cyl_area:f32 = 22.0/7.0 * radius.powf(2.0) * height;

println!("The volume of the cylinder is{:.2}",cyl_area);
}

fn main(){
    println!("THE CALCULATOR");
    println!("\n1 Trapezium\n2.rhombus\n3.parallelogram\n4. cube\n5.cylinder");
    println!("Insert the number attached to the shape for your calculation");
    let mut input =String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let input:i32 = input.trim().parse().expect("Invalid input");

    let arr = [trapezium,rhombus,Parallelogram,cube,cylinder];

    if input >= 1 && input <=5{
        arr[(input -1)as usize]();
    }
    
    else{
        println!("Invalid input");
    }
}

