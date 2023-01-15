extern crate chrono;
use chrono::Local;
use dotenv::dotenv;
use std::thread;

mod stats;

fn format_data() -> (
    systemstat::Memory,
    systemstat::Swap,
    systemstat::Duration,
    systemstat::OffsetDateTime,
    systemstat::CPULoad,
    systemstat::SocketStats,
) {
    // grabs data from stats and returns a formatted string
    let data = stats::get_data();
    data
}

fn run() {
	let data = format_data();
    let mem = data.0;
    let swap = data.1;
    let uptime = data.2;
    let boot = data.3;
    let cpu = data.4;
    let socket = data.5;

    // format time into a readable string
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let time = now[0..4].to_string()
        + "-"
        + &now[5..7]
        + "-"
        + &now[8..10]
        + " "
        + &now[11..13]
        + ":"
        + &now[14..16]
        + ":"
        + &now[17..19];

		let email_body = format!(
			"Time of report: {:?}\n\nMemory: {} MB / {} MB currently in use.\nSwap: {} / {} MB\nUptime: {} days, {} hours, {} minutes, {} seconds\nBoot time: {}\nCPU load: {}% user, {}% system, {}% intr, {}% idle\n\nSocket stats: {:?}",
			time, 
			mem.free.as_u64() / 1000000,
			mem.total.as_u64() / 1000000,
			swap.total.as_u64() / 1000000,
			swap.free.as_u64() / 1000000,
			uptime.as_secs() / 86400,
			uptime.as_secs() / 3600,
			uptime.as_secs() / 60,
			uptime.as_secs(),
			boot,
			cpu.user * 100.0,
			cpu.system * 100.0,
			cpu.interrupt * 100.0,
			cpu.idle * 100.0,
			socket
		);

		// Use this to add custom text to the email body before or after the stats.
		let email_body = format!(
			"{}\n\n\nAutomated report sent from Rust. :)",
			email_body
		);

    // data_display.iter().for_each(|x| println!("{}", x));
    // data_display.iter().for_each(|x| println!("{}", x));

        use lettre::transport::smtp::authentication::Credentials;
    use lettre::{Message, SmtpTransport, Transport};


	dotenv().ok();
	let USER = std::env::var("USER").unwrap();
	let LOGIN_PASSWORD = std::env::var("LOGIN_PASSWORD").unwrap();

    let email = Message::builder()
      .from("sender@email.com <sender@email.com>".parse().unwrap())
      .to("receiver@gmail.com <receiver@gmail.com>".parse().unwrap())
      .subject("Sending email with Rust")
      .body(String::from(email_body))
      .unwrap();

    let creds = Credentials::new(USER, LOGIN_PASSWORD);

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.gmail.com")
      .unwrap()
      .credentials(creds)
      .build();

    // Send the email
    match mailer.send(&email) {
      Ok(_) => println!("Email sent successfully!"),
      Err(e) => panic!("Could not send email:\n {:?}", e),
    }
}

fn main() {
    // if time is XX:XX, send email
	// Note: time is in 24 hour format | If you just call run() it will send an email immediately
	loop {
		let now = Local::now().format("%H:%M").to_string();
		if now == "20:17" {
			run();
			// comment out to have the program stop after a single email is sent
			thread::sleep(std::time::Duration::from_secs(60));

			// uncomment to have the program break after a single email is sent
			//break;
		}
	}
}