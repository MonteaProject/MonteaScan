use time::{OffsetDateTime, macros::offset};
use hyper::Client;
use hyper_tls::HttpsConnector;
use serde::{Deserialize, Serialize};
use mongodb::{options::ClientOptions, Client as MongoClient, bson::doc};
use std::clone::Clone;
use serde_json::{Result};
use std::io::Read;
use flate2::read::GzDecoder;
use hyper_proxy::{Proxy, ProxyConnector, Intercept};
use headers::Authorization;

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
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();

    // 手動でオプションを設定する
    client_options.app_name = Some("My App".to_string());

    // デプロイメントのハンドルを取得する
    let mongo_client = MongoClient::with_options(client_options).unwrap();

    // データベースの名前をリストアップする
    for db_name in mongo_client.list_database_names(None, None).await.unwrap() {
        println!("list DB: {}", db_name);
    }

    // データベースのハンドルを取得する
    let db = mongo_client.database("Nvd");

    // データベースのコレクション名を列挙する
    for collection_name in db.list_collection_names(None).await.unwrap() {
        println!("list Collection: {}", collection_name);
    }

    // HTTPS Client //
    // if let Ok(http_proxy) = std::env::var("http_proxy") {
    //     let proxy = {
    //         let proxy_uri = http_proxy.parse().unwrap();
    //         let mut proxy = Proxy::new(Intercept::All, proxy_uri);
    //         proxy.set_authorization(Authorization::basic("John Doe", "Agent1234"));
    //         let connector = HttpsConnector::new();
    //         let proxy_connector = ProxyConnector::from_proxy(connector, proxy).unwrap();
    //         proxy_connector
    //     };
    //     let client = Client::builder().build::<_, hyper::Body>(proxy);
    // } else if let Ok(https_proxy) = std::env::var("https_proxy") {
    //     let proxy = {
    //         let proxy_uri = https_proxy.parse().unwrap();
    //         let mut proxy = Proxy::new(Intercept::All, proxy_uri);
    //         proxy.set_authorization(Authorization::basic("John Doe", "Agent1234"));
    //         let connector = HttpsConnector::new();
    //         let proxy_connector = ProxyConnector::from_proxy(connector, proxy).unwrap();
    //         proxy_connector
    //     };
    //     let client = Client::builder().build::<_, hyper::Body>(proxy);
    // } else {
    //     let https = HttpsConnector::new();
    //     let client = Client::builder().build::<_, hyper::Body>(https);
    // }

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);


    // 年別情報
    let utc = OffsetDateTime::now_utc();
    let jct = utc.to_offset(offset!(+9));
    let year = jct.year();

    let mut year_vec = vec![];
    for i in 2002..year+1 {
        year_vec.push(i);
    }

    for y in year_vec {
        let y: &str = &y.to_string();
        let url = String::from("https://nvd.nist.gov/feeds/json/cve/1.1/nvdcve-1.1-") + y + ".json.gz";
        let res = client.get(url.parse().unwrap()).await.unwrap();
        let resp = hyper::body::to_bytes(res.into_body()).await.unwrap();

        let mut gz = GzDecoder::new(&resp[..]);
        let mut resp_body = String::new();
        gz.read_to_string(&mut resp_body).unwrap();
        
        let nvd_year: Nvd = serde_json::from_str(&resp_body).unwrap();

        let col = String::from("Nvd") + y;
        let typed_collection = db.collection::<CVE_Items>(&col);
        
        let filter = doc! {};
        let delete_result = typed_collection.delete_many(filter, None).await.unwrap();
        println!("Deleted {} documents, col:{}", delete_result.deleted_count, col);
        
        for i in 0..nvd_year.CVE_Items.len() {
            let insert_result = typed_collection.insert_one(nvd_year.CVE_Items[i].clone(), None).await.unwrap();
            println!("document ID:{}, col:{}", insert_result.inserted_id, col);
        }
    }

    Ok(())
}
