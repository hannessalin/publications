use std::env;

fn main() {
	let args: Vec<String> = env::args().collect();
	let number:u32 = args[1].parse().unwrap();
	
	let mut f_start = 1;
	let mut f_next = 2;
	let mut sum = 0;

	while f_next < number {
		if f_next % 2 == 0 {
			sum = sum + f_next;
		}
		f_next = f_next + f_start;
		f_start = f_next - f_start;
	}
	println!("Total: {}",sum);
}
