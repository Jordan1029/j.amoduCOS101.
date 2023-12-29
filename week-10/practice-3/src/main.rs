fn main() {
    let v = vec![20, 40, 60, 80];
    let v2 = v.clone(); // Clone the vector if you want to keep a copy in v
    let v2_return = display(v2);
    println!("In main {:?}", v2_return);
}

fn display(v: Vec<i32>) -> Vec<i32> {
    // returning the same vector
    println!("inside display {:?}", v.clone()); // Clone if you want to print without consuming v
    v // v is implicitly returned here
}
