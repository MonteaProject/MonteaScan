mod mod_oval;
use crate::mod_oval::alma::main   as oval_alma;
use crate::mod_oval::rhel::main   as oval_rhel;
use crate::mod_oval::rocky::main  as oval_rocky;
use crate::mod_oval::ubuntu::main as oval_ubuntu;

use anyhow::{Result, Error, anyhow};
use mongodb::{options::ClientOptions, Client as MongoClient, bson::doc};


#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
  let mut client_options: ClientOptions = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
  client_options.app_name = Some("My App".to_string());

  let mongo_client: MongoClient = MongoClient::with_options(client_options).unwrap();

  for db_name in mongo_client.list_database_names(None, None).await.unwrap() {
    println!("list DB: {}", db_name);
  }

  if let Err(e) = oval_alma(mongo_client.clone()) {
    println!("{:#}", e);
  }

  if let Err(e) = oval_rhel(mongo_client.clone()) {
    println!("{:#}", e);
  }

  if let Err(e) = oval_rocky(mongo_client.clone()) {
    println!("{:#}", e);
  }

  if let Err(e) = oval_ubuntu(mongo_client.clone()) {
    println!("{:#}", e);
  }

  Ok(())
}