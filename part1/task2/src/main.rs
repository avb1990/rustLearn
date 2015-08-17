use std::io::Write;
fn main() {
	let mut input = String::new();		

	print!("enter first number:");	
	std::io::stdout().flush();
	std::io::stdin().read_line(&mut input).ok().expect("read first number error");
	let mut firstNumber = input.trim().parse::<i32>().ok().expect("parse first number error");

	println!("");
	input.clear();

	print!("enter second number:");
	std::io::stdout().flush();
	std::io::stdin().read_line(&mut input).ok().expect("read second number error");
	let secondNumber = input.trim().parse::<i32>().ok().expect("parse second number error");

	//println!("the first number is: {} the second: {}", firstNumber, secondNumber);
	let mut little = 0;
	let mut big = 0;
	let mut tmp = 0 ;
	if firstNumber < secondNumber{
		little = firstNumber;
		big = secondNumber;
	} else {
		little = secondNumber;
		big = firstNumber;
	}

	while( (big % little) != 0 ){
		//println!("in cycle big: {} little: {}", big, little);
		big -= little;
		if( big < little ){
			tmp = little;
			little  = big;
			big = tmp;
		}
	}
	println!("the greatest common deviser is: {}", little);
}
