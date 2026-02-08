use crate::core::EmailContent;

pub fn activation_email_template(email: &str, token: &str, app_name: &str) -> EmailContent {
    EmailContent::new(
        format!("{} - Account Verification", app_name),
        format!(
            r#"
           <!DOCTYPE html>
            <html>
            <head>
                <style>
                    .container {{
                        font-family: Arial, sans-serif;
                        max-width: 600px;
                        margin: 0 auto;
                        padding: 20px;
                    }}
                    .header {{
                        background-color: #f8f9fa;
                        padding: 20px;
                        text-align: center;
                        border-radius: 5px;
                    }}
                    .content {{
                        padding: 20px;
                        line-height: 1.6;
                    }}
                    .code {{
                        font-size: 24px;
                        font-weight: bold;
                        color: #007bff;
                        background-color: #f8f9fa;
                        padding: 10px 20px;
                        border-radius: 5px;
                        margin: 20px 0;
                        display: inline-block;
                    }}
                    .footer {{
                        margin-top: 20px;
                        text-align: center;
                        color: #6c757d;
                        font-size: 14px;
                    }}
                </style>
            </head>
            <body>
                <div class="container">
                    <div class="header">
                        <h1>{app_name}</h1>
                    </div>
                    <div class="content">
                        <h2>Verify your email address</h2>
                        <p>Hello,</p>
                        <p>Thank you for registering. To verify your email address ({email}), please use this security code:</p>
                        <div class="code">{token}</div>
                        <p>If you didn't request this code, you can safely ignore this email. Someone else might have typed your email address by mistake.</p>
                    </div>
                    <div class="footer">
                        <p>Thanks,<br>{app_name} Team</p>
                        <p>This is an automated message, please do not reply.</p>
                    </div>
                </div>
            </body>
            </html>
        "#
        ),
    )
}
