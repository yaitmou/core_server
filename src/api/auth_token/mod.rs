use std::sync::Arc;

use warp::{reject::Rejection, reply::Reply, Filter};

use crate::{
    api::auth_token::presentation::handlers::RefreshAccessTokenHandler, di::ServiceLocator,
};

pub mod auth_token_di;
pub mod data;
pub mod domain;
pub mod presentation;

pub struct AuthTokenFeature {
    refresh_access_token_handler: Arc<RefreshAccessTokenHandler>,
}

impl AuthTokenFeature {
    pub fn new(sl: Arc<ServiceLocator>) -> Self {
        Self {
            refresh_access_token_handler: Arc::new(RefreshAccessTokenHandler::new(sl.clone())),
        }
    }

    pub fn routes(self: Arc<Self>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        Arc::clone(&self.refresh_access_token_handler).route()
    }
}
