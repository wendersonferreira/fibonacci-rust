fn main() {
	for n in 0..301 {
		println!("The fibonacci number at {} position is {}", n, fibonacci(n));		
	}
}

fn fibonacci(n: u32) -> u32 {
	fn internal(n: u32, a:u32, b:u32) -> u32 {
		match n {
			0 => a,
			_ => internal(n-1, b, (a+b)%10000)
		}
	}
	internal(n%1500000,0,1)
}
