# rust-email-smtp
This is a pet project I used to continue learning Rust!

### How to Run

Begin by setting up an SMTP relay. 

In gmail, go to the desktop site, all settings and set up 2-Factor Authentication.
From there, set an app password.

Clone this repo with 
```https://github.com/Mcutrer5/rust-email-smtp.git```

Create a .env file to store environment variables for your email and app password
```bash
LOGIN_PASSWORD=login
RECIPIENT_EMAIL=email_here
```

Change the recipient and sender emails in main.rs, and use ```cargo run```
