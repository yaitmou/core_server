use std::{collections::HashMap, sync::Arc};
use warp::{reject::Rejection, reply::Reply, Filter};

use crate::{
    api::auth::{data::dtos::reset_pass_dto::ResetPasswordDto, domain::entities::User},
    core::{response::ApiResponse, MsgBuilder, UseCase},
    di::ServiceLocator,
};

pub struct ResetPasswordHandler {
    sl: Arc<ServiceLocator>,
}

impl ResetPasswordHandler {
    pub fn new(sl: Arc<ServiceLocator>) -> Self {
        Self { sl }
    }

    async fn handle(&self, params: ResetPasswordDto) -> Result<impl Reply, Rejection> {
        let mut filter = HashMap::new();
        filter.insert("email".to_string(), params.email.clone());

        let mut user: User = self.sl.get_user().execute(filter).await?;

        user.can_continue()?;
        /* ········································································ [ Reset Pwd ] */
        user.re_set_pwd(params.new_password, params.token)?;

        self.sl
            .email_service()
            .send_pwd_reset_confirmation_email(&params.email)
            .await?;

        self.sl.update_user_usecase().execute(user).await?;

        let msg = MsgBuilder::custom("Password reset successfully");
        let response = ApiResponse::<()>::success(msg, None);

        Ok(warp::reply::with_status(
            warp::reply::json(&response),
            warp::http::StatusCode::OK,
        ))
    }

    // set up the handler route
    pub fn route(self: Arc<Self>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        warp::path("reset-password")
            .and(warp::post())
            .and(warp::path::end())
            .and(warp::body::json())
            .and_then(move |dto: ResetPasswordDto| {
                // Create a new Arc pointer that this closure owns
                let handler = self.clone();
                // This async block needs to own its data because it might run in the future
                async move {
                    // Now we can use handler.handle() safely because we own this Arc
                    handler.handle(dto).await
                }
            })
    }
}
