use std::{collections::HashMap, sync::Arc};

use warp::{reject::Rejection, reply::Reply, Filter};

use crate::{
    api::auth::{data::delete_many_user_dto::DeleteManyUsersDto, domain::entities::Claims},
    core::{
        middleware::{admin_middleware, auth_middleware},
        response::ApiResponse,
        CommandUseCase, MsgBuilder, UseCase,
    },
    di::ServiceLocator,
};

pub struct DeleteManyUserHandler {
    sl: Arc<ServiceLocator>,
}

impl DeleteManyUserHandler {
    pub fn new(sl: Arc<ServiceLocator>) -> Self {
        Self { sl }
    }

    async fn handle(&self, dto: DeleteManyUsersDto) -> Result<impl Reply, Rejection> {
        /* ································································· [ Build the filter ] */
        let mut filter = HashMap::new();
        filter.insert("id.in".to_string(), dto.ids.join(","));

        /* ······································································· [ Delete User ] */
        self.sl.delete_many_users().execute(filter).await?;

        /* ······················································ [ Delete User's Refresh Token ] */
        let mut tokens_filter = HashMap::new();
        tokens_filter.insert("user_id.in".to_string(), dto.ids.join(","));
        self.sl
            .delete_many_refresh_tokens()
            .execute(tokens_filter)
            .await?;

        /* ································································· [ Success Response ] */
        let msg = MsgBuilder::deleted_success("User");
        let response = ApiResponse::<()>::success(msg, None);

        Ok(warp::reply::with_status(
            warp::reply::json(&response),
            warp::http::StatusCode::OK,
        ))
    }

    pub fn route(
        self: Arc<Self>,
    ) -> impl warp::Filter<Extract = impl Reply, Error = Rejection> + Clone {
        warp::path!("users")
            .and(warp::delete())
            .and(warp::body::json())
            .and(auth_middleware(self.sl.jwt_service()))
            .and_then(move |dto: DeleteManyUsersDto, claims: Claims| async move {
                admin_middleware(claims).await?;
                Ok::<DeleteManyUsersDto, warp::Rejection>(dto)
            })
            .and_then(move |dto: DeleteManyUsersDto| {
                let handler = self.clone();
                async move { handler.handle(dto).await }
            })
    }
}
