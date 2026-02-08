use std::sync::Arc;

use warp::{
    http::StatusCode,
    reject::Rejection,
    reply::{json, with_status, Reply},
    Filter,
};

use crate::{
    api::auth::{data::user_response_dto::UserResponseDto, domain::entities::Claims},
    core::{
        middleware::{auth_middleware, owner_or_admin_middleware},
        response::ApiResponse,
        MsgBuilder, UseCase, Validators,
    },
    di::ServiceLocator,
};

pub struct GetUserByIdHandler {
    sl: Arc<ServiceLocator>,
}

impl GetUserByIdHandler {
    pub fn new(sl: Arc<ServiceLocator>) -> Self {
        Self { sl }
    }
    //* ┌──────────────────────────────────────────────────────────────────────────────────────────┐
    //* │                                 Handle (i.e. Service0)                                   │
    //* └──────────────────────────────────────────────────────────────────────────────────────────┘
    async fn handle(self: Arc<Self>, id: String, claims: Claims) -> Result<impl Reply, Rejection> {
        Validators::validate_object_id(&id)?;

        owner_or_admin_middleware(id.clone(), claims).await?;

        let user = self.sl.get_user_by_id_usecase().execute(id).await?;

        /* User dto filtering ··················································· [FILTER ANCHOR] */
        // We should move here user dto filtering instead of limiting access to users only by admin
        // once we open this endpoint to all users
        /* ······················································································ */

        /* Convert entity to dto ································································ */
        // This includes some fields that should be filtered out for non admin users (verified, role
        // , etc.)
        let user_dto = UserResponseDto::from(user);

        //* Success ············································································· */
        let msg = MsgBuilder::loaded_success("User");
        let response = ApiResponse::success(msg, Some(user_dto));
        Ok(with_status(json(&response), StatusCode::OK))

        //* ····················································································· */
    }

    //* ┌──────────────────────────────────────────────────────────────────────────────────────────┐
    //* │                                        The Route                                         │
    //* └──────────────────────────────────────────────────────────────────────────────────────────┘
    pub fn route(self: Arc<Self>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        warp::path!("user" / String)
            .and(warp::get())
            .and(auth_middleware(self.sl.jwt_service()))
            .and_then(move |user_id: String, claims: Claims| {
                let handler = self.clone();
                async move { handler.handle(user_id, claims).await }
            })
    }
}
