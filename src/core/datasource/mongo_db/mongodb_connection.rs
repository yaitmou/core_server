use bson::doc;
use mongodb::{Client, Database};

use crate::core::{AppError, Config};

// This should be the only file the should change if we change db infrastructure along with
// crud_datasource implementation
pub struct MongoConnection {
    pub database: Database,
}
impl MongoConnection {
    pub async fn new(config: Config) -> Result<Self, AppError> {
        let client = match Client::with_uri_str(config.database_uri_string).await {
            Ok(result) => result,
            Err(e) => {
                let msg = format!("{:?}", e);
                return Err(AppError::DatabaseError(msg));
            }
        };
        let database = client.database(&config.database_name);

        // Optional: ping the database to verify connection
        if let Err(e) = database.run_command(doc! {"ping": 1}).await {
            let msg = format!("{:?}", e);
            return Err(AppError::DatabaseError(msg));
        }

        Ok(Self { database })
    }
}
