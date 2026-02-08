use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub database_uri_string: String,
    pub database_name: String,
    pub jwt_secret: String,
    pub jwt_expiration: i64,
    pub email_from: String,
    pub app_name: String,
    pub uploads_base: String,
    pub resend_token: String,
}

impl Config {
    pub fn new(is_dev: bool) -> Result<Self, env::VarError> {
        /* ······································································ [ Development ] */
        if is_dev {
            Ok(Config {
                database_uri_string: "mongodb://127.0.0.1:27017/dev_db".to_string(),
                database_name: "dev_db".to_string(),
                jwt_secret: "1little_2_unsafe_jwt_123456_secret".to_string(),
                jwt_expiration: 3600,
                email_from: "no-reply@mail.com".to_string(),
                app_name: "younss_core_server".to_string(), // Change to fit your needs ;P
                uploads_base: "./uploads".to_string(),      // if needed
                resend_token: "".to_string(),               // You should provide a resend_token
            })
        } else {
            /* ··································································· [ Production ] */
            Ok(Config {
                database_uri_string: env::var("MONGODB_URI")?,
                database_name: env::var("DATABASE_NAME")?,
                jwt_secret: env::var("JWT_SECRET")?,
                jwt_expiration: env::var("JWT_EXPIRATION")?.parse().unwrap_or(3600), // 60 * 60 = 3600 seconds => 1 Hour
                email_from: env::var("EMAIL_FROM")?,
                app_name: env::var("APP_NAME")?,
                // upload_base is the root where your server is storing user's related images or
                // documents. Make to set the proper permission.
                uploads_base: env::var("UPLOADS_BAS")?,
                // Here we are using resend for email handling.
                // If you are using a different email provider or simple smtp, provide an
                // implementation for the EmailService found under services/email_service
                resend_token: env::var("RESEND_TOKEN")?,
            })
        }
    }
}
