use std::io;

fn main() {
	println!("input N - amount of fibonachi first elements you want to see");
	let mut inputStr = String::new();
	io::stdin().read_line(&mut inputStr).ok().expect("can't read line!");
	let n = inputStr.trim().parse::<i32>().ok().expect("can't parse int from string you input");

	let mut f0 = 0;
	let mut f1 = 1;
	for n in 0..n+1{
		let element = match n{
			0 => f0,
			1	=> f1,
			_ => {
				let tmp = f0+f1;
				f0 = f1;
				f1 = tmp;	
				tmp
			}
		};	
		println!("the {} element is {}",n,element);
	}
}
