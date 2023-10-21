fn main() {
	let p:f64 = 520000000.0;// let p be profit
	let r:f64 = 10.0;// let r be the rate 
	let t:f64 = 5.0;// let t be the number of years
	let b:f64 = 1.0 + (r / 100.00);
	let a:f64 = p * b.powf(t);
	let ci:f64 = a - p;
	println!("Compound Interest {}",ci);
}