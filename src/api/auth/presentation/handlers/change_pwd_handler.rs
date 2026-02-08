use std::sync::Arc;

use warp::{reject::Rejection, reply::Reply, Filter};

use crate::{
    api::auth::{
        data::dtos::change_pwd_dto::ChangePwdRDto,
        domain::entities::{Claims, User},
    },
    core::{middleware::auth_middleware, response::ApiResponse, AppError, MsgBuilder, UseCase},
    di::ServiceLocator,
};

pub struct ChangePwdHandler {
    sl: Arc<ServiceLocator>,
}

impl ChangePwdHandler {
    pub fn new(sl: Arc<ServiceLocator>) -> Self {
        Self { sl }
    }

    async fn handle(&self, params: ChangePwdRDto, claims: Claims) -> Result<impl Reply, Rejection> {
        // get user based on the provided user_id
        let user_id = match params.user_id {
            Some(value) => {
                if value != claims.user_id && !claims.is_admin() {
                    return Err(warp::reject::custom(AppError::Forbidden(
                        MsgBuilder::no_permission_to("continue"),
                    )));
                }

                value
            }
            None => claims.user_id,
        };

        let mut user: User = self.sl.get_user_by_id_usecase().execute(user_id).await?;

        user.can_continue()?;

        /* ·································································· [ Update Password ] */
        user.change_pwd(params.new_pwd, params.old_pwd)?;

        self.sl.update_user_usecase().execute(user).await?;

        let msg = MsgBuilder::custom("Your password has been updated successfully");
        let response = ApiResponse::<()>::success(msg, None);

        Ok(warp::reply::with_status(
            warp::reply::json(&response),
            warp::http::StatusCode::OK,
        ))
    }

    pub fn route(
        self: Arc<Self>,
    ) -> impl warp::Filter<Extract = impl Reply, Error = Rejection> + Clone {
        warp::path!("change-pwd")
            .and(warp::post())
            .and(warp::path::end())
            .and(warp::body::json())
            .and(auth_middleware(self.sl.jwt_service()))
            .and_then(move |dto: ChangePwdRDto, claims: Claims| {
                let handler = self.clone();
                async move { handler.handle(dto, claims).await }
            })
    }
}
