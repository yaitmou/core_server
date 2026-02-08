use std::{collections::HashMap, sync::Arc};

use warp::{reject::Rejection, reply::Reply, Filter};

use crate::{
    api::auth::{
        data::{dtos::register_dto::RegisterDto, user_response_dto::UserResponseDto},
        domain::entities::User,
    },
    core::{
        rand_token_service::TokenService, response::ApiResponse, AppError, CoreEventHandler,
        MsgBuilder, UseCase, UserRegisteredEvent,
    },
    di::ServiceLocator,
};

pub struct RegisterUserHandler {
    sl: Arc<ServiceLocator>,
}

impl RegisterUserHandler {
    pub fn new(sl: Arc<ServiceLocator>) -> Self {
        Self { sl }
    }

    // handle new register request
    async fn handle<E: CoreEventHandler + 'static>(
        &self,
        params: RegisterDto,
        event_handler: Arc<E>,
    ) -> Result<impl Reply, Rejection> {
        /* ····················································· [ Account Already Exists Check ] */
        let mut filter = HashMap::new();
        filter.insert("email".to_string(), params.email.clone());

        if let Ok(_) = self.sl.get_user().execute(filter).await {
            let msg = MsgBuilder::already_exists("This email");
            let err = AppError::Forbidden(msg);
            return Err(warp::reject::custom(err));
        }

        /* ································································ [ Create a new user ] */
        let mut user = User::new(params.email.clone(), params.first_name, params.last_name);
        user.set_pwd(params.password)?;

        /* ························································· [ Account activation token ] */
        let activation_token = TokenService::generate_token();
        user.activation_token = Some(activation_token.clone());

        /* ········································································· [ Add user ] */
        user = self.sl.add_one_user().execute(user).await?;

        /* ······························································ [ Auth Event (if any) ] */
        let event = UserRegisteredEvent {
            user_id: user.id.clone(),
        };
        let event_handler_clone = event_handler.clone();
        let event_clone = event.clone();
        tokio::spawn(async move {
            let _ = event_handler_clone.on_user_registered(&event_clone).await;
        });

        /* ···························································· [ Send Activation Email ] */

        self.sl
            .email_service()
            .send_activation_email(&user.email, &activation_token)
            .await?;

        /* ································································· [ Success Response ] */
        let msg = MsgBuilder::created_success("Account");
        let response = ApiResponse::success(msg, Some(UserResponseDto::from(user)));

        Ok(warp::reply::with_status(
            warp::reply::json(&response),
            warp::http::StatusCode::OK,
        ))
    }

    // set up the handler route
    pub fn route<E: CoreEventHandler + 'static>(
        self: Arc<Self>,
        event_handler: Arc<E>,
    ) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        warp::path("register")
            .and(warp::post())
            .and(warp::body::json())
            .and_then(move |dto: RegisterDto| {
                // Create a new Arc pointer that this closure owns
                let handler = self.clone();
                let event_handler = Arc::clone(&event_handler);
                // This async block needs to own its data because it might run in the future
                async move {
                    // Now we can use handler.handle() safely because we own this Arc
                    handler.handle(dto, event_handler).await
                }
            })
    }
}
