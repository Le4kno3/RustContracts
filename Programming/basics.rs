fn main(){
	
	//variables are immutable by default
	let val1 = 5;
	//val1 = 6	//this will cause an error
	println!("{}", val1);

	//make variables mutable/editable
	let mut val2 = 5;
	val2 = 6;
	println!("{}", val2);

	//functions
	fun1();

	//functions - args
	fun2(4,4);

	//functions - args
	let val4 = fun3(5,4);
	println!("{}",val4)
}

fn fun1(){
	let val3 = 7;
	println!("{}",val3);
}

fn fun2(x:i32,y:i32){	//we need to specify variable type, "int x" === "x:i32"
	println!("{}",x+y);
}

fn fun3(x:i32,y:i32)->i32{	//we need to specify return type also
	return x+y;
}