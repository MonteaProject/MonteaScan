use actix_web::{web, get, App, middleware, HttpResponse, HttpServer};
use mongodb::{bson::doc, options::ClientOptions, Client as MongoClient};
use serde::{Deserialize, Serialize};
use bson::Document;
use futures::StreamExt;

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct OvalRhel {
    definitions: Definitions
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct Definitions {
    definition: Vec<Definition>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct Definition {
    #[serde(rename = "@id")]
    id: Option<String>,
    #[serde(rename = "@class")]
    class: Option<String>,
    metadata: Option<Metadata>,
    criteria: Option<Criteria>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct Metadata {
    title: Option<String>,
    affected: Option<Affected>,
    reference: Option<Vec<Reference>>,
    description: Option<String>,
    advisory: Option<Advisory>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct Affected {
    #[serde(rename = "@family")]
    family: Option<String>,
    platform: Option<Vec<String>>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct Reference {
    #[serde(rename = "@ref_id")]
    ref_id: Option<String>,
    #[serde(rename = "@ref_url")]
    ref_url: Option<String>,
    #[serde(rename = "@source")]
    source: Option<String>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct Advisory{
    #[serde(rename = "@from")]
    from: Option<String>,
    severity: Option<String>,
    rights: Option<String>,
    issued: Option<Issued>,
    updated: Option<Updated>,
    cve: Option<Vec<Cve>>,
    bugzilla: Option<Vec<Bugzilla>>,
    affected_cpe_list: Option<AffectedCpeList>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct Issued {
    #[serde(rename = "@date")]
    date: Option<String>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct Updated {
    #[serde(rename = "@date")]
    date: Option<String>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct Cve {
    #[serde(rename = "@cvss2")]
    cvss2: Option<String>,
    #[serde(rename = "@cvss3")]
    cvss3: Option<String>,
    #[serde(rename = "@cwe")]
    cwe: Option<String>,
    #[serde(rename = "@href")]
    href: Option<String>,
    #[serde(rename = "@impact")]
    impact: Option<String>,
    #[serde(rename = "@public")]
    public: Option<String>,
    #[serde(rename = "$value")]
    cve: Option<String>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct Bugzilla {
    #[serde(rename = "@href")]
    href: Option<String>,
    #[serde(rename = "@id")]
    id: Option<String>,
    #[serde(rename = "$value")]
    bugzilla: Option<String>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct AffectedCpeList {
    cpe: Option<Vec<String>>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct Criteria {
    #[serde(rename = "@operator")]
    operator: Option<String>,
    criterion: Option<Vec<Criterion>>,
    criteria: Option<Vec<Criteria2>>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct Criterion {
    #[serde(rename = "@comment")]
    comment: Option<String>,
    #[serde(rename = "@test_ref")]
    test_ref: Option<String>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct Criteria2 {
    #[serde(rename = "@operator")]
    operator: Option<String>,
    criterion: Option<Vec<Criterion2>>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct Criterion2 {
    #[serde(rename = "@comment")]
    comment: Option<String>,
    #[serde(rename = "@test_ref")]
    test_ref: Option<String>
}

// fn load_rustls_config() -> rustls::ServerConfig {
//     let config = ServerConfig::builder()
//         .with_safe_defaults()
//         .with_no_client_auth();

//     let cert_file = &mut BufReader::new(File::open("cert/cert.pem").unwrap());
//     let key_file = &mut BufReader::new(File::open("cert/key.pem").unwrap());

//     let cert_chain = certs(cert_file)
//         .unwrap()
//         .into_iter()
//         .map(Certificate)
//         .collect();

//     let mut key: Vec<PrivateKey> = pkcs8_private_keys(key_file)
//         .unwrap()
//         .into_iter()
//         .map(PrivateKey)
//         .collect();

//     if key.is_empty() {
//         eprintln!("Could not locate PKCS 8 private keys.");
//         std::process::exit(1);
//     }
//     config.with_single_cert(cert_chain, key.remove(0)).unwrap()
// }


#[get("/get_id/{id}")]
async fn get_id(client: web::Data<MongoClient>, id: web::Path<String>) -> HttpResponse {
    let db = client.database("OvalRHEL");
    let col = String::from("RHEL6");
    let type_collection = db.collection::<Definition>(&col);

    let id = id.into_inner();

    match type_collection
        .find_one(doc! { "@id": &id }, None)
        .await
    {
        Ok(Some(i)) => HttpResponse::Ok().json(i),
        Ok(None) => {
            HttpResponse::NotFound().body("Not Found")
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/rhel6/")]
async fn rhel6(client: web::Data<MongoClient>) -> HttpResponse {
    let db = client.database("OvalRHEL");
    let col = String::from("RHEL6");
    
    let aggr_pipeline: Vec<Document> = Vec::new();
    let mut cursor = db
        .collection::<Document>(&col)
        .aggregate(aggr_pipeline, None)
        .await
        .expect("Error performing aggregation on examplemodel  collection.");

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
    let db = client.database("OvalRHEL");
    let col = String::from("RHEL7");
    
    let aggr_pipeline: Vec<Document> = Vec::new();
    let mut cursor = db
        .collection::<Document>(&col)
        .aggregate(aggr_pipeline, None)
        .await
        .expect("Error performing aggregation on examplemodel  collection.");

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
    let db = client.database("OvalRHEL");
    let col = String::from("RHEL8");
    
    let aggr_pipeline: Vec<Document> = Vec::new();
    let mut cursor = db
        .collection::<Document>(&col)
        .aggregate(aggr_pipeline, None)
        .await
        .expect("Error performing aggregation on examplemodel  collection.");

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
    let db = client.database("OvalRHEL");
    let col = String::from("RHEL9");

    let aggr_pipeline: Vec<Document> = Vec::new();
    let mut cursor = db
        .collection::<Document>(&col)
        .aggregate(aggr_pipeline, None)
        .await
        .expect("Error performing aggregation on examplemodel  collection.");

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
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
    client_options.app_name = Some("My App".to_string());

    let mongo_client = MongoClient::with_options(client_options).unwrap();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::info!("Starting HTTP Server...");

    // let config = load_rustls_config();

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(mongo_client.clone()))
            .service(get_id)
            .service(rhel6)
            .service(rhel7)
            .service(rhel8)
            .service(rhel9)
    })
    // .bind_rustls("127.0.0.1:7878", config).unwrap()
    .bind(("127.0.0.1", 7878)).unwrap()
    .run()
    .await
}
