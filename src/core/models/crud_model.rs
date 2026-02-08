use std::fmt::Debug;

use crate::core::AppError;

pub trait CrudModel<T>: Clone + Send + Sync + Debug {
    fn try_from_entity(entity: T) -> Result<Self, AppError>;
    fn to_entity(self) -> T;
}
