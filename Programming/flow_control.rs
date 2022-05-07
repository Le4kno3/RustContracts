fn main(){
	let mut x = 8;

	//basic if...else
	if x==11{
		println!("X is not 10");
	}else{
	    println!("X is 10");
	}

	//loops
	let mut count = 5;
	loop{
		if count==0{
			break;
		}

		if x==10{
			println!("X is 10");
		}else{
		    println!("X is not 10");
		}
        x = x + 1;
		count = count - 1;

	}
}