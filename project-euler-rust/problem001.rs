use std::env;

fn main() {
	let args: Vec<String> = env::args().collect();
	let number:i32 = args[1].parse().unwrap();
	
	let mut sum:i32 = 0; 
	for n in 1..number {
		if (n % 3 == 0) | (n % 5 == 0) {
			sum = sum + n;
		} 
	}
	println!("Total: {}",sum);
}
