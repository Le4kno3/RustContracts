use std::io;	//rust standard library for IO operations

fn main(){
	let mut buffer = String::new();
	println!("Enter a message");
	io::stdin().read_line(&mut buffer);
	println!("Buffer is: {}", buffer)
}