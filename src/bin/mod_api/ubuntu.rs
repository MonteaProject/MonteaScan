#![allow(unused_attributes)]
#![no_main]

use actix_web::{web, get, HttpResponse};
use bson::Document;
use futures::StreamExt;
use mongodb::Client as MongoClient;


#[get("/trusty/")]
pub async fn trusty(client: web::Data<MongoClient>) -> HttpResponse {
  let db: mongodb::Database = client.database("oval-ubuntu");
  let col: String = String::from("ubuntu-trusty");
  
  let aggr_pipeline: Vec<Document> = Vec::new();
  let mut cursor: mongodb::Cursor<Document> = db
    .collection::<Document>(&col)
    .aggregate(aggr_pipeline, None)
    .await
    .expect("Error performing aggregation on ubuntu-trusty collection.");

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

#[get("/xenial/")]
pub async fn xenial(client: web::Data<MongoClient>) -> HttpResponse {
  let db: mongodb::Database = client.database("oval-ubuntu");
  let col: String = String::from("ubuntu-xenial");
  
  let aggr_pipeline: Vec<Document> = Vec::new();
  let mut cursor: mongodb::Cursor<Document> = db
    .collection::<Document>(&col)
    .aggregate(aggr_pipeline, None)
    .await
    .expect("Error performing aggregation on ubuntu-xenial collection.");

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

#[get("/bionic/")]
pub async fn bionic(client: web::Data<MongoClient>) -> HttpResponse {
  let db: mongodb::Database = client.database("oval-ubuntu");
  let col: String = String::from("ubuntu-bionic");
  
  let aggr_pipeline: Vec<Document> = Vec::new();
  let mut cursor: mongodb::Cursor<Document> = db
    .collection::<Document>(&col)
    .aggregate(aggr_pipeline, None)
    .await
    .expect("Error performing aggregation on ubuntu-bionic collection.");

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

#[get("/focal/")]
pub async fn focal(client: web::Data<MongoClient>) -> HttpResponse {
  let db: mongodb::Database = client.database("oval-ubuntu");
  let col: String = String::from("ubuntu-focal");
  
  let aggr_pipeline: Vec<Document> = Vec::new();
  let mut cursor: mongodb::Cursor<Document> = db
    .collection::<Document>(&col)
    .aggregate(aggr_pipeline, None)
    .await
    .expect("Error performing aggregation on ubuntu-focal collection.");

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

#[get("/jammy/")]
pub async fn jammy(client: web::Data<MongoClient>) -> HttpResponse {
  let db: mongodb::Database = client.database("oval-ubuntu");
  let col: String = String::from("ubuntu-jammy");
  
  let aggr_pipeline: Vec<Document> = Vec::new();
  let mut cursor: mongodb::Cursor<Document> = db
    .collection::<Document>(&col)
    .aggregate(aggr_pipeline, None)
    .await
    .expect("Error performing aggregation on ubuntu-jammy collection.");

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

#[get("/kinetic/")]
pub async fn kinetic(client: web::Data<MongoClient>) -> HttpResponse {
  let db: mongodb::Database = client.database("oval-ubuntu");
  let col: String = String::from("ubuntu-kinetic");
  
  let aggr_pipeline: Vec<Document> = Vec::new();
  let mut cursor: mongodb::Cursor<Document> = db
    .collection::<Document>(&col)
    .aggregate(aggr_pipeline, None)
    .await
    .expect("Error performing aggregation on ubuntu-kinetic collection.");

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

#[get("/lunar/")]
pub async fn lunar(client: web::Data<MongoClient>) -> HttpResponse {
  let db: mongodb::Database = client.database("oval-ubuntu");
  let col: String = String::from("ubuntu-lunar");
  
  let aggr_pipeline: Vec<Document> = Vec::new();
  let mut cursor: mongodb::Cursor<Document> = db
    .collection::<Document>(&col)
    .aggregate(aggr_pipeline, None)
    .await
    .expect("Error performing aggregation on ubuntu-lunar collection.");

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