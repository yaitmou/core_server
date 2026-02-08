use std::collections::HashMap;

use bson::oid::ObjectId;
use regex::Regex;

use crate::{
    api::auth::domain::entities::Claims,
    core::{AppError, MsgBuilder},
};
pub type ValidatorsResult<T> = Result<T, AppError>;

pub struct Validators {}

impl Validators {
    /// Validates a 6 digit pin code
    pub fn validate_pin_code(code: u32) -> ValidatorsResult<()> {
        if code < 100_000 || code > 999_999 {
            let msg = MsgBuilder::custom("Invalid PIN Code");
            return Err(AppError::InvalidInput(msg));
        }
        Ok(())
    }
    pub fn admin_only_validator(claims: &Claims) -> ValidatorsResult<()> {
        if !claims.is_admin() {
            let msg = MsgBuilder::no_permission_to("Perform this action");
            return Err(AppError::Forbidden(msg));
        }
        Ok(())
    }

    pub fn admin_or_owner_validator(claims: &Claims, user_id: &str) -> ValidatorsResult<()> {
        if user_id != claims.user_id && !claims.is_admin() {
            let msg = MsgBuilder::no_permission_to("Perform this action");
            return Err(AppError::Forbidden(msg));
        }
        Ok(())
    }

    pub fn validate_query(query: HashMap<String, String>) -> ValidatorsResult<()> {
        if query.is_empty() {
            return Err(AppError::EmptyQuery);
        }
        Ok(())
    }

    pub fn validate_email(email: &str) -> ValidatorsResult<String> {
        if email.is_empty() {
            return Err(AppError::InvalidInput("Email cannot be empty".to_string()));
        }

        if email.len() > 254 {
            return Err(AppError::InvalidInput("Email is too long".to_string()));
        }

        let email_regex =
            Regex::new(r"^[a-zA-Z0-9.!#$%&’*+/=?^_`{|}~-]+@[a-zA-Z0-9-]+(?:\.[a-zA-Z0-9-]+)*$")
                .unwrap();

        if !email_regex.is_match(email) {
            return Err(AppError::InvalidInput("Invalid email format".to_string()));
        }

        Ok(email.to_string())
    }

    pub fn validate_web_link(link: &str) -> ValidatorsResult<String> {
        let pattern = r"^(https?://)?[a-zA-Z0-9][-a-zA-Z0-9.]*\.[a-z]{2,}(/.*)?$";
        let re = Regex::new(pattern).map_err(|_| {
            let msg = "Could not validate organization website link".to_string();
            AppError::InternalServer(msg)
        })?;
        if re.is_match(link) {
            return Ok(link.to_string());
        }
        Err(AppError::InvalidInput("Invalid website link".to_string()))
    }

    pub fn validate_optional_web_link(
        maybe_link: Option<String>,
    ) -> ValidatorsResult<Option<String>> {
        maybe_link.map(|s| Self::validate_web_link(&s)).transpose()
    }

    pub fn validate_text_len(
        text: String,
        label: Option<String>,
        min: Option<i32>,
        max: Option<i32>,
    ) -> ValidatorsResult<String> {
        let min = min.unwrap_or(3) as usize;
        let max = max.unwrap_or(100) as usize;
        let label = label.unwrap_or("Label".to_string());
        if text.len() < min || text.len() > max {
            let reason = format!(
                "{} must be between {} and {} characters long",
                label, min, max
            );
            return Err(AppError::InvalidInput(reason));
        }
        Ok(text)
    }

    pub fn validate_object_id(id: &str) -> ValidatorsResult<ObjectId> {
        let reason = format!("Invalid Object ID");
        ObjectId::parse_str(id).map_err(|_| AppError::InvalidInput(reason))
    }

    pub fn validate_optional_object_id(
        maybe_id: Option<String>,
    ) -> ValidatorsResult<Option<ObjectId>> {
        maybe_id
            .map(|s| Self::validate_object_id(&s)) // Converts Option<String> -> Option<Result<ObjectId, Error>>
            .transpose()
    }

    pub fn validate_object_id_vec(ids: Vec<String>) -> ValidatorsResult<Vec<ObjectId>> {
        let mut result = Vec::with_capacity(ids.len());
        for id in ids {
            result.push(Validators::validate_object_id(&id)?);
        }
        Ok(result)
    }

    pub fn validate_dir_name(entity_type: &str) -> ValidatorsResult<&str> {
        // Allow alphanumeric and underscores, 2-50 chars
        let is_valid_format = entity_type.chars().all(|c| c.is_alphanumeric() || c == '_')
            && entity_type.len() >= 2
            && entity_type.len() <= 50;

        // Disallow dangerous names
        let dangerous_names = ["..", ".", "/", "\\", " ", ""];
        if dangerous_names.contains(&entity_type) && !is_valid_format {
            return Err(AppError::InvalidFolderName(entity_type.to_string()));
        }

        Ok(entity_type)
    }
    pub fn validate_positive_f64(num: f64, label: &str) -> ValidatorsResult<f64> {
        if num < 0.0 {
            let msg = format!("{} must be a positive number", label);
            return Err(AppError::InvalidInput(msg));
        }
        Ok(num)
    }
    pub fn validate_positive_i32(num: i32, label: &str) -> ValidatorsResult<i32> {
        if num < 0 {
            let msg = format!("{} mst be a positive number", label);
            return Err(AppError::InvalidInput(msg));
        }
        Ok(num)
    }
}
