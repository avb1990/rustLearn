use std::io::Write;

fn f( x:f64 ) -> f64{
	x * x - 10000.0
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

	if f(a) > 0.0 || f(b) < 0.0 {
		println!("equation hasn't any solutions");
		return;
	}
	let mut tmp:f64 = 0.0;
	let mut counter = 0;
	while( (f(b) - f(a)) > 0.001 ){
		counter += 1;
		tmp = a + (b - a) / 2.0;
		//println!("in cycle. tmp: {}, f(tmp): {}, a: {}, b: {}", tmp, f(tmp), a, b);
		if f(tmp) > 0.0 
			{b -= tmp-a;}
		else {a = tmp;}
	}
	//println!("a: {}, b: {}",a,b);
	println!("aproximate solution of equation calculate in {} cycles is: {}. ", counter, a);
}
