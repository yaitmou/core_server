use crate::{
    api::auth::{data::dtos::activate_account_dto::ActivateAccountDto, domain::entities::User},
    core::{response::ApiResponse, AppError, MsgBuilder, UseCase},
    di::ServiceLocator,
};
use std::{collections::HashMap, sync::Arc};
use warp::{reject::Rejection, reply::Reply, Filter};

pub struct ActivateAccountHandler {
    sl: Arc<ServiceLocator>,
}

impl ActivateAccountHandler {
    pub fn new(sl: Arc<ServiceLocator>) -> Self {
        Self { sl }
    }

    async fn handle(&self, params: ActivateAccountDto) -> Result<impl Reply, Rejection> {
        /* ····························································· [ Filter user by email ] */
        let mut filter = HashMap::new();
        filter.insert("email".to_string(), params.email.clone());

        let mut user: User = self
            .sl
            .get_user()
            .execute(filter)
            .await
            .map_err(|_| AppError::NotFound(MsgBuilder::not_found("account")))?;

        /* ····························································· [ Make sure user is OK ] */
        if user.banned {
            let err = AppError::Forbidden(MsgBuilder::try_later());
            return Err(warp::reject::custom(err));
        }

        if user.verified {
            let msg = MsgBuilder::custom("This account is already verified!");
            let err = AppError::Forbidden(msg);
            return Err(warp::reject::custom(err));
        }

        /* ···························································· [ Verify Activation Pin ] */
        if user.activation_token != Some(params.token) {
            let err = AppError::Forbidden(MsgBuilder::try_again("activation PIN"));
            return Err(warp::reject::custom(err));
        }

        /* ································································· [ Activate Account ] */
        user.verified = true;
        user.activation_token = None;

        /* ······································································ [ Update User ] */
        self.sl.update_user_usecase().execute(user).await?;
        let msg = MsgBuilder::custom("Account activated successfully");
        let response = ApiResponse::<()>::success(msg, None);

        Ok(warp::reply::with_status(
            warp::reply::json(&response),
            warp::http::StatusCode::OK,
        ))
    }

    // set up the handler route
    pub fn route(self: Arc<Self>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        warp::path("activate-account")
            .and(warp::post())
            .and(warp::path::end())
            .and(warp::body::json())
            .and_then(move |dto: ActivateAccountDto| {
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
