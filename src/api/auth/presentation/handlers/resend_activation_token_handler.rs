use crate::{
    api::auth::{data::dtos::send_token_dto::SendTokenDto, domain::entities::User},
    core::{response::ApiResponse, MsgBuilder, UseCase},
    di::ServiceLocator,
};
use std::{collections::HashMap, sync::Arc};
use warp::{reject::Rejection, reply::Reply, Filter};
pub struct ResendActivationTokenHandler {
    sl: Arc<ServiceLocator>,
}

impl ResendActivationTokenHandler {
    pub fn new(sl: Arc<ServiceLocator>) -> Self {
        Self { sl }
    }

    async fn handle(&self, params: SendTokenDto) -> Result<impl Reply, Rejection> {
        let mut filter = HashMap::new();
        filter.insert("email".to_string(), params.email.clone());

        let mut user: User = self.sl.get_user().execute(filter).await?;
        user.set_activation_token()?;

        /* ······································································· [ Update User ] */
        self.sl.update_user_usecase().execute(user.clone()).await?;

        /* ···························································· [ Send Email With Token ] */
        self.sl
            .email_service()
            .send_activation_email(&user.email, &user.clone().activation_token.unwrap())
            .await?;

        let msg = MsgBuilder::custom("An activation PIN was sent to your email");
        let response = ApiResponse::<()>::success(msg, None);

        Ok(warp::reply::with_status(
            warp::reply::json(&response),
            warp::http::StatusCode::OK,
        ))
    }

    // set up the handler route
    pub fn route(self: Arc<Self>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        warp::path("resend-activation-token")
            .and(warp::post())
            .and(warp::path::end())
            .and(warp::body::json())
            .and_then(move |dto: SendTokenDto| {
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
