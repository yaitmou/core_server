use std::{collections::HashMap, sync::Arc};

use crate::{
    api::auth::{data::dtos::logout_dto::LogoutDto, domain::entities::User},
    core::{response::ApiResponse, CommandUseCase, MsgBuilder, UseCase},
    di::ServiceLocator,
};
use warp::{reject::Rejection, reply::Reply, Filter};

pub struct LogoutHandler {
    sl: Arc<ServiceLocator>,
}

impl LogoutHandler {
    pub fn new(sl: Arc<ServiceLocator>) -> Self {
        Self { sl }
    }

    async fn handle(&self, params: LogoutDto) -> Result<impl Reply, Rejection> {
        let mut filter = HashMap::new();
        filter.insert("email".to_string(), params.email.clone());

        let mut user: User = self.sl.get_user().execute(filter).await?;

        user.log_out();

        self.sl
            .update_user_usecase()
            .execute(user.clone().into())
            .await?;

        self.sl.delete_refresh_tokens().execute(user.id).await?;

        let msg = MsgBuilder::custom("Logout Success");
        let response = ApiResponse::<()>::success(msg, None);

        Ok(warp::reply::with_status(
            warp::reply::json(&response),
            warp::http::StatusCode::OK,
        ))
    }

    // set up the handler route
    pub fn route(self: Arc<Self>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        warp::path("logout")
            .and(warp::post())
            .and(warp::body::json())
            .and_then(move |dto: LogoutDto| {
                // we should check if user can continue here ...
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
