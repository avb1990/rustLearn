use std::io::Write;
fn f(x:f64) -> f64{
	x * x 
}

fn main() {
	let mut input = String::new();	

	print!("input a:");
	std::io::stdout().flush();
	std::io::stdin().read_line(&mut input).ok().expect("failed to read a");
	let mut a = input.trim().parse::<f64>().ok().expect("failed to parse a");
	
	input.clear();

	print!("input b:");
	std::io::stdout().flush();
	std::io::stdin().read_line(&mut input).ok().expect("failed to read b");
	let mut b = input.trim().parse::<f64>().ok().expect("failed to parse b");

	assert!( b > a );
	//println!("a: {}, b: {}",a,b);
	let eps:f64 = 0.001;
	let mut result:f64  = 0.0;
	let mut x:f64 = a;
	while(x < b){
		result += f(x)*eps;
		x+=eps;
	}

	println!("result: {}",result);
}
