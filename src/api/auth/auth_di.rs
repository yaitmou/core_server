use std::sync::Arc;

use mongodb::Database;

use crate::api::auth::domain::usecases::{
    add_one_user::AddOneUser, user_delete_many::DeleteManyUsers,
};

use super::{data::*, domain::usecases::*};

pub struct AuthDi {
    pub get_user_by_id: Arc<GetUserById>,
    pub get_user: Arc<GetUser>,
    pub update_user: Arc<UpdateUser>,
    pub delete_user: Arc<DeleteUser>,
    pub delete_many_users: Arc<DeleteManyUsers>,
    pub get_many_users: Arc<GetManyUsers>,
    pub add_one_user: Arc<AddOneUser>,
}

impl AuthDi {
    pub fn new(db: &Database) -> Self {
        let datasource = Arc::new(UserDataSourceMongoDbImpl::new(db));
        let repository = Arc::new(UserRepositoryImpl::new(datasource));

        // usecases
        let add_one_user = Arc::new(AddOneUser::new(repository.clone()));
        let get_user_by_id = Arc::new(GetUserById::new(repository.clone()));
        let get_user = Arc::new(GetUser::new(repository.clone()));
        let update_user = Arc::new(UpdateUser::new(repository.clone()));

        let delete_user = Arc::new(DeleteUser::new(repository.clone()));
        let delete_many_users = Arc::new(DeleteManyUsers::new(repository.clone()));

        let get_many_users = Arc::new(GetManyUsers::new(repository.clone()));

        Self {
            add_one_user,
            get_user_by_id,
            get_user,
            update_user,
            delete_user,
            delete_many_users,
            get_many_users,
        }
    }
}
