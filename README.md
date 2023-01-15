# rust-email-smtp
This is a pet project I used to continue learning Rust!

### How to Run

Begin by setting up an SMTP relay. 

In gmail, go to the desktop site, 
then click the gear in the top right corner,
from there, click all settings and click on the
forwarding section. Make sure enable iMap is on here.

From there, click on your icon in the top right corner, 
Manage your account, 
go to security and set up 2-Factor Authentication.

After that is set up, create an app password. This will be stored in a .env file

Clone this repo with 
```https://github.com/Mcutrer5/rust-email-smtp.git```

Create a .env file to store environment variables for your email and app password
```bash
LOGIN_PASSWORD=login
RECIPIENT_EMAIL=email_here
```

Change the recipient and sender emails in main.rs, and use ```cargo run```
