use crate::core::EmailContent;

pub fn password_reset_confirmation_email_template(email: &str, app_name: &str) -> EmailContent {
    EmailContent::new(
        format!("{} - Password Reset Successful", app_name),
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
                    .alert {{
                        background-color: #f8d7da;
                        color: #721c24;
                        padding: 10px 20px;
                        border-radius: 5px;
                        margin: 20px 0;
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
                        <h2>Password Reset Successful</h2>
                        <p>Hello,</p>
                        <p>Your password for your {app_name} account ({email}) has been successfully reset.</p>
                        <div class="alert">
                            <p>If you did not request this password reset, please contact our support team immediately (support@qkons.com) and secure your account.</p>
                        </div>
                        <p>You can now log in to your account using your new password.</p>
                    </div>
                    <div class="footer">
                        <p>Thanks,<br>{app_name} Team</p>
                        <p>This is an automated message, please do not reply.</p>
                    </div>
                </div>
            </body>
            </html>
        "#,
        ),
    )
}
