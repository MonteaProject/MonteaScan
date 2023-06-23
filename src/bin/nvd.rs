use time::{OffsetDateTime, macros::offset};
use mongodb::{options::ClientOptions, Client as MongoClient, bson::doc};
use serde::{Deserialize, Serialize};
use serde_json::{Result};
use std::clone::Clone;
use std::io::Read;
use flate2::read::GzDecoder;


#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
#[allow(dead_code)]
#[allow(non_snake_case)]
struct Nvd {
  CVE_data_type: Option<String>,
  CVE_data_format: Option<String>,
  CVE_data_version: Option<String>,
  CVE_data_numberOfCVEs: Option<String>,
  CVE_data_timestamp: Option<String>,
  CVE_Items: Vec<CVE_Items>,
}

// Nvd
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
struct CVE_Items {
  cve: Option<Cve>,
  configurations: Option<Configurations>,
  impact: Option<Impact>,
  publishedDate: Option<String>,
  lastModifiedDate: Option<String>,
}

// CVE_Items
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
#[allow(dead_code)]
#[allow(non_snake_case)]
struct Cve {
  data_type: Option<String>,
  data_format: Option<String>,
  data_version: Option<String>,
  CVE_data_meta: Option<CVE_data_meta>,
  problemtype: Option<problemtype>,
  references: Option<references>,
  #[serde(rename = "@description")]
  description2: Option<Vec<description2>>,
}

// Cve
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
struct CVE_data_meta {
  ID: Option<String>,
  ASSIGNER: Option<String>,
}

// Cve
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
struct problemtype {
  problemtype_data: Option<Vec<problemtype_data>>,
}

// Cve
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
struct problemtype_data {
  description: Option<Vec<description>>,
}

// Cve
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
struct description {
  lang: Option<String>,
  value: Option<String>,
}

// Cve
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
struct references {
  reference_data: Option<Vec<reference_data>>,
}

// Cve
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
struct reference_data {
  url: Option<String>,
  name: Option<String>,
  refsource: Option<String>,
  tags: Option<Vec<String>>,
}

// Cve
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
struct description2 {
  description_data: Option<Vec<description_data>>,
}

// Cve
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
struct description_data {
  lang: Option<String>,
  value: Option<String>,
}

// CVE_Items
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
struct Configurations {
  CVE_data_version: Option<String>,
  nodes: Option<Vec<nodes>>,
}

// Configurations
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
struct nodes {
  operator: Option<String>,
  children: Option<Vec<children>>,
  negate: Option<bool>,
  cpe_match: Option<Vec<cpe_match>>,
}

// Configurations
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
struct cpe_match {
  vulnerable: Option<bool>,
  cpe23Uri: Option<String>,
  versionStartExcluding: Option<String>,
  versionStartIncluding: Option<String>,
  versionEndExcluding: Option<String>,
  versionEndIncluding: Option<String>,
}

// Configurations
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
struct children {
  operator: Option<String>,
  #[serde(rename = "@cpe_match")]
  cpe_match2: Option<Vec<cpe_match2>>,
}

// Configurations
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
struct cpe_match2 {
  vulnerable: Option<bool>,
  cpe23Uri: Option<String>,
  versionStartExcluding: Option<String>,
  versionStartIncluding: Option<String>,
  versionEndExcluding: Option<String>,
  versionEndIncluding: Option<String>,
}

// CVE_Items
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
struct Impact {
  baseMetricV3: Option<baseMetricV3>,
  baseMetricV2: Option<baseMetricV2>,
}

// Impact
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
struct baseMetricV3 {
  cvssV3: Option<cvssV3>,
  exploitabilityScore: Option<f64>,
  impactScore: Option<f64>,
}

// baseMetricV3
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
struct cvssV3 {
  version: Option<String>,
  vectorString: Option<String>,
  attackVector: Option<String>,
  attackComplexity: Option<String>,
  privilegesRequired: Option<String>,
  userInteraction: Option<String>,
  scope: Option<String>,
  confidentialityImpact: Option<String>,
  integrityImpact: Option<String>,
  availabilityImpact: Option<String>,
  baseScore: Option<f64>,
  baseSeverity: Option<String>,
}

// Impact
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
struct baseMetricV2 {
  cvssV2: Option<cvssV2>,
  severity: Option<String>,
  exploitabilityScore: Option<f64>,
  impactScore: Option<f64>,
  obtainAllPrivilege: Option<bool>,
  obtainUserPrivilege: Option<bool>,
  obtainOtherPrivilege: Option<bool>,
  userInteractionRequired: Option<bool>,
}

// baseMetricV2
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
struct cvssV2 {
  version: Option<String>,
  vectorString: Option<String>,
  accessVector: Option<String>,
  accessComplexity: Option<String>,
  authentication: Option<String>,
  confidentialityImpact: Option<String>,
  integrityImpact: Option<String>,
  availabilityImpact: Option<String>,
  baseScore: Option<f64>,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
  // MongoDB //
  // データベース名 = Jvn
  // テーブル = コレクション = JvnXX
  // レコード = ドキュメント

  // 接続文字列を解析して、options構造体に変換する
  let mut client_options: ClientOptions = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();

  // 手動でオプションを設定する
  client_options.app_name = Some("My App".to_string());

  // デプロイメントのハンドルを取得する
  let mongo_client: MongoClient = MongoClient::with_options(client_options).unwrap();

  // データベースの名前をリストアップする
  for db_name in mongo_client.list_database_names(None, None).await.unwrap() {
    println!("list DB: {}", db_name);
  }

  // データベースのハンドルを取得する
  let db: mongodb::Database = mongo_client.database("Nvd");

  // データベースのコレクション名を列挙する
  for collection_name in db.list_collection_names(None).await.unwrap() {
    println!("list Collection: {}", collection_name);
  }

  // 年別情報
  let utc: OffsetDateTime = OffsetDateTime::now_utc();
  let jct: OffsetDateTime = utc.to_offset(offset!(+9));
  let year: i32 = jct.year();

  let mut year_vec: Vec<i32> = vec![];
  for i in 2002..year+1 {
    year_vec.push(i);
  }

  for y in year_vec {
    let y: &str = &y.to_string();
    let url: String = String::from("https://nvd.nist.gov/feeds/json/cve/1.1/nvdcve-1.1-") + y + ".json.gz";

    let response = reqwest::get(&url).await.unwrap();
    let bytes = response.bytes().await.unwrap();

    let mut gz: GzDecoder<&[u8]> = GzDecoder::new(&bytes[..]);
    let mut resp_body: String = String::new();
    gz.read_to_string(&mut resp_body).unwrap();
    
    let nvd_year: Nvd = serde_json::from_str(&resp_body).unwrap();

    let col: String = String::from("Nvd") + y;
    let typed_collection: mongodb::Collection<CVE_Items> = db.collection::<CVE_Items>(&col);
    
    let filter: bson::Document = doc! {};
    let delete_result: mongodb::results::DeleteResult = typed_collection.delete_many(filter, None).await.unwrap();
    println!("Deleted {} documents, col:{}", delete_result.deleted_count, col);
    
    for i in 0..nvd_year.CVE_Items.len() {
      let insert_result: mongodb::results::InsertOneResult = typed_collection.insert_one(nvd_year.CVE_Items[i].clone(), None).await.unwrap();
      println!("document ID:{}, col:{}", insert_result.inserted_id, col);
    }
  }

  Ok(())
}