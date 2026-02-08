use mongodb::bson::oid::ObjectId;
use mongodb::bson::DateTime as BsonDateTime;
use serde::{Deserialize, Serialize};

use crate::{
    api::{
        auth::domain::entities::user_role::UserRole,
        auth_token::domain::entities::refresh_token::RefreshToken,
    },
    core::{crud_model::CrudModel, AppError, Validators},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RefreshTokenMongoModel {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    #[serde(default)]
    pub user_id: ObjectId,

    #[serde(default)]
    pub token: String,

    #[serde(default)]
    pub user_role: UserRole,

    pub expires_at: BsonDateTime,

    #[serde(default)]
    pub revoked: bool,

    // Audit Fields
    pub created_by: ObjectId,
    pub created_at: BsonDateTime,
    pub updated_by: ObjectId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<BsonDateTime>,
}

impl TryFrom<RefreshToken> for RefreshTokenMongoModel {
    type Error = AppError;

    fn try_from(entity: RefreshToken) -> Result<Self, Self::Error> {
        let id = if entity.id.is_empty() {
            None
        } else {
            let id_or_err = Validators::validate_object_id(&entity.id)?;
            Some(id_or_err)
        };

        let user_id = Validators::validate_object_id(&entity.user_id)?;
        let created_by = Validators::validate_object_id(&entity.created_by)?;
        let updated_by = Validators::validate_object_id(&entity.updated_by)?;

        let mut updated_at: Option<BsonDateTime> = None;
        if id.is_none() {
            // we are creating a new document so we need to set updated at
            updated_at = Some(BsonDateTime::from_chrono(entity.updated_at));
        }

        Ok(Self {
            id,
            user_id,
            token: entity.token,
            user_role: entity.user_role,
            expires_at: BsonDateTime::from_chrono(entity.expires_at),
            revoked: entity.revoked,

            // Audit fields
            created_at: BsonDateTime::from_chrono(entity.created_at),
            created_by,
            updated_at,
            updated_by,
        })
    }
}

impl From<RefreshTokenMongoModel> for RefreshToken {
    fn from(model: RefreshTokenMongoModel) -> Self {
        Self {
            id: model.id.unwrap().to_string(),
            user_id: model.user_id.to_string(),
            token: model.token,
            user_role: model.user_role,
            expires_at: model.expires_at.to_chrono(),
            revoked: model.revoked,

            // Audit Fields
            created_by: model.created_by.to_string(),
            created_at: model.created_at.to_chrono(),
            updated_by: model.updated_by.to_string(),
            updated_at: model.updated_at.unwrap().to_chrono(),
        }
    }
}

impl CrudModel<RefreshToken> for RefreshTokenMongoModel {
    fn try_from_entity(refresh_token: RefreshToken) -> Result<Self, AppError> {
        refresh_token.try_into()
    }

    fn to_entity(self) -> RefreshToken {
        self.into()
    }
}
