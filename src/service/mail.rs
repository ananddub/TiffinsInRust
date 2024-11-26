pub mod Mail{
    use dotenv::dotenv;
    use lettre::message::{header, Message};
    use lettre::{SmtpTransport, Transport};
    use lettre::transport::smtp::authentication::Credentials;
    use std::env;
    use thiserror::Error;

    #[derive(Debug, Error)]
    pub enum MailError {
        #[error("Failed to retrieve environment variables: {0}")]
        EnvVarError(String),
        #[error("Email creation failed: {0}")]
        EmailCreationError(String),
        #[error("SMTP transport error: {0}")]
        SmtpTransportError(#[from] lettre::transport::smtp::Error),
    }

    pub async fn mail(to: &str, html: &str) -> Result<(), MailError> {
        // Load environment variables
        dotenv().ok();

        // Retrieve environment variables with proper error handling
        let username = env::var("EMAIL_USER").map_err(|_| MailError::EnvVarError("EMAIL_USER not set".to_string()))?;
        let password = env::var("EMAIL_PASS").map_err(|_| MailError::EnvVarError("EMAIL_PASS not set".to_string()))?;
        let port = env::var("EMAIL_PORT")
            .map_err(|_| MailError::EnvVarError("EMAIL_PORT not set".to_string()))?
            .parse::<u16>()
            .map_err(|_| MailError::EnvVarError("EMAIL_PORT is not a valid number".to_string()))?;
        let host = env::var("EMAIL_HOST").map_err(|_| MailError::EnvVarError("EMAIL_HOST not set".to_string()))?;

        // Create the email
        let email = Message::builder()
            .from(username.parse().map_err(|_| MailError::EmailCreationError("Invalid 'from' email".to_string()))?)
            .to(to.parse().map_err(|_| MailError::EmailCreationError("Invalid 'to' email".to_string()))?)
            .subject("Email Subject")
            .header(header::ContentType::TEXT_HTML)
            .body(html.to_string())
            .map_err(|e| MailError::EmailCreationError(format!("Email creation failed: {}", e)))?;

        // Create SMTP credentials and mailer
        let creds = Credentials::new(username.clone(), password);
        let mailer = SmtpTransport::relay(&host)
            .map_err(|e| MailError::SmtpTransportError(e))?
            .port(port)
            .credentials(creds)
            .build();

        // Send the email
        mailer.send(&email).map_err(|e| MailError::SmtpTransportError(e))?;

        Ok(())
    }
    pub  fn otp_html(otp:&str,email:&str)->String{
        format!(r#"
<head>
    <title>Verification Code</title>
    <style>
        body{{
            font-family: 'Arial', sans-serif;
            margin: 0;
            padding: 0;
            background: linear-gradient(135deg, #a8edea, #fed6e3);
            color: #333;
            line-height: 1.8;
            display: flex;
            justify-content: center;
            align-items: center;
            height: 100vh;
        }}

        .container{{
            width: 100%;
            max-width: 450px;
            margin: auto;
            padding: 20px 25px;
            background: #ffffff;
            border-radius: 12px;
            box-shadow: 0 8px 16px rgba(0, 0, 0, 0.15);
        }}

        h1{{
            text-align: center;
            font-size: 28px;
            margin-bottom: 15px;
            color: #333;
            font-weight: bold;
            letter-spacing: 1px;
        }}

        p{{
            margin-bottom: 20px;
            text-align: center;
            color: #555;
            font-size: 16px;
        }}

        .code-container{{
            text-align: center;
            margin: 20px 0;
        }}

        .code{{
            background: #e0f7fa;
            color: #00796b;
            padding: 15px 30px;
            border-radius: 8px;
            display: inline-block;
            font-family: 'Courier New', monospace;
            font-size: 26px;
            font-weight: bold;
            box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
            transition: transform 0.3s ease;
        }}

        .code:hover{{
            transform: scale(1.05);
        }}

        .footer{{
            font-size: 13px;
            color: #666;
            text-align: center;
            margin-top: 25px;
        }}

        .footer a{{
            color: #00796b;
            text-decoration: none;
            font-weight: bold;
        }}

        .footer a:hover{{
            text-decoration: underline;
        }}

        .button{{
            display: block;
            text-align: center;
            background: #00796b;
            color: white;
            padding: 12px 25px;
            border-radius: 8px;
            font-size: 16px;
            text-decoration: none;
            margin: 20px auto;
            width: 70%;
            transition: background 0.3s ease;
        }}

        .button:hover{{
            background: #005f4f;
        }}
    </style>
</head>
<body>
    <div class="container">
        <h1><b>Welcome to Tiffins</b></h1>
        <p>
            Use the verification code below to securely log in to your account associated with <b>{email}</b>.
        </p>
        <p><b>Your code is valid for the next 5 minutes.</b></p>

        <div class="code-container">
            <span class="code"><b>{otp}</b></span>
        </div>

        <p><b>If you didn’t request this, you can safely ignore this email. Your account is secure.</b></p>

        <div class="footer">
            <p><b>Cheers,</b></p>
            <p><b>The Tiffins Team</b></p>
            <p><b>Need help? Visit our <a href="https://yourwebsite.com/support">Support Page</a>.</b></p>
        </div>
    </div>
</body>
        "#)
    }

    pub fn forgot_password_html(otp: &str, email: &str) -> String {
        format!(
            r#"
<head>
    <title>Password Reset Request</title>
    <style>
        body {{
            font-family: 'Arial', sans-serif;
            margin: 0;
            padding: 0;
            background: linear-gradient(135deg, #fdfbfb, #ebedee);
            color: #333;
            line-height: 1.8;
            display: flex;
            justify-content: center;
            align-items: center;
            height: 100vh;
        }}

        .container {{
            width: 100%;
            max-width: 450px;
            margin: auto;
            padding: 20px 25px;
            background: #ffffff;
            border-radius: 12px;
            box-shadow: 0 8px 16px rgba(0, 0, 0, 0.15);
        }}

        h1 {{
            text-align: center;
            font-size: 28px;
            margin-bottom: 15px;
            color: #d32f2f;
            font-weight: bold;
            letter-spacing: 1px;
        }}

        p {{
            margin-bottom: 20px;
            text-align: center;
            color: #555;
            font-size: 16px;
        }}

        .code-container {{
            text-align: center;
            margin: 20px 0;
        }}

        .code {{
            background: #fbe9e7;
            color: #d84315;
            padding: 15px 30px;
            border-radius: 8px;
            display: inline-block;
            font-family: 'Courier New', monospace;
            font-size: 26px;
            font-weight: bold;
            box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
            transition: transform 0.3s ease;
        }}

        .code:hover {{
            transform: scale(1.05);
        }}

        .footer {{
            font-size: 13px;
            color: #666;
            text-align: center;
            margin-top: 25px;
        }}

        .footer a {{
            color: #d84315;
            text-decoration: none;
            font-weight: bold;
        }}

        .footer a:hover {{
            text-decoration: underline;
        }}
    </style>
</head>
<body>
    <div class="container">
        <h1>Password Reset Request</h1>
        <p>
            We received a request to reset the password for the account associated with <b>{email}</b>.
        </p>
        <p><b>Use the code below to reset your password. This code is valid for the next 10 minutes.</b></p>

        <div class="code-container">
            <span class="code">{otp}</span>
        </div>

        <p>If you didn’t request this, you can safely ignore this email.</p>

        <div class="footer">
            <p><b>Cheers,</b></p>
            <p><b>The Support Team</b></p>
            <p><b>Need help? Visit our <a href="https://yourwebsite.com/support">Support Page</a>.</b></p>
        </div>
    </div>
</body>
        "#
        )
    }
}