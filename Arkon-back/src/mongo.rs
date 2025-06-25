use crate::models::Product;
use mongodb::{Database, error::Error as MongoError};
use mongodb::bson::{to_document};

pub async fn insert_product(db: &Database, product: Product) -> Result<(), MongoError> {
    let collection = db.collection::<mongodb::bson::document::Document>("produtos");
    let doc = to_document(&product).map_err(|e| MongoError::from(e))?;
    collection.insert_one(doc, None).await?;

    Ok(())
}
