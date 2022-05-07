fn main(){
	
	//array
	let val1 = [1,2,3];
	println!("{}", val1);

	//string - 1
	let val2 = "This is a sample string".to_string();
	println!("{}", val2);

	//string - 2
	let mut val2 = String::new();
	val2 = "This is a sample string".to_string();
	println!("{}", val2);

	//string - 2 - Access one of the element using indexing
	let val3 = "This is a sample string".to_string;
	//println!("{}", val3[0]);	//this will give error
	// let val4 = val3.chars().nth(0);
	// println!("{}", val4)

	//tuples
	let val5 = (10, 3.14, 'a' ,true)
	println!("{}",tup.2);

}
