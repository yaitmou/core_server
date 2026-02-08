use std::sync::Arc;

use presentation::handlers::*;
use warp::{reject::Rejection, reply::Reply, Filter};

use crate::{core::CoreEventHandler, di::ServiceLocator};

pub mod auth_di;
pub mod data;
pub mod domain;
pub mod presentation;

pub struct UserFeature {
    register_user_handler: Arc<RegisterUserHandler>,
    login_handler: Arc<LoginHandler>,
    activate_account_handler: Arc<ActivateAccountHandler>,
    forgot_password_handler: Arc<ForgotPassHandler>,
    reset_password_handler: Arc<ResetPasswordHandler>,
    logout_handler: Arc<LogoutHandler>,
    resend_activation_token_handler: Arc<ResendActivationTokenHandler>,
    // resend_reset_pwd_token_handler: Arc<ResendResetPwdTokenHandler>,
    update_user_handler: Arc<UpdateUserHandler>,
    change_pwd_handler: Arc<ChangePwdHandler>,
    /// [DELETE] /user
    delete_user_handler: Arc<DeleteUserHandler>,
    /// [DELETE] /users
    delete_many_users_handler: Arc<DeleteManyUserHandler>,
    /// [GET] /user/search/email
    get_many_users_emails_handler: Arc<GetManyUsersEmailsHandler>,
    /// [GET] /user/search
    get_one_user_handler: Arc<GetOneUserHandler>,
    /// [GET] /user/[String]
    get_user_by_id_handler: Arc<GetUserByIdHandler>,
    /// [GET] /user
    get_many_users_handler: Arc<GetManyUsersHandler>,
}

impl UserFeature {
    pub fn new(sl: Arc<ServiceLocator>) -> Self {
        Self {
            register_user_handler: Arc::new(RegisterUserHandler::new(sl.clone())),
            login_handler: Arc::new(LoginHandler::new(sl.clone())),
            activate_account_handler: Arc::new(ActivateAccountHandler::new(sl.clone())),
            forgot_password_handler: Arc::new(ForgotPassHandler::new(sl.clone())),
            reset_password_handler: Arc::new(ResetPasswordHandler::new(sl.clone())),
            logout_handler: Arc::new(LogoutHandler::new(sl.clone())),
            resend_activation_token_handler: Arc::new(ResendActivationTokenHandler::new(
                sl.clone(),
            )),
            // resend_reset_pwd_token_handler: Arc::new(ResendResetPwdTokenHandler::new(sl.clone())),
            get_user_by_id_handler: Arc::new(GetUserByIdHandler::new(sl.clone())),
            get_one_user_handler: Arc::new(GetOneUserHandler::new(sl.clone())),
            update_user_handler: Arc::new(UpdateUserHandler::new(sl.clone())),
            change_pwd_handler: Arc::new(ChangePwdHandler::new(sl.clone())),
            delete_user_handler: Arc::new(DeleteUserHandler::new(sl.clone())),
            delete_many_users_handler: Arc::new(DeleteManyUserHandler::new(sl.clone())),
            get_many_users_handler: Arc::new(GetManyUsersHandler::new(sl.clone())),
            get_many_users_emails_handler: Arc::new(GetManyUsersEmailsHandler::new(sl.clone())),
        }
    }

    pub fn routes<E: CoreEventHandler + 'static>(
        self: Arc<Self>,
        event_handler: Arc<E>,
    ) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        Arc::clone(&self.register_user_handler)
            .route(event_handler.clone())
            .or(Arc::clone(&self.login_handler).route())
            .or(Arc::clone(&self.activate_account_handler).route())
            .or(Arc::clone(&self.forgot_password_handler).route())
            .or(Arc::clone(&self.reset_password_handler).route())
            .or(Arc::clone(&self.logout_handler).route())
            .or(Arc::clone(&self.resend_activation_token_handler).route())
            // [GET] api/user/search/email
            .or(Arc::clone(&self.get_many_users_emails_handler).route())
            // [GET] api/user/search
            .or(Arc::clone(&self.get_one_user_handler).route())
            // [GET] api/user/<String>
            .or(Arc::clone(&self.get_user_by_id_handler).route())
            // [GET] api/user
            .or(Arc::clone(&self.get_many_users_handler).route())
            // [PUT] api/user/<String>
            .or(Arc::clone(&self.update_user_handler).route())
            // [POST] api/change-pwd
            .or(Arc::clone(&self.change_pwd_handler).route())
            // [DELETE] api/user
            .or(Arc::clone(&self.delete_user_handler).route(event_handler))
            // [DELETE] api/users
            .or(Arc::clone(&self.delete_many_users_handler).route())
    }
}
