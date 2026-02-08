use crate::core::{
    crud_model::CrudModel,
    pagination::{PaginatedParams, PaginatedResponse},
};
use async_trait::async_trait;
use std::collections::HashMap;
use warp::reject::Reject;

#[async_trait]
pub trait CrudDataSource<T, M, E>
where
    M: CrudModel<T>,
    E: Reject,
{
    async fn create(&self, item: &T) -> Result<T, E>;
    async fn find_one_by_id(&self, id: &str) -> Result<T, E>;
    async fn find_one(&self, query: HashMap<String, String>) -> Result<T, E>;
    async fn find(&self, params: PaginatedParams) -> Result<PaginatedResponse<T>, E>;
    async fn update_one(&self, item: &T) -> Result<T, E>;
    async fn delete_by_id(&self, id: &str) -> Result<T, E>;
    async fn delete_one(&self, query: HashMap<String, String>) -> Result<T, E>;
    async fn delete_many(&self, query: HashMap<String, String>) -> Result<u64, E>;
}
