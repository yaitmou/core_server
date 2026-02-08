use std::{collections::HashMap, sync::Arc};

use chrono::Utc;
use warp::{
    http::StatusCode,
    reject::Rejection,
    reply::{json, with_status, Reply},
    Filter,
};

use crate::{
    api::auth_token::{
        data::dtos::{
            refresh_access_token_request_dtos::RefreshAccessTokenRequestDto,
            refresh_access_token_response_dto::RefreshAccessTokenResponseDto,
        },
        domain::entities::refresh_token::RefreshToken,
    },
    core::{errors::early_err_response, response::ApiResponse, MsgBuilder, UseCase},
    di::ServiceLocator,
};

pub struct RefreshAccessTokenHandler {
    sl: Arc<ServiceLocator>,
}

impl RefreshAccessTokenHandler {
    pub fn new(sl: Arc<ServiceLocator>) -> Self {
        Self { sl }
    }

    async fn handle(
        self: Arc<Self>,
        dto: RefreshAccessTokenRequestDto,
    ) -> Result<impl Reply, Rejection> {
        // Search Filter
        let mut filter = HashMap::new();
        filter.insert("token".to_string(), dto.refresh_token.to_string());
        filter.insert("revoked".to_string(), "false".to_string());
        filter.insert("expires_at.gt".to_string(), Utc::now().to_rfc3339());

        let mut refresh_token: RefreshToken =
            match self.sl.get_one_refresh_token().execute(filter).await {
                Ok(result) => result,
                Err(err) => {
                    return Ok(early_err_response(err).await);
                }
            };

        let user = self
            .sl
            .get_user_by_id_usecase()
            .execute(refresh_token.user_id.to_string())
            .await?;

        /* ······································································ [ Renew Token ] */
        // Renew refresh token if it is expiring today
        if refresh_token.should_renew() {
            let new_token = self.sl.jwt_service().generate_jwt(&user)?;

            refresh_token.set_token(new_token);

            self.sl
                .update_one_refresh_token()
                .execute(refresh_token.clone())
                .await?;
        }

        // The token is still valid for over a day, so non need to update it we continue to access
        // token generation
        // create a new access token
        let access_token = self.sl.jwt_service().generate_jwt(&user)?;

        let response_body = RefreshAccessTokenResponseDto {
            id: refresh_token.id,
            access_token,
            refresh_token: refresh_token.token,
        };

        let msg = MsgBuilder::created_success("refresh token");
        let response = ApiResponse::success(msg, Some(response_body));

        Ok(with_status(json(&response), StatusCode::OK).into_response())
    }

    pub fn route(self: Arc<Self>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        warp::path("refresh-jwt")
            .and(warp::post())
            .and(warp::body::json())
            .and_then(move |dto: RefreshAccessTokenRequestDto| {
                let handler = self.clone();
                async move { handler.handle(dto).await }
            })
    }
}
