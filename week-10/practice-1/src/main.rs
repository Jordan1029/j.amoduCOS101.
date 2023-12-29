fn main() {
    let v = vec![101, 250, 330, 400]; // vector v owns the object in heap

    let v2 = &v; // v2 is an immutable reference to v

    // Here, you are printing the original vector v, and it's perfectly fine
    // to do so even though v2 is still in scope.
    println!("{:?}", v);
}
