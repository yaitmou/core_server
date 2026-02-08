use async_trait::async_trait;

use crate::{
    api::auth::{
        data::{datasource::user_datasource::UserDataSource, UserMongoModel},
        domain::entities::User,
    },
    core::{AppError, CrudRepository},
};

#[async_trait]
pub trait UserRepository:
    CrudRepository<User, UserMongoModel, AppError, dyn UserDataSource>
{
}
