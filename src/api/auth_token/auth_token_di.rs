use std::sync::Arc;

use mongodb::Database;

use crate::api::auth_token::{
    data::{
        datasources::refresh_token_mongo_db::RefreshTokenMongoDatasourceImpl,
        repositories::refresh_token_repository_impl::RefreshTokenRepositoryImpl,
    },
    domain::usecases::*,
};

pub struct AuthTokenDi {
    pub get_one_refresh_token: Arc<GetOneRefreshToken>,
    pub create_refresh_token: Arc<CreateRefreshToken>,
    pub update_one_refresh_token: Arc<UpdateOneRefreshToken>,
    pub delete_user_refresh_tokens: Arc<DeleteRefreshTokens>,
    pub delete_many_refresh_tokens: Arc<DeleteManyRefreshTokens>,
}

impl AuthTokenDi {
    pub fn new(db: &Database) -> Self {
        /* ························································ [ Datasource Implementation ] */
        let database = Arc::new(RefreshTokenMongoDatasourceImpl::new(&db));

        /* ························································ [ Repository Implementation ] */
        let repository = Arc::new(RefreshTokenRepositoryImpl::new(database));

        /* ········································································· [ Usecases ] */
        let get_one_refresh_token = Arc::new(GetOneRefreshToken::new(repository.clone()));
        let create_refresh_token = Arc::new(CreateRefreshToken::new(repository.clone()));
        let delete_user_refresh_tokens = Arc::new(DeleteRefreshTokens::new(repository.clone()));
        let update_one_refresh_token = Arc::new(UpdateOneRefreshToken::new(repository.clone()));
        let delete_many_refresh_tokens = Arc::new(DeleteManyRefreshTokens::new(repository.clone()));

        Self {
            get_one_refresh_token,
            create_refresh_token,
            update_one_refresh_token,
            delete_user_refresh_tokens,
            delete_many_refresh_tokens,
        }
    }
}
