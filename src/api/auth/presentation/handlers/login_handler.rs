use std::{collections::HashMap, sync::Arc};

use warp::{
    http::StatusCode,
    reject::Rejection,
    reply::{with_header, Reply},
    Filter,
};

use crate::{
    api::{
        auth::{
            data::dtos::login_dto::{LoginDto, LoginResponseDto},
            domain::entities::User,
        },
        auth_token::domain::entities::refresh_token::RefreshToken,
    },
    core::{AppError, MsgBuilder, UseCase},
    di::ServiceLocator,
};

pub struct LoginHandler {
    sl: Arc<ServiceLocator>,
}

impl LoginHandler {
    pub fn new(sl: Arc<ServiceLocator>) -> Self {
        Self { sl }
    }

    async fn handle(&self, params: LoginDto) -> Result<impl Reply, Rejection> {
        /* ····························································· [ Check If User Exists ] */
        let mut filter = HashMap::new();
        filter.insert("email".to_string(), params.email.clone());
        let mut user: User = match self.sl.get_user().execute(filter).await {
            Ok(result) => result,
            Err(_) => {
                let msg = MsgBuilder::try_again("credentials");
                let err = AppError::NotFound(msg);
                return Err(warp::reject::custom(err));
            }
        };

        /* ······························································ [ Is Password Correct ] */
        user.verify_pwd(params.password)?;
        user.can_continue()?;
        user.log_in();

        /* ······················································································ */
        // At this point user has entered all required credentials and all were valid. Next, we
        // should generate a new refresh token and access token.
        /* ······················································································ */
        /* ··························································· [ Generate refresh token ] */
        let refresh_token_value = self.sl.jwt_service().generate_jwt(&user)?;

        let refresh_token = RefreshToken::new(
            user.id.clone(),
            refresh_token_value,
            user.role.clone(),
            None,
        );

        let refresh_token: RefreshToken =
            match self.sl.create_refresh_token().execute(refresh_token).await {
                Ok(result) => result,
                Err(e) => {
                    return Err(warp::reject::custom(e));
                }
            };

        /* ···························································· [ Generate access token ] */
        let access_token = self.sl.jwt_service().generate_jwt(&user)?;

        /* ······································································ [ Update User ] */
        self.sl.update_user_usecase().execute(user.clone()).await?;

        /* ···························································· [ Prepare http response ] */
        // Construct the http response with auth jwt
        let response_data = LoginResponseDto {
            user: user.into(),
            refresh_token: refresh_token.token,
        };

        let response = warp::reply::json(&response_data);
        let response = with_header(response, "x-auth-token", &access_token);
        let response = warp::reply::with_status(response, StatusCode::OK);

        Ok(response)
    }

    pub fn route(self: Arc<Self>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        warp::path("login")
            .and(warp::post())
            .and(warp::body::json())
            .and_then(move |dto: LoginDto| {
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
