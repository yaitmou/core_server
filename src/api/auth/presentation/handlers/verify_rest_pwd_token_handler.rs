use std::{collections::HashMap, sync::Arc};
use warp::{reject::Rejection, reply::Reply, Filter};

use crate::{
    api::auth::{data::VerifyResetPwdPinDto, domain::entities::User},
    core::{response::ApiResponse, MsgBuilder, UseCase},
    di::ServiceLocator,
};

pub struct VerifyResetPwdTokenHandler {
    sl: Arc<ServiceLocator>,
}

impl VerifyResetPwdTokenHandler {
    pub fn new(sl: Arc<ServiceLocator>) -> Self {
        Self { sl }
    }

    async fn handle(self: Arc<Self>, dto: VerifyResetPwdPinDto) -> Result<impl Reply, Rejection> {
        let mut filter = HashMap::new();
        filter.insert("email".to_string(), dto.email.clone());

        let user: User = self.sl.get_user().execute(filter).await?;

        user.is_allowed()?;

        user.verify_reset_pwd_token(dto.token)?;

        let msg = MsgBuilder::custom("Reset password OTP verified successfully!");
        let response = ApiResponse::<()>::success(msg, None);

        Ok(warp::reply::with_status(
            warp::reply::json(&response),
            warp::http::StatusCode::OK,
        ))
    }

    pub fn route(self: Arc<Self>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        warp::path("verify-reset-pwd-token")
            .and(warp::path::end())
            .and(warp::post())
            .and(warp::body::json())
            .and_then(move |dto: VerifyResetPwdPinDto| {
                let handler = self.clone();
                async move { handler.handle(dto).await }
            })
    }
}
