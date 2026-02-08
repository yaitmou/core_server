use bson::{oid::ObjectId, DateTime as BsonDateTime};
use serde::{Deserialize, Serialize};

use crate::{
    api::auth::domain::entities::{user_role::UserRole, User},
    core::{crud_model::CrudModel, AppError, Validators},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserMongoModel {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub password: String,
    #[serde(default)]
    pub first_name: String,
    #[serde(default)]
    pub last_name: String,
    #[serde(default)]
    pub role: UserRole,
    #[serde(default)]
    pub reset_pwd_token: Option<String>,
    #[serde(default)]
    pub reset_pwd_count: i32,
    #[serde(default)]
    pub activation_token: Option<String>,
    #[serde(default)]
    pub activation_count: i32,
    #[serde(default)]
    pub is_logged_out: bool,
    #[serde(default)]
    pub verified: bool,
    #[serde(default)]
    pub banned: bool,

    pub created_at: BsonDateTime,
}

impl TryFrom<User> for UserMongoModel {
    type Error = AppError;

    fn try_from(user: User) -> Result<Self, Self::Error> {
        let id = if user.id.is_empty() {
            None
        } else {
            let id_or_err = Validators::validate_object_id(&user.id)?;
            Some(id_or_err)
        };

        Ok(Self {
            id,
            email: user.email,
            password: user.password,
            first_name: user.first_name,
            last_name: user.last_name,
            role: user.role,
            reset_pwd_token: user.reset_pwd_token,
            reset_pwd_count: user.reset_pwd_count,
            activation_token: user.activation_token,
            activation_count: user.activation_count,
            is_logged_out: user.is_logged_out,
            verified: user.verified,
            banned: user.banned,
            created_at: BsonDateTime::from_chrono(user.created_at),
        })
    }
}

impl From<UserMongoModel> for User {
    fn from(model: UserMongoModel) -> Self {
        Self {
            id: model.id.unwrap().to_string(),
            email: model.email,
            password: model.password,
            first_name: model.first_name,
            last_name: model.last_name,
            role: model.role,
            reset_pwd_token: model.reset_pwd_token,
            reset_pwd_count: model.reset_pwd_count,
            activation_token: model.activation_token,
            activation_count: model.activation_count,
            is_logged_out: model.is_logged_out,
            verified: model.verified,
            banned: model.banned,
            created_at: model.created_at.to_chrono(),
        }
    }
}

impl CrudModel<User> for UserMongoModel {
    fn try_from_entity(user: User) -> Result<Self, AppError> {
        user.try_into()
    }

    fn to_entity(self) -> User {
        self.into()
    }
}
