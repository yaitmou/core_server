use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

use crate::core::{
    crud_model::CrudModel,
    datasource::crud_datasource::CrudDataSource,
    pagination::{PaginatedParams, PaginatedResponse},
    AppError,
};

use super::CrudRepository;

pub trait CrudRepositoryImpl<T, M: CrudModel<T>, D: ?Sized + CrudDataSource<T, M, AppError>> {
    fn get_datasource(&self) -> Arc<D>;
}

#[async_trait]
impl<T, M, D, R> CrudRepository<T, M, AppError, D> for R
where
    T: Clone + Send + Sync,
    M: CrudModel<T> + Serialize + DeserializeOwned,
    D: ?Sized + CrudDataSource<T, M, AppError> + Send + Sync,
    R: CrudRepositoryImpl<T, M, D> + Send + Sync,
{
    async fn create_one(&self, obj: &T) -> Result<T, AppError> {
        self.get_datasource().create(obj).await
    }
    async fn find(&self, params: PaginatedParams) -> Result<PaginatedResponse<T>, AppError> {
        self.get_datasource().find(params).await
    }
    async fn find_one(&self, query: HashMap<String, String>) -> Result<T, AppError> {
        self.get_datasource().find_one(query).await
    }
    async fn find_one_by_id(&self, id: &str) -> Result<T, AppError> {
        self.get_datasource().find_one_by_id(id).await
    }
    async fn delete_one(&self, query: HashMap<String, String>) -> Result<T, AppError> {
        self.get_datasource().delete_one(query).await
    }
    async fn delete_one_by_id(&self, id: &str) -> Result<T, AppError> {
        self.get_datasource().delete_by_id(id).await
    }
    async fn update_one(&self, obj: &T) -> Result<T, AppError> {
        self.get_datasource().update_one(obj).await
    }
    async fn delete_many(&self, query: HashMap<String, String>) -> Result<u64, AppError> {
        self.get_datasource().delete_many(query).await
    }
}
