use async_trait::async_trait;

use mongodb::{Collection, Database};

use crate::{
    api::auth::{
        data::{UserDataSource, UserMongoModel},
        domain::entities::User,
    },
    core::datasource::mongo_db::crud_datasource_mongodb_impl::CrudDatasourceMongoImpl,
};

pub struct UserDataSourceMongoDbImpl {
    collection: Collection<UserMongoModel>,
}

impl UserDataSourceMongoDbImpl {
    pub fn new(db: &Database) -> Self {
        let collection = db.collection("users");
        Self { collection }
    }
}

#[async_trait]
impl CrudDatasourceMongoImpl<User, UserMongoModel> for UserDataSourceMongoDbImpl {
    fn get_collection(&self) -> &Collection<UserMongoModel> {
        &self.collection
    }
}

#[async_trait]
impl UserDataSource for UserDataSourceMongoDbImpl {}
