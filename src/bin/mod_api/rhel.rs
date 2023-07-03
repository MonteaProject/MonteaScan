#![allow(unused_attributes)]
#![no_main]

use actix_web::{web, get, HttpResponse};
use bson::Document;
use futures::StreamExt;
use mongodb::Client as MongoClient;


#[get("/rhel6/")]
pub async fn rhel6(client: web::Data<MongoClient>) -> HttpResponse {
  let db: mongodb::Database = client.database("oval-rhel");
  let col: String = String::from("rhel6");
  
  let aggr_pipeline: Vec<Document> = Vec::new();
  let mut cursor: mongodb::Cursor<Document> = db
    .collection::<Document>(&col)
    .aggregate(aggr_pipeline, None)
    .await
    .expect("Error performing aggregation on rhel6 collection.");

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

#[get("/rhel7/")]
pub async fn rhel7(client: web::Data<MongoClient>) -> HttpResponse {
  let db: mongodb::Database = client.database("oval-rhel");
  let col: String = String::from("rhel7");
  
  let aggr_pipeline: Vec<Document> = Vec::new();
  let mut cursor: mongodb::Cursor<Document> = db
    .collection::<Document>(&col)
    .aggregate(aggr_pipeline, None)
    .await
    .expect("Error performing aggregation on rhel7 collection.");

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

#[get("/rhel8/")]
pub async fn rhel8(client: web::Data<MongoClient>) -> HttpResponse {
  let db: mongodb::Database = client.database("oval-rhel");
  let col: String = String::from("rhel8");
  
  let aggr_pipeline: Vec<Document> = Vec::new();
  let mut cursor: mongodb::Cursor<Document> = db
    .collection::<Document>(&col)
    .aggregate(aggr_pipeline, None)
    .await
    .expect("Error performing aggregation on rhel8 collection.");

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

#[get("/rhel9/")]
pub async fn rhel9(client: web::Data<MongoClient>) -> HttpResponse {
  let db: mongodb::Database = client.database("oval-rhel");
  let col: String = String::from("rhel9");

  let aggr_pipeline: Vec<Document> = Vec::new();
  let mut cursor: mongodb::Cursor<Document> = db
    .collection::<Document>(&col)
    .aggregate(aggr_pipeline, None)
    .await
    .expect("Error performing aggregation on rhel9 collection.");

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