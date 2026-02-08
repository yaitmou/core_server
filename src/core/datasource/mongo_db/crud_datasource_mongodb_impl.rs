// core/datasource/crud_datasource_mongodb_impl.rs
use std::collections::HashMap;

use async_trait::async_trait;
use bson::{doc, oid::ObjectId};
use mongodb::{options::ReturnDocument, Collection};
use serde::{de::DeserializeOwned, Serialize};

use crate::core::{
    crud_model::CrudModel,
    datasource::crud_datasource::CrudDataSource,
    pagination::{PaginatedParams, PaginatedResponse},
    query_params_parser::query_to_document,
    AppError,
};

#[async_trait]
pub trait CrudDatasourceMongoImpl<T, M: CrudModel<T>> {
    fn get_collection(&self) -> &Collection<M>;
}

#[async_trait]
impl<T, M, DS> CrudDataSource<T, M, AppError> for DS
where
    T: Clone + Send + Sync,
    M: CrudModel<T> + Serialize + DeserializeOwned,
    DS: CrudDatasourceMongoImpl<T, M> + Send + Sync,
{
    /* ··········································································· [ CREATE ONE ]*/
    async fn create(&self, item: &T) -> Result<T, AppError> {
        let model: M = M::try_from_entity(item.clone())?;

        let result = self
            .get_collection()
            .insert_one(&model)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Could not create document: {}", e)))?;

        if let Some(id) = result.inserted_id.as_object_id() {
            let filter = doc! { "_id": id };
            let created_model = self
                .get_collection()
                .find_one(filter)
                .await
                .map_err(|e| {
                    AppError::DatabaseError(format!("Failed to fetch created document: {}", e))
                })?
                .ok_or_else(|| AppError::DatabaseError("Created document not found".to_string()))?;

            Ok(created_model.to_entity())
        } else {
            Ok(item.clone())
        }
    }

    /* ·············································································· [ FIND ONE ]*/
    async fn find_one(&self, query: HashMap<String, String>) -> Result<T, AppError> {
        let (filter, _, _) = query_to_document(query);

        let model = self
            .get_collection()
            .find_one(filter)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Database error: {}", e)))?
            .ok_or_else(|| AppError::NotFound("Document not found".to_string()))?;

        Ok(model.to_entity())
    }

    /* ········································································ [ FIND ONE BY ID ]*/
    async fn find_one_by_id(&self, id: &str) -> Result<T, AppError> {
        ObjectId::parse_str(id).map_err(|_| {
            let msg = format!("Invalid Object ID format. Found id: {id}");
            AppError::InvalidInput(msg)
        })?;

        let mut query = HashMap::new();
        query.insert("_id".to_string(), id.to_string());

        self.find_one(query).await
    }

    /* ············································································· [ FIND MANY ]*/
    async fn find(&self, params: PaginatedParams) -> Result<PaginatedResponse<T>, AppError> {
        let page = params.page;
        let limit = params.limit;
        let skip = page * limit;

        let (filter, sort, project) = query_to_document(params.query);

        let total = self
            .get_collection()
            .count_documents(filter.clone())
            .await
            .map_err(|e| AppError::DatabaseError(format!("Count failed: {}", e)))?;

        let mut pipeline = vec![doc! { "$match": filter }];

        if let Some(sort) = sort {
            pipeline.push(sort);
        } else {
            pipeline.push(doc! { "$sort": { "created_at": -1 } });
        }

        if let Some(project) = project {
            pipeline.push(project);
        }

        if limit > 0 {
            pipeline.push(doc! { "$skip": skip as i64 });
            pipeline.push(doc! { "$limit": limit as i64 });
        }

        let mut cursor = self
            .get_collection()
            .aggregate(pipeline)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Aggregation failed: {}", e)))?;

        let mut buff = Vec::new();
        while cursor
            .advance()
            .await
            .map_err(|e| AppError::DatabaseError(format!("Cursor error: {}", e)))?
        {
            let model = cursor.deserialize_current().map_err(|e| {
                AppError::DatabaseError(format!("cursor Deserialization error: {}", e))
            })?;
            buff.push(model);
        }

        let has_next = total > (skip as u64 + buff.len() as u64);

        let models = buff
            .into_iter()
            .map(|doc| {
                bson::from_document(doc)
                    .map_err(|e| AppError::DatabaseError(format!("Deserialization error: {}", e)))
            })
            .collect::<Result<Vec<M>, AppError>>()?;

        let records = models
            .iter()
            .map(|model| model.clone().to_entity())
            .collect::<Vec<T>>();

        Ok(PaginatedResponse {
            records: records,
            has_next,
            current_page: page,
            total,
        })
    }

    /* ············································································ [ UPDATE ONE ]*/
    async fn update_one(&self, item: &T) -> Result<T, AppError> {
        let model: M = M::try_from_entity(item.clone())?;
        let mut document = bson::to_document(&model)
            .map_err(|e| AppError::DatabaseError(format!("DB Serialization error: {}", e)))?;

        let id = document
            .get_object_id("_id")
            .map_err(|_| AppError::InvalidInput("Item has no _id field".to_string()))?;

        // Ensure updated_at is removed if it somehow got serialized
        // otherwise it will create a conflict
        document.remove("updated_at");
        let update_doc = doc! {
            "$set": document,
            "$currentDate": { "updated_at": true }
        };

        let updated_document: Option<M> = self
            .get_collection()
            .find_one_and_update(doc! { "_id": id }, update_doc)
            .return_document(ReturnDocument::After)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Update failed: {}", e)))?;

        match updated_document {
            Some(value) => Ok(value.to_entity()),
            None => Err(AppError::DatabaseError(
                "Could not update the document for the moment".to_string(),
            )),
        }
    }

    /* ······································································ [ DELETE ONE BY ID ]*/
    async fn delete_by_id(&self, id: &str) -> Result<T, AppError> {
        let object_id = ObjectId::parse_str(id)
            .map_err(|_| AppError::InvalidInput("Invalid ObjectId format".to_string()))?;

        let deleted_model: M = self
            .get_collection()
            .find_one_and_delete(doc! { "_id": object_id })
            .await
            .map_err(|e| AppError::DatabaseError(format!("Delete failed: {}", e)))?
            .ok_or_else(|| AppError::NotFound("Document not found".to_string()))?;

        Ok(deleted_model.to_entity())
    }

    /* ············································································ [ DELETE ONE ]*/
    async fn delete_one(&self, query: HashMap<String, String>) -> Result<T, AppError> {
        let (filter, _, _) = query_to_document(query);

        let deleted_model: M = self
            .get_collection()
            .find_one_and_delete(filter)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Delete failed: {}", e)))?
            .ok_or_else(|| AppError::NotFound("Document not found".to_string()))?;

        Ok(deleted_model.to_entity())
    }

    /* ··········································································· [ DELETE MANY ]*/
    async fn delete_many(&self, query: HashMap<String, String>) -> Result<u64, AppError> {
        let (filter, _, _) = query_to_document(query);

        let result = self
            .get_collection()
            .delete_many(filter)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Delete many failed: {}", e)))?;

        if result.deleted_count == 0 {
            return Err(AppError::NotFound(
                "No documents found to delete".to_string(),
            ));
        }

        Ok(result.deleted_count)
    }
}
