use mongodb::{options::ClientOptions, Client as MongoClient, bson::doc};
use serde::{Deserialize, Serialize};
use serde_json::{Result};
use std::io::Read;
use std::clone::Clone;
use bzip2::read::BzDecoder;
use quick_xml::de::from_str;


#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct OvalAlma {
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
    // affected: Option<Affected>, // None,AlmaLinux
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
    cve: Option<Vec<Cve>>,
    bugzilla: Option<Vec<Bugzilla>>,
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
struct Cve {
    // #[serde(rename = "@cvss2")] // None,AlmaLinux
    // cvss2: Option<String>,
    #[serde(rename = "@cvss3")]
    cvss3: Option<String>,
    #[serde(rename = "@cwe")]
    cwe: Option<String>,
    #[serde(rename = "@href")]
    href: Option<String>,
    #[serde(rename = "@impact")]
    impact: Option<String>,
    #[serde(rename = "@public")]
    public: Option<String>,
    #[serde(rename = "$value")]
    cve: Option<String>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct Bugzilla {
    #[serde(rename = "@href")]
    href: Option<String>,
    #[serde(rename = "@id")]
    id: Option<String>,
    // #[serde(rename = "$value")] // None,AlmaLinux
    // bugzilla: Option<String>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct AffectedCpeList {
    cpe: Option<Vec<String>>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]  // AlmaLinux 特有
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
    criterion: Option<Vec<Criterion2>>,
    criteria: Option<Vec<Criteria3>>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct Criterion2 {
    #[serde(rename = "@comment")]
    comment: Option<String>,
    #[serde(rename = "@test_ref")]
    test_ref: Option<String>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct Criteria3 {
    #[serde(rename = "@operator")]
    operator: Option<String>,
    criteria: Option<Vec<Criteria4>>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct Criteria4 {
    #[serde(rename = "@operator")]
    operator: Option<String>,
    criterion: Option<Vec<Criterion3>>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct Criterion3 {
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

    let db: mongodb::Database = mongo_client.database("OvalAlma");

    for collection_name in db.list_collection_names(None).await.unwrap() {
        println!("list Collection: {}", collection_name);
    }

    let mut alma_ver: Vec<i32> = vec![];
    for i in 8..10 {
        alma_ver.push(i);
    }

    for v in alma_ver {
        let v: &str = &v.to_string();
        let url: String = String::from("https://repo.almalinux.org/security/oval/org.almalinux.alsa-") + v + ".xml.bz2";

        let response = reqwest::get(&url).await.unwrap();
        let bytes = response.bytes().await.unwrap();

        let mut gz: BzDecoder<&[u8]> = BzDecoder::new(&bytes[..]);
        let mut resp_body: String = String::new();
        gz.read_to_string(&mut resp_body).unwrap();

        let oval_alma: OvalAlma = from_str(&resp_body).unwrap();

        let col: String = String::from("Alma") + v;
        let typed_collection: mongodb::Collection<Definition> = db.collection::<Definition>(&col);
        
        let filter: bson::Document = doc! {};
        let delete_result: mongodb::results::DeleteResult = typed_collection.delete_many(filter, None).await.unwrap();
        println!("Deleted {} documents, col:{}", delete_result.deleted_count, col);
        
        for i in 0..oval_alma.definitions.definition.len() {
            let insert_result: mongodb::results::InsertOneResult = typed_collection.insert_one(oval_alma.definitions.definition[i].clone(), None).await.unwrap();
            println!("document ID:{}, col:{}", insert_result.inserted_id, col);
        }
    }

    Ok(())
}
