use mongodb::bson::Document;
use mongodb::Collection;
use crate::api::app::mongo_component::ClientMongoComponent;

pub struct DocumentRepositoryMongo {
    pub collection: Collection<Document>
}

impl ClientMongoComponent for DocumentRepositoryMongo {}

