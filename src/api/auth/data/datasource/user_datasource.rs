use async_trait::async_trait;

use crate::{
    api::auth::{data::UserMongoModel, domain::entities::User},
    core::{datasource::crud_datasource::CrudDataSource, AppError},
};

#[async_trait]
pub trait UserDataSource: CrudDataSource<User, UserMongoModel, AppError> + Send + Sync {}
