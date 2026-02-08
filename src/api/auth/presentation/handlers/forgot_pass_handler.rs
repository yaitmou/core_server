use std::{collections::HashMap, sync::Arc};

use warp::{reject::Rejection, reply::Reply, Filter};

use crate::{
    api::auth::{data::forgot_pwd_dto::ForgotPwdDto, domain::entities::User},
    core::{response::ApiResponse, MsgBuilder, UseCase},
    di::ServiceLocator,
};

pub struct ForgotPassHandler {
    sl: Arc<ServiceLocator>,
}

impl ForgotPassHandler {
    pub fn new(sl: Arc<ServiceLocator>) -> Self {
        Self { sl }
    }

    async fn handle(&self, dto: ForgotPwdDto) -> Result<impl Reply, Rejection> {
        let mut filter = HashMap::new();
        filter.insert("email".to_string(), dto.email);

        let mut user: User = self.sl.get_user().execute(filter).await?;

        user.can_continue()?;

        /* ··························································· [ Update Reset Pwd Token ] */
        user.set_reset_pwd_token()?;

        self.sl.update_user_usecase().execute(user.clone()).await?;

        self.sl
            .email_service()
            .send_reset_pwd_email(&user.email, &user.reset_pwd_token.unwrap())
            .await?;

        let msg = MsgBuilder::custom("A reset password PIN has been sent to your email");
        let response = ApiResponse::<()>::success(msg, None);

        Ok(warp::reply::with_status(
            warp::reply::json(&response),
            warp::http::StatusCode::OK,
        ))
    }

    pub fn route(self: Arc<Self>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        warp::path("forgot-password")
            .and(warp::post())
            .and(warp::path::end())
            .and(warp::body::json())
            .and_then(move |dto: ForgotPwdDto| {
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
