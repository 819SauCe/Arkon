use crate::models::Product;
use mongodb::{Database, error::Error as MongoError};
use mongodb::bson::{doc, Document};
use bson::to_document;
use futures_util::stream::TryStreamExt;

pub async fn insert_product(db: &Database, product: Product) -> Result<(), MongoError> {
    let collection = db.collection::<mongodb::bson::document::Document>("produtos");
    let doc = to_document(&product).map_err(|e| MongoError::from(e))?;
    collection.insert_one(doc, None).await?;

    Ok(())
}

pub async fn buscar_produtos_ativos(db: &Database) -> Result<Vec<Document>, MongoError> {
    let collection = db.collection::<mongodb::bson::document::Document>("produtos");
    let filtro = doc! { "active": true };
    let mut cursor = collection.find(filtro, None).await?;
    let mut produtos = Vec::new();

    while let Some(produto) = cursor.try_next().await? {
        produtos.push(produto);
    }

    Ok(produtos)
}
