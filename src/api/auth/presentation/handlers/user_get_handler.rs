use std::{collections::HashMap, sync::Arc};
use warp::{
    http::StatusCode,
    reject::Rejection,
    reply::{json, with_status, Reply},
    Filter,
};

use crate::{
    api::auth::{data::user_response_dto::UserResponseDto, domain::entities::Claims},
    core::{
        errors::early_err_response, middleware::auth_middleware, response::ApiResponse, AppError,
        MsgBuilder, UseCase,
    },
    di::ServiceLocator,
};

/// Get user by query parameters
/// the get user by id is handled by user_get_by_id handler so no need to cover this here
pub struct GetOneUserHandler {
    sl: Arc<ServiceLocator>,
}

impl GetOneUserHandler {
    pub fn new(sl: Arc<ServiceLocator>) -> Self {
        Self { sl }
    }

    //* ┌──────────────────────────────────────────────────────────────────────────────────────────┐
    //* │                                 Handle (i.e. Service0)                                   │
    //* └──────────────────────────────────────────────────────────────────────────────────────────┘
    async fn handle(&self, query: HashMap<String, String>) -> Result<impl Reply, Rejection> {
        /* No Empty query ······································································· */
        // We should not allow empty query
        if query.is_empty() {
            let err = AppError::EmptyQuery;
            return Ok(early_err_response(err).await);
        }

        /* Try to get the user based on the provided query ······································ */
        let user = match self.sl.get_user().execute(query).await {
            Ok(result) => result,
            Err(err) => {
                return Ok(early_err_response(err).await);
            }
        };

        // Prevent returning the entire user's data control what your return back to users (e.g. no
        // password)
        let safe_user = UserResponseDto::from(user);

        //* Success ············································································· */
        //* ····················································································· */
        let msg = MsgBuilder::loaded_success("User");
        let response = ApiResponse::success(msg, Some(safe_user));
        Ok(with_status(json(&response), StatusCode::OK).into_response())
        //* ····················································································· */
    }

    //* ┌──────────────────────────────────────────────────────────────────────────────────────────┐
    //* │                                        The Route                                         │
    //* └──────────────────────────────────────────────────────────────────────────────────────────┘
    pub fn route(self: Arc<Self>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        warp::path!("user" / "search")
            .and(warp::get())
            .and(warp::query::<HashMap<String, String>>())
            .and(warp::path::end())
            .and(auth_middleware(self.sl.jwt_service()))
            .and_then(move |query: HashMap<String, String>, _: Claims| {
                let handler = self.clone();
                async move { handler.handle(query).await }
            })
    }
}
