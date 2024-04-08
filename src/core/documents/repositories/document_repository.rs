use async_trait::async_trait;
use crate::core::documents::entities::DocumentsEntity;
use crate::models::shared::errors::CustomError;

#[async_trait]
pub trait AuthzCardRepository {
    async fn insert_authz_card(&self, authz_card: DocumentsEntity) -> Result<(), CustomError>;
    async fn delete_authz_card(&self, resource: &str, action: &str) -> Result<(), CustomError>;
    async fn fetch_many(&self) -> Vec<DocumentsEntity>;
    async fn fetch_many_by_id(&self, resource: &str) -> Vec<DocumentsEntity>;
    async fn fetch_one_by_id(&self, resource: &str, action: &str) -> Result<DocumentsEntity, CustomError>;
}