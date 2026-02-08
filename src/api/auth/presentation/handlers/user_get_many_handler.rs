use std::sync::Arc;

use warp::Filter;
use warp::{reject::Rejection, reply::Reply};

use crate::api::auth::data::user_response_dto::UserResponseDto;
use crate::api::auth::domain::entities::Claims;
use crate::core::middleware::auth_middleware;
use crate::core::pagination::PaginatedParams;
use crate::core::pagination::PaginatedResponse;
use crate::core::response::ApiResponse;
use crate::core::UseCase;
use crate::di::ServiceLocator;

pub struct GetManyUsersHandler {
    sl: Arc<ServiceLocator>,
}
impl GetManyUsersHandler {
    pub fn new(sl: Arc<ServiceLocator>) -> Self {
        Self { sl }
    }

    async fn handle(&self, params: PaginatedParams) -> Result<impl Reply, Rejection> {
        let paginated_response = match self.sl.get_many_users().execute(params).await {
            Ok(result) => result,
            Err(e) => {
                return Err(warp::reject::custom(e));
            }
        };

        let mut records_dtos = Vec::new();
        for record in paginated_response.records {
            let user_dto = UserResponseDto::from(record);
            records_dtos.push(user_dto);
        }

        let response_data = PaginatedResponse {
            records: records_dtos,
            has_next: paginated_response.has_next,
            current_page: paginated_response.current_page,
            total: paginated_response.total,
        };

        //··························································································
        //···[ Response ]···········································································
        //··························································································

        let response = ApiResponse::success(
            "Users data loaded successfully!".to_string(),
            Some(response_data),
        );

        Ok(warp::reply::with_status(
            warp::reply::json(&response),
            warp::http::StatusCode::OK,
        ))
    }

    pub fn route(self: Arc<Self>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        warp::path!("user")
            .and(warp::get())
            .and(auth_middleware(self.sl.jwt_service()))
            .and(warp::query::<PaginatedParams>())
            .and_then(move |_: Claims, params: PaginatedParams| {
                let handler = self.clone();
                async move { handler.handle(params).await }
            })
    }
}
