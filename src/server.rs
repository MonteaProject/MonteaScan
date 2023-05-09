use std::{fs::File, io::BufReader};
use actix_web::{web, get, post, App, middleware, HttpRequest, HttpResponse, HttpServer, http::header::ContentType, error, Error};
use mongodb::{bson::doc, options::IndexOptions, options::ClientOptions, Client as MongoClient, Collection, IndexModel};
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};
use serde::{Deserialize, Serialize};


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
    affected: Option<Vec<Affected>>,
    reference: Option<Vec<Reference>>,
    description: Option<String>,
    advisory: Option<Vec<Advisory>>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct Affected {
    #[serde(rename = "@family")]
    family: Option<String>,
    platform: Option<Vec<Platform>>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct Platform {
    platform: Option<String>
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
    affected_cpe_list: Option<Vec<AffectedCpeList>>,
    affected: Option<String>
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
    #[serde(rename = "@criteria")]
    criteria2: Option<Vec<Criteria2>>
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
    #[serde(rename = "@criterion")]
    criterion2: Option<Vec<Criterion2>>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct Criterion2 {
    #[serde(rename = "@comment")]
    comment: Option<String>,
    #[serde(rename = "@test_ref")]
    test_ref: Option<String>
}


fn load_rustls_config() -> rustls::ServerConfig {
    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth();

    let cert_file = &mut BufReader::new(File::open("cert.pem").unwrap());
    let key_file = &mut BufReader::new(File::open("key.pem").unwrap());

    let cert_chain = certs(cert_file)
        .unwrap()
        .into_iter()
        .map(Certificate)
        .collect();

    let mut key: Vec<PrivateKey> = pkcs8_private_keys(key_file)
        .unwrap()
        .into_iter()
        .map(PrivateKey)
        .collect();

    if key.is_empty() {
        eprintln!("Could not locate PKCS 8 private keys.");
        std::process::exit(1);
    }
    config.with_single_cert(cert_chain, key.remove(0)).unwrap()
}

#[get("/get_user/{username}")]
async fn get_user(client: web::Data<MongoClient>, username: web::Path<String>) -> HttpResponse {
    let db = client.database("OvalRHEL");
    let col = String::from("RHEL");
    let type_collection = db.collection::<OvalRhel>(&col);

    let username = username.into_inner();

    match type_collection
        .find_one(doc! { "username": &username }, None)
        .await
    {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => {
            HttpResponse::NotFound().body(format!("No user found"))
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
    client_options.app_name = Some("My App".to_string());

    let mongo_client = MongoClient::with_options(client_options).unwrap();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::info!("Starting HTTP Server...");

    let config = load_rustls_config();

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(mongo_client.clone()))
            .service(get_user)
    })
    .bind_rustls("127.0.0.1:7878", config).unwrap()
    .run()
    .await
}
