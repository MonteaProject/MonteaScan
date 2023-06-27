use mongodb::{options::ClientOptions, Client as MongoClient, bson::doc};
use serde::{Deserialize, Serialize};
use serde_json::{Result};
use std::io::Read;
use std::clone::Clone;
use bzip2::read::BzDecoder;
use quick_xml::de::from_str;


#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct OvalRocky {
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
    // cve: Option<Vec<Cve>>,           // None,RockyLinux
    // bugzilla: Option<Vec<Bugzilla>>, // None,RockyLinux
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

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
  let mut client_options: ClientOptions = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
  client_options.app_name = Some("My App".to_string());

  let mongo_client: MongoClient = MongoClient::with_options(client_options).unwrap();

  for db_name in mongo_client.list_database_names(None, None).await.unwrap() {
      println!("list DB: {}", db_name);
  }

  let db: mongodb::Database = mongo_client.database("OvalRocky");

  for collection_name in db.list_collection_names(None).await.unwrap() {
      println!("list Collection: {}", collection_name);
  }

  let mut rocky_ver: Vec<i32> = vec![];
  for i in 8..10 {
      rocky_ver.push(i);
  }

  for v in rocky_ver {
      let v: &str = &v.to_string();
      let url: String = String::from("https://dl.rockylinux.org/pub/oval/org.rockylinux.rlsa-") + v + ".xml.bz2";

      let response = reqwest::get(&url).await.unwrap();
      let bytes = response.bytes().await.unwrap();

      let mut gz: BzDecoder<&[u8]> = BzDecoder::new(&bytes[..]);
      let mut resp_body: String = String::new();
      gz.read_to_string(&mut resp_body).unwrap();

      let oval_rocky: OvalRocky = from_str(&resp_body).unwrap();

      let col: String = String::from("Rocky") + v;
      let typed_collection: mongodb::Collection<Definition> = db.collection::<Definition>(&col);
      
      let filter: bson::Document = doc! {};
      let delete_result: mongodb::results::DeleteResult = typed_collection.delete_many(filter, None).await.unwrap();
      println!("Deleted {} documents, col:{}", delete_result.deleted_count, col);
      
      for i in 0..oval_rocky.definitions.definition.len() {
          let insert_result: mongodb::results::InsertOneResult = typed_collection.insert_one(oval_rocky.definitions.definition[i].clone(), None).await.unwrap();
          println!("document ID:{}, col:{}", insert_result.inserted_id, col);
      }
  }

  Ok(())
}