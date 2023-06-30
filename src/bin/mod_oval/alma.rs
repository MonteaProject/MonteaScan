use anyhow::{Result, Context, anyhow};
use mongodb::{Client as MongoClient, bson::doc};
use serde::{Deserialize, Serialize};
// use serde_json::{Result};
use std::io::Read;
use std::clone::Clone;
use bzip2::read::BzDecoder;
use quick_xml::de::from_str;


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
  // affected: Option<Affected>, // None,AlmaLinux
  reference:   Option<Vec<AlmaReference>>,
  description: Option<String>,
  advisory:    Option<AlmaAdvisory>,
}

// #[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
// struct Affected {
//   #[serde(rename = "@family")]
//   family:   Option<String>,
//   platform: Option<Vec<String>>
// }

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
  // #[serde(rename = "@cvss2")] // None,AlmaLinux
  // cvss2: Option<String>,
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
  // #[serde(rename = "$value")] // None,AlmaLinux
  // bugzilla: Option<String>
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


// #[tokio::main(flavor = "current_thread")]
pub async fn main(mongo_client: MongoClient) -> Result<()> {
  // let mut client_options: ClientOptions = ClientOptions::parse("mongodb://localhost:27017").await?;
  // client_options.app_name = Some("My App".to_string());

  // let mongo_client: MongoClient = MongoClient::with_options(client_options)?;

  // for db_name in mongo_client.list_database_names(None, None).await? {
  //   println!("list DB: {}", db_name);
  // }

  let db: mongodb::Database = mongo_client.database("OvalAlma");

  for collection_name in db.list_collection_names(None).await? {
    println!("list Collection: {}", collection_name);
  }

  let alma_ver: [u8; 2] = [
    8, // AlmaLinux 8  2024-05-01  2029-03-01
    9  // AlmaLinux 9  2027-05-31  2032-05-31
  ];

  for v in alma_ver {
    let v: &str = &v.to_string();
    let url: String = String::from("https://repo.almalinux.org/security/oval/org.almalinux.alsa-") + v + ".xml.bz2";

    let response = reqwest::get(&url).await?;
    let bytes = response.bytes().await?;

    let mut gz: BzDecoder<&[u8]> = BzDecoder::new(&bytes[..]);
    let mut resp_body: String = String::new();
    gz.read_to_string(&mut resp_body)?;

    let oval_alma: OvalAlma = from_str(&resp_body)?;

    let col: String = String::from("Alma") + v;
    let typed_collection: mongodb::Collection<AlmaDefinition> = db.collection::<AlmaDefinition>(&col);
    
    let filter: bson::Document = doc! {};
    let delete_result: mongodb::results::DeleteResult = typed_collection.delete_many(filter, None).await?;
    println!("Deleted {} documents, col:{}", delete_result.deleted_count, col);
    
    for i in 0..oval_alma.definitions.definition.len() {
      let insert_result: mongodb::results::InsertOneResult = typed_collection.insert_one(oval_alma.definitions.definition[i].clone(), None).await?;
      println!("document ID:{}, col:{}", insert_result.inserted_id, col);
    }
  }

  Ok(())
}