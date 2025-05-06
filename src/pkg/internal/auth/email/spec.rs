use std::fmt::{self, Display};

use crate::conf::settings;

use super::sender::{send_email, SendEmail};


pub struct RegistrationTemplate<'a>{
    pub name: &'a str,
    pub username: &'a str,
    pub subject: &'a str,
    pub link: &'a str 
}


impl<'a> Display for RegistrationTemplate<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let html_template = format!(
            r#"
            <!DOCTYPE html>
            <html lang="en">
            <head>
                <meta charset="UTF-8">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <title>Welcome to {} - Your Cloud Platform</title>
                <style>
                    @import url('https://cdn.jsdelivr.net/npm/tailwindcss@2.0.2/dist/tailwind.min.css');
                </style>
            </head>
            <body class="bg-gray-100 text-gray-800">
                <div class="max-w-2xl mx-auto mt-12 p-6 bg-white rounded-lg shadow-lg">
                    <div class="text-center">
                        <h1 class="text-4xl font-bold text-blue-600 mb-6">Welcome to {}!</h1>
                        <p class="text-lg text-gray-700 mb-6">We're thrilled to have you join our platform. Your registration was successful, and you can now start utilizing all of our cloud services.</p>
                    </div>

                    <div class="bg-blue-50 p-6 rounded-xl shadow-md mb-6">
                        <h2 class="text-xl font-semibold text-gray-800 mb-4">Your Account Details</h2>
                        <div class="text-gray-700">
                            <p><strong>Name:</strong> {}</p>
                            <p><strong>Username:</strong> {}</p>
                        </div>
                    </div>

                    <div class="text-center">
                        <p class="text-gray-600 mb-4">To get started, click the link below:</p>
                        <a href="{}" class="inline-block px-8 py-3 bg-blue-500 text-white rounded-lg shadow-md hover:bg-blue-600 transition duration-200">
                            Get Started
                        </a>
                    </div>

                    <div class="mt-8 text-center text-sm text-gray-600">
                        <p>If you did not register for this account, please ignore this email.</p>
                    </div>
                </div>
            </body>
            </html>
            "#,
            &settings.service_name, &settings.service_name, self.name, self.username, self.link
        );

        write!(f, "{}", html_template)
    }
}


impl<'a> SendEmail for RegistrationTemplate<'a>{
    fn send(&self, email: &str) -> crate::prelude::Result<()> {
        send_email(&self.name, email, &self.subject, &format!("{}", &self), true)?;
        Ok(())
    }
}


#[cfg(test)]
pub mod tests{
    use tracing_test::traced_test;

    use crate::prelude::Result;
    use super::*;

    #[test]
    #[traced_test]
    fn test_send_mail_template() -> Result<()>{
        RegistrationTemplate{
            name: "Ashu Pednekar",
            username: "ashupednekar",
            subject: "Welcome",
            link: "https://google.com"
        }.send("ashupednekar49@gmail.com")?;
        Ok(())
    }
}
