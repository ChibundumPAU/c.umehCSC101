use std::io;

fn main() {
		let mut input1 = String::new();
		let mut input2 = String::new();
		let mut input3 = String::new();
	    let mut input4 = String::new();
	    let mut input5 = String::new();
	 	let mut input6 = String::new();
   	    let mut input7 = String::new();
   	    let mut input8 = String::new();
   	    let mut input9 = String::new();
		let mut input10 = String::new();


     println!("\nWelcome to the PAU Student Council Voter
	 System!");

	println!("Name?: ");
	io::stdin().read_line(&mut input4).expect("Not a valid string");
	println!("Email?: ");
	io::stdin().read_line(&mut input5).expect("Not a valid string");
	println!("Department?: ");
	io::stdin().read_line(&mut input6).expect("Not a valid string");
	println!("State of Origin?: ");
	io::stdin().read_line(&mut input7).expect("Not a valid string");

	println!("Firstly, we must check if you are eligible to vote:");
	println!("Are you a current Class Rep? (1 for Yes, 2 for No");

	io::stdin().read_line(&mut input1).expect("Not a valid string");
	let answer:f32 = input1.trim().parse().expect("Not a valid number");
	if answer == 1.0 {
		println!("What level are you in");
	}else {
		println!("Sorry you are not eligible to vote")
	}

    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let level:f32 = input2.trim().parse().expect("Not a valid number");
	if level < 100.0 {
		println!("Sorry you are not eligible to vote");
	} else{
		println!("What is your GPA?");
	}

	io::stdin().read_line(&mut input3).expect("Not a valid string");
	 let gpa:f32 = input2.trim().parse().expect("Not a valid number");

	if gpa > 4.0 {
		println!("You can vote!");
		println!("Name: {}", input4);
		println!("Email: {}", input5) ;
		println!("Department: {}", input6);
		println!("State of Origin: {}", input7);
	} else{
		println!("Sorry you are not eligible to vote");
	}
	println!("Do you wish to continue to the Faculty Publication Incentive System? (1 for Yes, 2 for No");
	let a:f32 = input8.trim().parse().expect("Not a valid number");
    io::stdin().read_line(&mut input8).expect("Not a valid string");
	if a == 1.0{
		println!("Welcome to the FacPub function!");
	}
	println!("Name of Faculty?:");
	io::stdin().read_line(&mut input9).expect("Not a valid string");

	println!("Papers published?: ");
	 io::stdin().read_line(&mut input10).expect("Not a valid string");
    let paper:f32 = input10.trim().parse().expect("Not a valid number");
    if paper > 3.0 {
    	println!("Your incentive is N500,000");
   if paper > 5.0 {
    	println!("Your incentive is N800,000");
   if paper > 10.0 {
    	println!("Your incentie is N1,000,000");
    }
    }
    }else{
    	println!("Your incentive is N100,000");
    }
}
	

