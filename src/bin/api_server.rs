mod mod_api;
use crate::mod_api::alma::{alma8, alma9};
use crate::mod_api::rhel::{rhel6, rhel7, rhel8, rhel9};
use crate::mod_api::rocky::{rocky8, rocky9};
use crate::mod_api::ubuntu::{trusty, xenial, bionic, focal, jammy, kinetic, lunar};

use anyhow::Result;
use actix_web::{web, App, middleware, HttpServer};
use mongodb::{options::ClientOptions, Client as MongoClient};


#[tokio::main(flavor = "multi_thread")]
pub async fn main() -> Result<()> {
  let mut client_options: ClientOptions = ClientOptions::parse("mongodb://localhost:27017").await?;
  client_options.app_name = Some("My App".to_string());

  let mongo_client: MongoClient = MongoClient::with_options(client_options)?;

  env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
  log::info!("Starting HTTP Server...");

  HttpServer::new(move || {
    App::new()
      .wrap(middleware::Logger::default())
      .app_data(web::Data::new(mongo_client.clone()))
      .service(alma8)
      .service(alma9)
      .service(rhel6)
      .service(rhel7)
      .service(rhel8)
      .service(rhel9)
      .service(rocky8)
      .service(rocky9)
      .service(trusty)
      .service(xenial)
      .service(bionic)
      .service(focal)
      .service(jammy)
      .service(kinetic)
      .service(lunar)
  })
  .bind(("127.0.0.1", 7878))?
  .run()
  .await?;

  Ok(())
}