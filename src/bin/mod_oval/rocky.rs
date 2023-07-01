use anyhow::Result;
use mongodb::{Client as MongoClient, bson::doc};
use serde::{Deserialize, Serialize};
use std::io::Read;
use std::clone::Clone;
use bzip2::read::BzDecoder;
use quick_xml::de::from_str;


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
  from:     Option<String>,
  severity: Option<String>,
  rights:   Option<String>,
  issued:   Option<RockyIssued>,
  updated:  Option<RockyUpdated>,
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


pub async fn main(mongo_client: MongoClient) -> Result<()> {
  let db: mongodb::Database = mongo_client.database("oval-rocky");

  for collection_name in db.list_collection_names(None).await? {
    println!("list Collection: {}", collection_name);
  }

  let rocky_ver: [u8; 2] = [
    8, // Rocky Linux 8  2024-05-31  2029-05-31
    9  // Rocky Linux 9  2027-05-31  2032-05-31
  ];

  for v in rocky_ver {
    let v: &str = &v.to_string();
    let url: String = String::from("https://dl.rockylinux.org/pub/oval/org.rockylinux.rlsa-") + v + ".xml.bz2";

    let response = reqwest::get(&url).await?;
    let bytes = response.bytes().await?;

    let mut gz: BzDecoder<&[u8]> = BzDecoder::new(&bytes[..]);
    let mut resp_body: String = String::new();
    gz.read_to_string(&mut resp_body)?;

    let oval_rocky: OvalRocky = from_str(&resp_body)?;

    let col: String = String::from("Rocky") + v;
    let typed_collection: mongodb::Collection<RockyDefinition> = db.collection::<RockyDefinition>(&col);
    
    let filter: bson::Document = doc! {};
    let delete_result: mongodb::results::DeleteResult = typed_collection.delete_many(filter, None).await?;
    println!("Deleted {} documents, col:{}", delete_result.deleted_count, col);
    
    for i in 0..oval_rocky.definitions.definition.len() {
      let insert_result: mongodb::results::InsertOneResult = typed_collection.insert_one(oval_rocky.definitions.definition[i].clone(), None).await?;
      println!("document ID:{}, col:{}", insert_result.inserted_id, col);
    }
  }

  Ok(())
}