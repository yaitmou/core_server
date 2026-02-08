use std::sync::Arc;

use warp::{reject::Rejection, reply::Reply, Filter};

use crate::{
    api::auth::{
        data::{update_user_dto::UpdateUserDto, user_response_dto::UserResponseDto},
        domain::entities::Claims,
    },
    core::{
        middleware::{auth_middleware, owner_or_admin_middleware},
        response::ApiResponse,
        MsgBuilder, UseCase,
    },
    di::ServiceLocator,
};

pub struct UpdateUserHandler {
    sl: Arc<ServiceLocator>,
}

impl UpdateUserHandler {
    pub fn new(sl: Arc<ServiceLocator>) -> Self {
        Self { sl }
    }

    pub async fn handle(&self, dto: UpdateUserDto) -> Result<impl Reply, Rejection> {
        // First we need to make sure the user exists before updating...
        let mut user = self
            .sl
            .get_user_by_id_usecase()
            .execute(dto.id.clone())
            .await?;

        user = dto.apply_to(user)?;

        match self.sl.update_user_usecase().execute(user.clone()).await {
            Ok(result) => result,
            Err(e) => {
                return Err(warp::reject::custom(e));
            }
        };

        let user_dto = UserResponseDto::from(user);
        let msg = MsgBuilder::updated_success("User");
        let response = ApiResponse::success(msg, Some(user_dto));

        Ok(warp::reply::with_status(
            warp::reply::json(&response),
            warp::http::StatusCode::OK,
        ))
    }

    // set up the handler route with path "user" and method PUT
    pub fn route(
        self: Arc<Self>,
    ) -> impl warp::Filter<Extract = impl Reply, Error = Rejection> + Clone {
        warp::path!("user")
            .and(warp::put())
            .and(warp::body::json())
            .and(auth_middleware(self.sl.jwt_service()))
            .and_then(move |dto: UpdateUserDto, claims: Claims| async move {
                owner_or_admin_middleware(dto.id.clone(), claims.clone()).await?;
                let mut dto = dto;
                if !claims.is_admin() {
                    dto.apply_non_admin_filter();
                }
                Ok::<UpdateUserDto, warp::Rejection>(dto)
            })
            .and_then(move |dto: UpdateUserDto| {
                let handler = self.clone();
                async move { handler.handle(dto).await }
            })
    }
}
