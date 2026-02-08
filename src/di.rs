use std::sync::Arc;

use mongodb::Database;

use crate::{
    api::{
        auth::{
            auth_di::AuthDi,
            domain::usecases::{add_one_user::AddOneUser, user_delete_many::DeleteManyUsers, *},
        },
        auth_token::{auth_token_di::AuthTokenDi, domain::usecases::*},
    },
    core::{
        datasource::mongo_db::mongodb_connection::MongoConnection, jwt_service::JwtService,
        AppError, Config, EmailService, EmailServicerResendImpl, StorageConfig, StorageService,
    },
    websocket::ClientsManager,
};

pub struct ServiceLocator {
    //
    pub db: Database,

    // Global services
    jwt_service: Arc<JwtService>,
    auth_di: Arc<AuthDi>,
    auth_token_di: Arc<AuthTokenDi>,
    ws_clients: Arc<ClientsManager>,
    email_service: Arc<dyn EmailService>,
    storage_service: Arc<StorageService>,
}

impl ServiceLocator {
    pub async fn new(config: Config) -> Result<Self, AppError> {
        //---[ DB Config ]--------------------------------------------------------------------------
        let db = MongoConnection::new(config.clone()).await?.database;

        //---[ Global Services]---------------------------------------------------------------------
        let jwt_service = Arc::new(JwtService::new(config.clone()));
        let email_service = Arc::new(EmailServicerResendImpl::new(&config));
        let mut storage_config = StorageConfig::default();
        storage_config.base_path = config.clone().uploads_base;
        let storage_service = Arc::new(StorageService::new(storage_config));

        //---[ Features ]---------------------------------------------------------------------------
        let auth_di = Arc::new(AuthDi::new(&db));
        let auth_token_di = Arc::new(AuthTokenDi::new(&db));
        let ws_clients = Arc::new(ClientsManager::new());

        Ok(Self {
            db,
            email_service,
            jwt_service,
            auth_di,
            auth_token_di,
            ws_clients,
            storage_service,
        })
    }

    /* ········································································ [ Core Services ] */
    pub fn jwt_service(&self) -> Arc<JwtService> {
        Arc::clone(&self.jwt_service)
    }
    pub fn email_service(&self) -> Arc<dyn EmailService> {
        Arc::clone(&self.email_service)
    }
    pub fn storage_service(&self) -> Arc<StorageService> {
        Arc::clone(&self.storage_service)
    }

    pub fn ws_clients(&self) -> Arc<ClientsManager> {
        Arc::clone(&self.ws_clients)
    }

    /*  ··································································· [ Auth Refresh Token ]*/
    pub fn create_refresh_token(&self) -> Arc<CreateRefreshToken> {
        Arc::clone(&self.auth_token_di.create_refresh_token)
    }
    pub fn get_one_refresh_token(&self) -> Arc<GetOneRefreshToken> {
        Arc::clone(&self.auth_token_di.get_one_refresh_token)
    }
    pub fn update_one_refresh_token(&self) -> Arc<UpdateOneRefreshToken> {
        Arc::clone(&self.auth_token_di.update_one_refresh_token)
    }
    pub fn delete_refresh_tokens(&self) -> Arc<DeleteRefreshTokens> {
        Arc::clone(&self.auth_token_di.delete_user_refresh_tokens)
    }
    pub fn delete_many_refresh_tokens(&self) -> Arc<DeleteManyRefreshTokens> {
        Arc::clone(&self.auth_token_di.delete_many_refresh_tokens)
    }

    /* ············································································ [ Auth User ] */
    pub fn add_one_user(&self) -> Arc<AddOneUser> {
        Arc::clone(&self.auth_di.add_one_user)
    }
    pub fn get_user_by_id_usecase(&self) -> Arc<GetUserById> {
        Arc::clone(&self.auth_di.get_user_by_id)
    }
    pub fn update_user_usecase(&self) -> Arc<UpdateUser> {
        Arc::clone(&self.auth_di.update_user)
    }
    pub fn delete_user_usecase(&self) -> Arc<DeleteUser> {
        Arc::clone(&self.auth_di.delete_user)
    }
    pub fn delete_many_users(&self) -> Arc<DeleteManyUsers> {
        Arc::clone(&self.auth_di.delete_many_users)
    }
    pub fn get_many_users(&self) -> Arc<GetManyUsers> {
        Arc::clone(&self.auth_di.get_many_users)
    }
    pub fn get_user(&self) -> Arc<GetUser> {
        Arc::clone(&self.auth_di.get_user)
    }
}
