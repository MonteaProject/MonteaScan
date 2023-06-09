use anyhow::Result;
use mongodb::{Client as MongoClient, bson::doc};
use serde::{Deserialize, Serialize};
use std::io::Read;
use std::clone::Clone;
use bzip2::read::BzDecoder;
use quick_xml::de::from_str;


#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct OvalUbuntu {
  definitions: UbuntuDefinitions
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct UbuntuDefinitions {
  definition: Vec<UbuntuDefinition>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct UbuntuDefinition {
  #[serde(rename = "@id")]
  id:       Option<String>,
  #[serde(rename = "@class")]
  class:    Option<String>,
  metadata: Option<UbuntuMetadata>,
  criteria: Option<UbuntuCriteria>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct UbuntuMetadata {
  title:       Option<String>,
  affected:    Option<UbuntuAffected>,
  reference:   Option<Vec<UbuntuReference>>,
  description: Option<String>,
  advisory:    Option<UbuntuAdvisory>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct UbuntuAffected {
  #[serde(rename = "@family")]
  family:   Option<String>,
  platform: Option<Vec<String>>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct UbuntuReference {
  #[serde(rename = "@ref_id")]
  ref_id:  Option<String>,
  #[serde(rename = "@ref_url")]
  ref_url: Option<String>,
  #[serde(rename = "@source")]
  source:  Option<String>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct UbuntuAdvisory {
  rights:          Option<String>,
  component:       Option<String>,
  current_version: Option<String>,
  cve:             Option<Vec<UbuntuCve>>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct UbuntuCve {
  #[serde(rename = "@cvss_score")]
  cvss_score:  Option<String>,
  #[serde(rename = "@cvss_vector")]
  cvss_vector: Option<String>,
  #[serde(rename = "@href")]
  href:        Option<String>,
  #[serde(rename = "@severity")]
  severity:    Option<String>,
  #[serde(rename = "@public")]
  public:      Option<String>,
  #[serde(rename = "@usns")]
  usns:        Option<String>,
  #[serde(rename = "$value")]
  cve:         Option<String>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct UbuntuCriteria {
  extend_definition: Option<Extend>,
  criteria:          Option<UbuntuCriteria2>,
  
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct Extend {
  #[serde(rename = "@definition_ref")]
  definition_ref:      Option<String>,
  #[serde(rename = "@comment")]
  comment:             Option<String>,
  #[serde(rename = "@applicability_check")]
  applicability_check: Option<String>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct UbuntuCriteria2 {
  #[serde(rename = "@operator")]
  operator:  Option<String>,
  criterion: Option<Vec<UbuntuCriterion>>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct UbuntuCriterion {
  #[serde(rename = "@comment")]
  comment:  Option<String>,
  #[serde(rename = "@test_ref")]
  test_ref: Option<String>
}


pub async fn main(mongo_client: MongoClient) -> Result<()> {
  let db: mongodb::Database = mongo_client.database("oval-ubuntu");

  for collection_name in db.list_collection_names(None).await? {
    println!("list Collection: {}", collection_name);
  }

  let code_name: [&str; 7] = [
    "trusty",  // Ubuntu 14.04 LTS  2019-04  2024-04
    "xenial",  // Ubuntu 16.04 LTS  2021-04  2026-04
    "bionic",  // Ubuntu 18.04 LTS  2023-06  2028-04
    "focal",   // Ubuntu 20.04 LTS  2025-04  2030-04
    "jammy",   // Ubuntu 22.04 LTS  2027-04  2032-04
    "kinetic", // Ubuntu 22.10      2023-07  2023-07
    "lunar",   // Ubuntu 23.04      2024-01  2024-01
  ];

  for v in code_name {
    let url: String = String::from("https://security-metadata.canonical.com/oval/com.ubuntu.") + v + ".pkg.oval.xml.bz2";

    let response = reqwest::get(&url).await?;
    let bytes = response.bytes().await?;

    let mut gz: BzDecoder<&[u8]> = BzDecoder::new(&bytes[..]);
    let mut resp_body: String = String::new();
    gz.read_to_string(&mut resp_body)?;

    let oval_ubuntu: OvalUbuntu = from_str(&resp_body)?;

    let col: String = String::from("ubuntu-") + v;
    let typed_collection: mongodb::Collection<UbuntuDefinition> = db.collection::<UbuntuDefinition>(&col);
    
    let filter: bson::Document = doc! {};
    let delete_result: mongodb::results::DeleteResult = typed_collection.delete_many(filter, None).await?;
    println!("Deleted {} documents, col:{}", delete_result.deleted_count, col);
    
    for i in 0..oval_ubuntu.definitions.definition.len() {
      let insert_result: mongodb::results::InsertOneResult = typed_collection.insert_one(oval_ubuntu.definitions.definition[i].clone(), None).await?;
      println!("document ID:{}, col:{}", insert_result.inserted_id, col);
    }
  }

  Ok(())
}