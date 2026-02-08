use async_trait::async_trait;

use crate::{
    api::auth_token::{
        data::datasources::{
            refresh_token_datasource::RefreshTokenDatasource,
            refresh_token_mongo_db::RefreshTokenMongoModel,
        },
        domain::entities::refresh_token::RefreshToken,
    },
    core::{AppError, CrudRepository},
};

#[async_trait]
pub trait RefreshTokenRepository:
    CrudRepository<RefreshToken, RefreshTokenMongoModel, AppError, dyn RefreshTokenDatasource>
{
}
