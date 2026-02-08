use async_trait::async_trait;

use crate::{
    api::auth_token::{
        data::datasources::refresh_token_mongo_db::RefreshTokenMongoModel,
        domain::entities::refresh_token::RefreshToken,
    },
    core::{datasource::crud_datasource::CrudDataSource, AppError},
};

#[async_trait]
pub trait RefreshTokenDatasource:
    CrudDataSource<RefreshToken, RefreshTokenMongoModel, AppError> + Send + Sync
{
}
