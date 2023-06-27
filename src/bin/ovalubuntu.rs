use mongodb::{options::ClientOptions, Client as MongoClient, bson::doc};
use serde::{Deserialize, Serialize};
use serde_json::{Result};
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
  #[serde(rename = "@from")]
  from:              Option<String>,
  severity:          Option<String>,
//   rights:            Option<String>,
  issued:            Option<UbuntuIssued>,
//   updated:           Option<RhelUpdated>,
  cve:               Option<Vec<UbuntuCve>>,
//   bugzilla:          Option<Vec<RhelBugzilla>>,
//   affected_cpe_list: Option<RhelAffectedCpeList>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct UbuntuIssued {
  #[serde(rename = "@date")]
  date: Option<String>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct RhelUpdated {
  #[serde(rename = "@date")]
  date: Option<String>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct UbuntuCve {
//   #[serde(rename = "@cvss2")]
//   cvss2:  Option<String>,
//   #[serde(rename = "@cvss3")]
//   cvss3:  Option<String>,
  #[serde(rename = "@cvss_score")]
  cvss_score:  Option<String>,
  #[serde(rename = "@cvss_vector")]
  cvss_vector:  Option<String>,
//   #[serde(rename = "@cwe")]
//   cwe:    Option<String>,
  #[serde(rename = "@href")]
  href:   Option<String>,
//   #[serde(rename = "@impact")]
//   impact: Option<String>,
  #[serde(rename = "@severity")]
  severity: Option<String>,
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
struct UbuntuCriteria {
  #[serde(rename = "@operator")]
  operator:  Option<String>,
//   criterion: Option<Vec<RhelCriterion>>,
  criteria:  Option<Vec<UbuntuCriteria2>>
}

// #[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
// struct RhelCriterion {
//   #[serde(rename = "@comment")]
//   comment:  Option<String>,
//   #[serde(rename = "@test_ref")]
//   test_ref: Option<String>
// }

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct UbuntuCriteria2 {
  #[serde(rename = "@operator")]
  operator:  Option<String>,
  criterion: Option<Vec<UbuntuCriterion2>>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct UbuntuCriterion2 {
  #[serde(rename = "@comment")]
  comment:  Option<String>,
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

  let db: mongodb::Database = mongo_client.database("OvalRHEL");

  for collection_name in db.list_collection_names(None).await.unwrap() {
    println!("list Collection: {}", collection_name);
  }

  let mut rhel_ver: Vec<i32> = vec![];
  for i in 6..10 {
    rhel_ver.push(i);
  }

  for v in rhel_ver {
    let v: &str = &v.to_string();
    let url: String = String::from("https://access.redhat.com/security/data/oval/v2/RHEL") + v + "/rhel-" + v + ".oval.xml.bz2";

    let response = reqwest::get(&url).await.unwrap();
    let bytes = response.bytes().await.unwrap();

    let mut gz: BzDecoder<&[u8]> = BzDecoder::new(&bytes[..]);
    let mut resp_body: String = String::new();
    gz.read_to_string(&mut resp_body).unwrap();

    let oval_rhel: OvalRhel = from_str(&resp_body).unwrap();

    let col: String = String::from("RHEL") + v;
    let typed_collection: mongodb::Collection<RhelDefinition> = db.collection::<RhelDefinition>(&col);
    
    let filter: bson::Document = doc! {};
    let delete_result: mongodb::results::DeleteResult = typed_collection.delete_many(filter, None).await.unwrap();
    println!("Deleted {} documents, col:{}", delete_result.deleted_count, col);
    
    for i in 0..oval_rhel.definitions.definition.len() {
      let insert_result: mongodb::results::InsertOneResult = typed_collection.insert_one(oval_rhel.definitions.definition[i].clone(), None).await.unwrap();
      println!("document ID:{}, col:{}", insert_result.inserted_id, col);
    }
  }

  Ok(())
}