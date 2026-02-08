use async_trait::async_trait;

use mongodb::{Collection, Database};

use crate::{
    api::auth_token::{
        data::datasources::{
            refresh_token_datasource::RefreshTokenDatasource,
            refresh_token_mongo_db::RefreshTokenMongoModel,
        },
        domain::entities::refresh_token::RefreshToken,
    },
    core::datasource::mongo_db::crud_datasource_mongodb_impl::CrudDatasourceMongoImpl,
};

pub struct RefreshTokenMongoDatasourceImpl {
    collection: Collection<RefreshTokenMongoModel>,
}

impl RefreshTokenMongoDatasourceImpl {
    pub fn new(db: &Database) -> Self {
        let collection = db.collection("refresh_tokens");
        Self { collection }
    }
}

#[async_trait]
impl CrudDatasourceMongoImpl<RefreshToken, RefreshTokenMongoModel>
    for RefreshTokenMongoDatasourceImpl
{
    fn get_collection(&self) -> &Collection<RefreshTokenMongoModel> {
        &self.collection
    }
}
#[async_trait]
impl RefreshTokenDatasource for RefreshTokenMongoDatasourceImpl {}
