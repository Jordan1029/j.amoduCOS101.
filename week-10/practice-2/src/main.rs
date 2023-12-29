fn main() {
    let v = vec![10, 20, 30];
    display(&v);
    println!("In main {:?}", v);
}

fn display(v: &Vec<i32>) {
    println!("inside display {:?}", v);
}
