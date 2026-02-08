pub struct MsgBuilder {}
impl MsgBuilder {
    /// <...> created successfully!
    pub fn created_success(entity: &str) -> String {
        format!("{entity} created successfully!")
    }

    /// <...> deleted successfully!
    pub fn deleted_success(entity: &str) -> String {
        format!("{entity} deleted successfully!")
    }

    /// <...> loaded successfully!
    pub fn loaded_success(entity: &str) -> String {
        format!("{entity} loaded successfully!")
    }

    /// <...> updated successfully
    pub fn updated_success(entity: &str) -> String {
        format!("{entity} updated successfully")
    }

    /// <...> already exits
    pub fn already_exists(entity: &str) -> String {
        format!("{entity} already exists")
    }

    /// This <...> is not found!
    pub fn not_found(entity: &str) -> String {
        format!("This {entity} is not found!")
    }

    /// You don't have permission to <...>
    pub fn no_permission_to(content: &str) -> String {
        format!("You don't have permission to {content}!")
    }

    /// <...>
    pub fn custom(msg: &str) -> String {
        format!("{msg}")
    }

    /// We cannot process this request for the moment. Please try again later.
    pub fn try_later() -> String {
        format!("We cannot process this request for the moment. Please try again later.")
    }

    /// Something went wrong! Please, verify your <...> and try again.
    pub fn try_again(msg: &str) -> String {
        format!("Something went wrong! Please, verify your {msg} and try again.")
    }
}
