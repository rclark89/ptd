// crates for constructing & sending email
use lettre_email::EmailBuilder;
use lettre::smtp::authentication::Credentials;
use lettre::{SmtpClient, Transport};

// future: put these into a (pub) mod, rather than direct from lib.rs
use ptd::randomcode;
use ptd::datetime;
use ptd::posname;
use ptd::engname;
use ptd::gas;
use ptd::elec;
use ptd::water;
use ptd::yesno;
use ptd::new_logfile;
use ptd::append_to_log;

// save results to logfile.txt - append new line with each line of input.
// future: save to DB or blockchain (append only)???

fn main() {
// Checks for creation of logfile. Not related to email at this point.
	let clog = new_logfile();
	let aplog = append_to_log();
	
	
// Instantiates objects to be used in email.
	let position = posname();
	let engineer = engname();
	let safe = yesno();
	let g = gas();
	let l = elec();
	let w = water();
	let code = randomcode();
	let dt = datetime();
	// Add object for creation of logfile.txt
	
// Create email permit to be sent.
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

// Send email.	
let email = EmailBuilder::new() //creates new email item
        .to("---------------")
		.to("---------------")
        .from("---------------")
        .subject("Permit to dig")
        .text(emailtext)
        .build()
        .unwrap();
		
    let mut mailer = SmtpClient::new_simple("smtp.zoho.eu") //sends email via Smtp - the simplest way of doing this!
        .unwrap()
		// DANGER ZONE
        .credentials(Credentials::new("---------------".into(), "---------------".into())) //Make sure not to expose your credentials.
		// DANGER ZONE
        .transport();

    let result = mailer.send(email.into());

    println!("{:?}", result);

}
