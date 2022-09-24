use std::io; //io is input/output library, taken from the std standard library
use rand::Rng;
use lettre_email::EmailBuilder;
use lettre::smtp::authentication::Credentials;
use lettre::{SmtpClient, Transport};
use chrono::{TimeZone, Utc, DateTime};


fn main() {
// Instantiates objects to be used in email.
	let position = posname();
	let engineer = engname();
	let safe = yesno();
	let g = gas();
	let l = elec();
	let w = water();
	let code = randomcode();
	let dt = datetime();
	
	println!("This permit will be emailed to the specified users.");
	
	let emailtext = format!(
	"\nThis email is from an automated inbox. Do not reply. 
	\nPermit to dig number: {} {} \n
	\nPosition name: {} \n
	\nEngineer name: {} \n
	\nIt is {} \n
	\nThe following services have been located: \n
	\n{}\n
	\n{}\n
	\n{}\n", 
	&code, &dt, &position, &engineer, &safe, &g, &l, &w //adds details from input functions to email text string.
	);
	
	let email = EmailBuilder::new() //creates new email item
        .to("------------")
		.to("------------")
        .from("------------")
        .subject("Permit to dig")
        .text(emailtext)
		//.text("This permit applies to {}.",loc)
		//.text("This permit was generated by {}.",person)
		//.text("Is it safe to dig? {}.",safe)
        .build()
        .unwrap();
		
    let mut mailer = SmtpClient::new_simple("------------") //sends email via Smtp - the simplest way of doing this!
        .unwrap()
        .credentials(Credentials::new("------------".into(), "------------".into())) //Make sure not to expose your credentials.
        .transport();

    let result = mailer.send(email.into());

    println!("{:?}", result);

}

fn randomcode() -> i64 {
	let uniquecode = rand::thread_rng().gen_range(1..=10000);
	println!("Permit reference {}",uniquecode);
	return uniquecode;

}

fn datetime() -> DateTime<Utc> {
	let dt = Utc::now();
	let unixtimestamp: i64 = dt.timestamp();
	let corrdt = Utc.timestamp(unixtimestamp, 0);
	return corrdt;
}

fn posname() -> String {
	let mut position = String::new();
	println!("Please input position name.");
	io::stdin() //calls stdin user input function from io module.
		.read_line(&mut position) //passing &mut guess as the argument to read_line method.
		// read_line takes user input and appends to a string. & indicates this is a reference.
		.expect("Failed to read line"); //Error handling.
		println!("This permit applies to {}",position);
	
	return position
}

fn engname() -> String {

	let mut engineer = String::new();
		println!("Please input engineer name.");
		io::stdin() //calls stdin user input function from io module.
			.read_line(&mut engineer) //passing &mut guess as the argument to read_line method.
			// read_line takes user input and appends to a string. & indicates this is a reference.
			.expect("Failed to read line"); //Error handling.
			println!("This permit was generated by {}",engineer);
		
		return engineer
			
}


fn yesno() -> String {
	let mut input = String::new();
	println!("Is it safe to dig? Y/N");
	io::stdin() //calls stdin user input function from io module.
		.read_line(&mut input) //passing &mut guess as the argument to read_line method.
		// read_line takes user input and appends to a string. & indicates this is a reference.
		.expect("Failed to read line"); //Error handling.
		if input.trim().to_lowercase() == "y" {
			println!("Safe to dig.");
			let outputyn = "safe to dig.".to_string();
			return outputyn;
		} else if input.trim().to_lowercase() == "n" {
			println!("Not safe to dig.");
			let outputyn = "not safe to dig. Do not proceed.".to_string();
			return outputyn;
		} else {
			println!("Invalid input.");
			let outputyn = "Invalid input".to_string();
			return outputyn
		}
}

fn gas() -> String {
  
	let mut ginput = String::new();
		println!("Is gas present? Y/N");
	io::stdin()
		.read_line(&mut ginput)
		.expect("Failed to read line");
		if ginput.trim().to_lowercase() == "y" {
			println!("Gas is present");
			let outputgas = "Gas is present.".to_string();
			return outputgas
		} else if ginput.trim().to_lowercase() == "n" {
			println!("Gas is not present");
			let outputgas = "Gas is not present.".to_string();
			return outputgas
		} else {
			println!("Invalid input.");
			let outputgas = "Invalid input".to_string();
			return outputgas
		}
			
}

fn elec() -> String {
	
	let mut einput = String::new();
		println!("Is electricity present? Y/N");
	io::stdin()
		.read_line(&mut einput)
		.expect("Failed to read line");
		if einput.trim().to_lowercase() == "y" {
			println!("Electricity is present");
			let outputlec = "Electricity is present.".to_string();
			return outputlec
		} else if einput.trim().to_lowercase() == "n" {
			println!("Electricity is not present");
			let outputlec = "Electricity is not present.".to_string();
			return outputlec
		} else {
			println!("Invalid input.");
			let outputlec = "Invalid input".to_string();
			return outputlec
		}
			
}

fn water() -> String {	

	let mut winput = String::new();
		println!("Is water present? Y/N");
	io::stdin()
		.read_line(&mut winput)
		.expect("Failed to read line");
		if winput.trim().to_lowercase() == "y" {
			println!("Water is present");
			let outputw = "Water is present.".to_string();
			return outputw
		} else if winput.trim().to_lowercase() == "n" {
			println!("Water is not present");
			let outputw = "Water is not present.".to_string();
			return outputw
		} else {
			println!("Invalid input.");
			let outputw = "Invalid input".to_string();
			return outputw
		}		
	
	
}

