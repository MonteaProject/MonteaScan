use actix_web::{web, get, App, middleware, HttpResponse, HttpServer};
use mongodb::{bson::doc, options::ClientOptions, Client as MongoClient};
use serde::{Deserialize, Serialize};
use bson::Document;
use futures::StreamExt;

// RedHat
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct OvalRhel {
  definitions: RhelDefinitions
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct RhelDefinitions {
  definition: Vec<RhelDefinition>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct RhelDefinition {
  #[serde(rename = "@id")]
  id:       Option<String>,
  #[serde(rename = "@class")]
  class:    Option<String>,
  metadata: Option<RhelMetadata>,
  criteria: Option<RhelCriteria>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct RhelMetadata {
  title:       Option<String>,
  affected:    Option<RhelAffected>,
  reference:   Option<Vec<RhelReference>>,
  description: Option<String>,
  advisory:    Option<RhelAdvisory>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct RhelAffected {
  #[serde(rename = "@family")]
  family:   Option<String>,
  platform: Option<Vec<String>>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct RhelReference {
  #[serde(rename = "@ref_id")]
  ref_id:  Option<String>,
  #[serde(rename = "@ref_url")]
  ref_url: Option<String>,
  #[serde(rename = "@source")]
  source:  Option<String>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct RhelAdvisory {
  #[serde(rename = "@from")]
  from:              Option<String>,
  severity:          Option<String>,
  rights:            Option<String>,
  issued:            Option<RhelIssued>,
  updated:           Option<RhelUpdated>,
  cve:               Option<Vec<RhelCve>>,
  bugzilla:          Option<Vec<RhelBugzilla>>,
  affected_cpe_list: Option<RhelAffectedCpeList>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct RhelIssued {
  #[serde(rename = "@date")]
  date: Option<String>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct RhelUpdated {
  #[serde(rename = "@date")]
  date: Option<String>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct RhelCve {
  #[serde(rename = "@cvss2")]
  cvss2:  Option<String>,
  #[serde(rename = "@cvss3")]
  cvss3:  Option<String>,
  #[serde(rename = "@cwe")]
  cwe:    Option<String>,
  #[serde(rename = "@href")]
  href:   Option<String>,
  #[serde(rename = "@impact")]
  impact: Option<String>,
  #[serde(rename = "@public")]
  public: Option<String>,
  #[serde(rename = "$value")]
  cve:    Option<String>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct RhelBugzilla {
  #[serde(rename = "@href")]
  href:     Option<String>,
  #[serde(rename = "@id")]
  id:       Option<String>,
  #[serde(rename = "$value")]
  bugzilla: Option<String>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct RhelAffectedCpeList {
  cpe: Option<Vec<String>>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct RhelCriteria {
  #[serde(rename = "@operator")]
  operator:  Option<String>,
  criterion: Option<Vec<RhelCriterion>>,
  criteria:  Option<Vec<RhelCriteria2>>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct RhelCriterion {
  #[serde(rename = "@comment")]
  comment:  Option<String>,
  #[serde(rename = "@test_ref")]
  test_ref: Option<String>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct RhelCriteria2 {
  #[serde(rename = "@operator")]
  operator:  Option<String>,
  criterion: Option<Vec<RhelCriterion2>>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct RhelCriterion2 {
  #[serde(rename = "@comment")]
  comment:  Option<String>,
  #[serde(rename = "@test_ref")]
  test_ref: Option<String>
}

// AlmaLinux
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct OvalAlma {
  definitions: AlmaDefinitions
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct AlmaDefinitions {
  definition: Vec<AlmaDefinition>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct AlmaDefinition {
  #[serde(rename = "@id")]
  id:       Option<String>,
  #[serde(rename = "@class")]
  class:    Option<String>,
  metadata: Option<AlmaMetadata>,
  criteria: Option<AlmaCriteria>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct AlmaMetadata {
  title:       Option<String>,
  reference:   Option<Vec<AlmaReference>>,
  description: Option<String>,
  advisory:    Option<AlmaAdvisory>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct AlmaReference {
  #[serde(rename = "@ref_id")]
  ref_id:  Option<String>,
  #[serde(rename = "@ref_url")]
  ref_url: Option<String>,
  #[serde(rename = "@source")]
  source:  Option<String>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct AlmaAdvisory {
  #[serde(rename = "@from")]
  from:              Option<String>,
  severity:          Option<String>,
  rights:            Option<String>,
  issued:            Option<AlmaIssued>,
  updated:           Option<AlmaUpdated>,
  cve:               Option<Vec<AlmaCve>>,
  bugzilla:          Option<Vec<AlmaBugzilla>>,
  affected_cpe_list: Option<AlmaAffectedCpeList>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct AlmaIssued {
  #[serde(rename = "@date")]
  date: Option<String>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct AlmaUpdated {
  #[serde(rename = "@date")]
  date: Option<String>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct AlmaCve {
  #[serde(rename = "@cvss3")]
  cvss3:  Option<String>,
  #[serde(rename = "@cwe")]
  cwe:    Option<String>,
  #[serde(rename = "@href")]
  href:   Option<String>,
  #[serde(rename = "@impact")]
  impact: Option<String>,
  #[serde(rename = "@public")]
  public: Option<String>,
  #[serde(rename = "$value")]
  cve:    Option<String>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct AlmaBugzilla {
  #[serde(rename = "@href")]
  href: Option<String>,
  #[serde(rename = "@id")]
  id:   Option<String>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct AlmaAffectedCpeList {
  cpe: Option<Vec<String>>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]  // AlmaLinux 特有
struct AlmaCriteria {
  #[serde(rename = "@operator")]
  operator:  Option<String>,
  criterion: Option<Vec<AlmaCriterion>>,
  criteria:  Option<Vec<AlmaCriteria2>>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct AlmaCriterion {
  #[serde(rename = "@comment")]
  comment:  Option<String>,
  #[serde(rename = "@test_ref")]
  test_ref: Option<String>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct AlmaCriteria2 {
  #[serde(rename = "@operator")]
  operator:  Option<String>,
  criterion: Option<Vec<AlmaCriterion2>>,
  criteria:  Option<Vec<AlmaCriteria3>>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct AlmaCriterion2 {
  #[serde(rename = "@comment")]
  comment:  Option<String>,
  #[serde(rename = "@test_ref")]
  test_ref: Option<String>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct AlmaCriteria3 {
  #[serde(rename = "@operator")]
  operator: Option<String>,
  criteria: Option<Vec<AlmaCriteria4>>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct AlmaCriteria4 {
  #[serde(rename = "@operator")]
  operator:  Option<String>,
  criterion: Option<Vec<AlmaCriterion3>>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct AlmaCriterion3 {
  #[serde(rename = "@comment")]
  comment:  Option<String>,
  #[serde(rename = "@test_ref")]
  test_ref: Option<String>
}

// RockyLinux
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct OvalRocky {
  definitions: RockyDefinitions
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct RockyDefinitions {
  definition: Vec<RockyDefinition>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct RockyDefinition {
  #[serde(rename = "@id")]
  id:       Option<String>,
  #[serde(rename = "@class")]
  class:    Option<String>,
  metadata: Option<RockyMetadata>,
  criteria: Option<RockyCriteria>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct RockyMetadata {
  title:       Option<String>,
  affected:    Option<RockyAffected>,
  reference:   Option<Vec<RockyReference>>,
  description: Option<String>,
  advisory:    Option<RockyAdvisory>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct RockyAffected {
  #[serde(rename = "@family")]
  family:   Option<String>,
  platform: Option<Vec<String>>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct RockyReference {
  #[serde(rename = "@ref_id")]
  ref_id:  Option<String>,
  #[serde(rename = "@ref_url")]
  ref_url: Option<String>,
  #[serde(rename = "@source")]
  source:  Option<String>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct RockyAdvisory {
  #[serde(rename = "@from")]
  from:              Option<String>,
  severity:          Option<String>,
  rights:            Option<String>,
  issued:            Option<RockyIssued>,
  updated:           Option<RockyUpdated>,
  affected_cpe_list: Option<RockyAffectedCpeList>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct RockyIssued {
  #[serde(rename = "@date")]
  date: Option<String>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct RockyUpdated {
  #[serde(rename = "@date")]
  date: Option<String>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct RockyAffectedCpeList {
  cpe: Option<Vec<String>>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct RockyCriteria {
  #[serde(rename = "@operator")]
  operator:  Option<String>,
  criterion: Option<Vec<RockyCriterion>>,
  criteria:  Option<Vec<RockyCriteria2>>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct RockyCriterion {
  #[serde(rename = "@comment")]
  comment:  Option<String>,
  #[serde(rename = "@test_ref")]
  test_ref: Option<String>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct RockyCriteria2 {
  #[serde(rename = "@operator")]
  operator:  Option<String>,
  criterion: Option<Vec<RockyCriterion2>>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct RockyCriterion2 {
  #[serde(rename = "@comment")]
  comment:  Option<String>,
  #[serde(rename = "@test_ref")]
  test_ref: Option<String>
}


#[get("/rhel6/")]
async fn rhel6(client: web::Data<MongoClient>) -> HttpResponse {
  let db: mongodb::Database = client.database("OvalRHEL");
  let col: String = String::from("RHEL6");
  
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
async fn rhel7(client: web::Data<MongoClient>) -> HttpResponse {
  let db: mongodb::Database = client.database("OvalRHEL");
  let col: String = String::from("RHEL7");
  
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
async fn rhel8(client: web::Data<MongoClient>) -> HttpResponse {
  let db: mongodb::Database = client.database("OvalRHEL");
  let col: String = String::from("RHEL8");
  
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
async fn rhel9(client: web::Data<MongoClient>) -> HttpResponse {
  let db: mongodb::Database = client.database("OvalRHEL");
  let col: String = String::from("RHEL9");

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

#[get("/alma8/")]
async fn alma8(client: web::Data<MongoClient>) -> HttpResponse {
  let db: mongodb::Database = client.database("OvalAlma");
  let col: String = String::from("Alma8");

  let aggr_pipeline: Vec<Document> = Vec::new();
  let mut cursor: mongodb::Cursor<Document> = db
    .collection::<Document>(&col)
    .aggregate(aggr_pipeline, None)
    .await
    .expect("Error performing aggregation on alma8 collection.");

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

#[get("/alma9/")]
async fn alma9(client: web::Data<MongoClient>) -> HttpResponse {
  let db: mongodb::Database = client.database("OvalAlma");
  let col: String = String::from("Alma9");

  let aggr_pipeline: Vec<Document> = Vec::new();
  let mut cursor: mongodb::Cursor<Document> = db
    .collection::<Document>(&col)
    .aggregate(aggr_pipeline, None)
    .await
    .expect("Error performing aggregation on alma9 collection.");

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

#[get("/rocky8/")]
async fn rocky8(client: web::Data<MongoClient>) -> HttpResponse {
  let db: mongodb::Database = client.database("OvalRocky");
  let col: String = String::from("Rocky8");

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
async fn rocky9(client: web::Data<MongoClient>) -> HttpResponse {
  let db: mongodb::Database = client.database("OvalRocky");
  let col: String = String::from("Rocky9");

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


#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
  let mut client_options: ClientOptions = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
  client_options.app_name = Some("My App".to_string());

  let mongo_client: MongoClient = MongoClient::with_options(client_options).unwrap();

  env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
  log::info!("Starting HTTP Server...");

  HttpServer::new(move || {
    App::new()
      .wrap(middleware::Logger::default())
      .app_data(web::Data::new(mongo_client.clone()))
      .service(rhel6)
      .service(rhel7)
      .service(rhel8)
      .service(rhel9)
      .service(alma8)
      .service(alma9)
      .service(rocky8)
      .service(rocky9)
  })
  .bind(("127.0.0.1", 7878)).unwrap()
  .run()
  .await
}