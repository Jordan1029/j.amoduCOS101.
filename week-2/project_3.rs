fn main() {
	let p:f64 = 210_000.00; //p is principle
	let r:f64 = 5.00; //r is rate
	let t:f64 = 3.00; // t is years

	let b:f64 = r / 100.00; 
	let a:f64 = p * (1.00 - b).powf(t); // a is amount
	let d:f64 = p - a; // d is depriciation
	
	println!(" The depreciation is {}",d);
}