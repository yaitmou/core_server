use std::collections::HashMap;

use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use warp::reject::Reject;

use crate::core::{
    crud_model::CrudModel,
    pagination::{PaginatedParams, PaginatedResponse},
};

#[async_trait]
pub trait CrudRepository<T, M, E, D: ?Sized = ()>: Send + Sync
where
    M: CrudModel<T> + Serialize + DeserializeOwned,
    E: Reject,
{
    async fn create_one(&self, obj: &T) -> Result<T, E>;
    async fn find(&self, params: PaginatedParams) -> Result<PaginatedResponse<T>, E>;
    async fn find_one(&self, query: HashMap<String, String>) -> Result<T, E>;
    async fn find_one_by_id(&self, id: &str) -> Result<T, E>;
    async fn delete_one(&self, query: HashMap<String, String>) -> Result<T, E>;
    async fn delete_one_by_id(&self, id: &str) -> Result<T, E>;
    async fn delete_many(&self, query: HashMap<String, String>) -> Result<u64, E>;
    async fn update_one(&self, obj: &T) -> Result<T, E>;
    // async fn update_many(
    //     &self,
    //     query: HashMap<String, String>,
    //     updates: HashMap<String, String>,
    // ) -> Result<u64, E>;
}
