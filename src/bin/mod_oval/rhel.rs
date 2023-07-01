use anyhow::Result;
use mongodb::{Client as MongoClient, bson::doc};
use serde::{Deserialize, Serialize};
use std::io::Read;
use std::clone::Clone;
use bzip2::read::BzDecoder;
use quick_xml::de::from_str;


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


pub async fn main(mongo_client: MongoClient) -> Result<()> {
  let db: mongodb::Database = mongo_client.database("oval-rhel");

  for collection_name in db.list_collection_names(None).await? {
    println!("list Collection: {}", collection_name);
  }

  let rhel_ver: [u8; 4] = [
    6, // Red Hat Enterprise Linux 6  2016-05-10  2020-11-30  2024-06-30
    7, // Red Hat Enterprise Linux 7  2019-08-06  2024-06-30  2028-06-30
    8, // Red Hat Enterprise Linux 8  2024-05-31  2029-05-31  2032-05-31
    9  // Red Hat Enterprise Linux 9  2027-05-31  2032-05-31  2035-05-31
  ];

  for v in rhel_ver {
    let v: &str = &v.to_string();
    let url: String = String::from("https://access.redhat.com/security/data/oval/v2/RHEL") + v + "/rhel-" + v + ".oval.xml.bz2";

    let response = reqwest::get(&url).await?;
    let bytes = response.bytes().await?;

    let mut gz: BzDecoder<&[u8]> = BzDecoder::new(&bytes[..]);
    let mut resp_body: String = String::new();
    gz.read_to_string(&mut resp_body)?;

    let oval_rhel: OvalRhel = from_str(&resp_body)?;

    let col: String = String::from("rhel") + v;
    let typed_collection: mongodb::Collection<RhelDefinition> = db.collection::<RhelDefinition>(&col);
    
    let filter: bson::Document = doc! {};
    let delete_result: mongodb::results::DeleteResult = typed_collection.delete_many(filter, None).await?;
    println!("Deleted {} documents, col:{}", delete_result.deleted_count, col);
    
    for i in 0..oval_rhel.definitions.definition.len() {
      let insert_result: mongodb::results::InsertOneResult = typed_collection.insert_one(oval_rhel.definitions.definition[i].clone(), None).await?;
      println!("document ID:{}, col:{}", insert_result.inserted_id, col);
    }
  }

  Ok(())
}