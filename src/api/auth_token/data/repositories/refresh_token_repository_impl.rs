use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    api::auth_token::{
        data::datasources::{
            refresh_token_datasource::RefreshTokenDatasource,
            refresh_token_mongo_db::RefreshTokenMongoModel,
        },
        domain::{
            entities::refresh_token::RefreshToken,
            repositories::refresh_token_repository::RefreshTokenRepository,
        },
    },
    core::CrudRepositoryImpl,
};

pub struct RefreshTokenRepositoryImpl {
    datasource: Arc<dyn RefreshTokenDatasource>,
}

impl RefreshTokenRepositoryImpl {
    // constructor
    pub fn new(datasource: Arc<dyn RefreshTokenDatasource>) -> Self {
        Self { datasource }
    }
}

#[async_trait]
impl CrudRepositoryImpl<RefreshToken, RefreshTokenMongoModel, dyn RefreshTokenDatasource>
    for RefreshTokenRepositoryImpl
{
    fn get_datasource(&self) -> Arc<dyn RefreshTokenDatasource> {
        self.datasource.clone()
    }
}

#[async_trait]
impl RefreshTokenRepository for RefreshTokenRepositoryImpl {}
