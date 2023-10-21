fn main() {
	let t:f64 = 450_000.00;
	let m:f64 = 1_500_000.00;
	let h:f64 = 750_000.00;
	let d:f64 = 2_850_000.00;
	let a:f64 = 250_000.00;
	
	let ft:f64 = 2.0;
	let fm:f64 = 1.0;
	let fh:f64 = 3.0;
	let fd:f64 = 3.0;
	let fa:f64 = 1.0;
	// f being the frequency or quantity of the items;

	let x = t + m + h + d + a;
	println!("The sum of the sales records {}", x );
	let fxt:f64 = t * ft;
	let fxm:f64 = m * fm;
	let fxh:f64 = h * fh;
	let fxd:f64 = d * fd;
	let fxa:f64 = a * fa;

	let sfx:f64 = fxt + fxm + fxh + fxd + fxa;

	//sfx being the sum of the "fx";

	let sf:f64 = ft + fm + fh + fd + fa;

	//sf being the sum of the frequencies;

	let average:f64 = sfx / sf;

	println!("the average is equal {}", average); 

}