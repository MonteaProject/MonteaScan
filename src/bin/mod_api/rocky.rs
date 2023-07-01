#![allow(unused_attributes)]
#![no_main]

use actix_web::{web, get, HttpResponse};
use bson::Document;
use futures::StreamExt;
use mongodb::Client as MongoClient;


#[get("/rocky8/")]
pub async fn rocky8(client: web::Data<MongoClient>) -> HttpResponse {
  let db: mongodb::Database = client.database("oval-rocky");
  let col: String = String::from("rocky8");

  let aggr_pipeline: Vec<Document> = Vec::new();
  let mut cursor: mongodb::Cursor<Document> = db
    .collection::<Document>(&col)
    .aggregate(aggr_pipeline, None)
    .await
    .expect("Error performing aggregation on rocky8 collection.");

  let mut results: Vec<Vec<Document>> = Vec::new();

  while let Some(result) = cursor.next().await {
    match result {
      Ok(document) => {
        results.push(vec![document]);
      }
      Err(_) => {
        return HttpResponse::InternalServerError().finish();
      }
    }
  }
  HttpResponse::Ok().json(results)
}

#[get("/rocky9/")]
pub async fn rocky9(client: web::Data<MongoClient>) -> HttpResponse {
  let db: mongodb::Database = client.database("oval-rocky");
  let col: String = String::from("rocky9");

  let aggr_pipeline: Vec<Document> = Vec::new();
  let mut cursor: mongodb::Cursor<Document> = db
    .collection::<Document>(&col)
    .aggregate(aggr_pipeline, None)
    .await
    .expect("Error performing aggregation on rocky9 collection.");

  let mut results: Vec<Vec<Document>> = Vec::new();

  while let Some(result) = cursor.next().await {
    match result {
      Ok(document) => {
        results.push(vec![document]);
      }
      Err(_) => {
        return HttpResponse::InternalServerError().finish();
      }
    }
  }
  HttpResponse::Ok().json(results)
}