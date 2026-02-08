use std::sync::Arc;

use warp::{reject::Rejection, reply::Reply, Filter};

use crate::{
    api::auth::{data::dtos::delete_user_dto::DeleteUserDto, domain::entities::Claims},
    core::{
        middleware::{auth_middleware, owner_or_admin_middleware},
        response::ApiResponse,
        CommandUseCase, CoreEventHandler, MsgBuilder, UseCase, UserDeletedEvent,
    },
    di::ServiceLocator,
};

pub struct DeleteUserHandler {
    sl: Arc<ServiceLocator>,
}

impl DeleteUserHandler {
    pub fn new(sl: Arc<ServiceLocator>) -> Self {
        Self { sl }
    }

    async fn handle<E: CoreEventHandler + 'static>(
        self: Arc<Self>,
        user_id: String,
        event_handler: Arc<E>,
    ) -> Result<impl Reply, Rejection> {
        /* ······································································· [ Delete User ] */
        self.sl
            .delete_user_usecase()
            .execute(user_id.to_string())
            .await?;

        /* ······················································ [ Delete User's Refresh Token ] */
        self.sl
            .delete_refresh_tokens()
            .execute(user_id.to_string())
            .await?;

        /* ······························································ [ Auth Event (if any) ] */
        let event = UserDeletedEvent { user_id };

        let event_handler_clone = event_handler.clone();
        let event_clone = event.clone();
        tokio::spawn(async move {
            let _ = event_handler_clone.on_user_deleted(&event_clone).await;
        });

        /* ································································· [ Success Response ] */
        let msg = MsgBuilder::deleted_success("User");
        let response = ApiResponse::<()>::success(msg, None);

        Ok(warp::reply::with_status(
            warp::reply::json(&response),
            warp::http::StatusCode::OK,
        ))
    }

    pub fn route<E: CoreEventHandler + 'static>(
        self: Arc<Self>,
        event_handler: Arc<E>,
    ) -> impl warp::Filter<Extract = impl Reply, Error = Rejection> + Clone {
        warp::path!("user")
            .and(warp::delete())
            .and(warp::path::end())
            .and(warp::body::json())
            .and(auth_middleware(self.sl.jwt_service()))
            .and_then(move |dto: DeleteUserDto, claims: Claims| async move {
                owner_or_admin_middleware(dto.user_id.clone(), claims).await?;
                Ok::<DeleteUserDto, warp::Rejection>(dto)
            })
            .and_then(move |dto: DeleteUserDto| {
                let handler = self.clone();
                let user_id = dto.user_id;
                let event_handler = Arc::clone(&event_handler);
                async move { handler.handle(user_id, event_handler).await }
            })
    }
}
