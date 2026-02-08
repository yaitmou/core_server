use std::sync::Arc;

use warp::{
    http::StatusCode,
    reject::Rejection,
    reply::{json, with_status, Reply},
    Filter,
};

use crate::core::errors::early_err_response;
use crate::core::middleware::auth_middleware;
use crate::core::pagination::PaginatedParams;
use crate::core::response::ApiResponse;
use crate::core::UseCase;
use crate::di::ServiceLocator;
use crate::{api::auth::domain::entities::Claims, core::MsgBuilder};

pub struct GetManyUsersEmailsHandler {
    sl: Arc<ServiceLocator>,
}
impl GetManyUsersEmailsHandler {
    pub fn new(sl: Arc<ServiceLocator>) -> Self {
        Self { sl }
    }

    //* ┌──────────────────────────────────────────────────────────────────────────────────────────┐
    //* │                                 Handle (i.e. Service0)                                   │
    //* └──────────────────────────────────────────────────────────────────────────────────────────┘
    async fn handle(&self, params: PaginatedParams, _: Claims) -> Result<impl Reply, Rejection> {
        let mut params = params;

        /* Limit returned values to include only emails ······························· [PROJECT] */
        params
            .query
            .insert("project".to_string(), "email:1, _id: 0".to_string());
        /* ······················································································ */

        /* Get users ············································································ */
        // Since we are using project all other fields won't be provided we will only have email
        // selected.
        let paginated_response = match self.sl.get_many_users().execute(params).await {
            Ok(result) => result,
            Err(err) => {
                return Ok(early_err_response(err).await);
            }
        };
        /* ······················································································ */

        /* Construct the emails vector ·························································· */
        let mut emails = Vec::new();
        for record in paginated_response.records.clone() {
            let email = record.email;
            emails.push(email);
        }
        /* ······················································································ */

        //* Success ············································································· */
        let data = paginated_response.with_records(emails);

        // Success Response
        let msg = MsgBuilder::deleted_success("Users Emails");
        let response = ApiResponse::success(msg, Some(data));
        Ok(with_status(json(&response), StatusCode::OK).into_response())

        //* ····················································································· */
    }

    //* ┌──────────────────────────────────────────────────────────────────────────────────────────┐
    //* │                                        The Route                                         │
    //* └──────────────────────────────────────────────────────────────────────────────────────────┘
    pub fn route(self: Arc<Self>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        warp::path!("user" / "search" / "email")
            .and(warp::get())
            .and(auth_middleware(self.sl.jwt_service()))
            .and(warp::query::<PaginatedParams>())
            .and_then(move |claims: Claims, params: PaginatedParams| {
                let handler = self.clone();
                async move { handler.handle(params, claims).await }
            })
    }
}
