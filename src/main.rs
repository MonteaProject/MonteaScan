use hyper::Client;
use hyper_tls::HttpsConnector;
use quick_xml::de::from_str;
use serde::{Deserialize, Serialize};
use mongodb::{options::ClientOptions, Client as MongoClient, bson::doc};
use std::{clone::Clone};
use serde_json::{Result};
use std::io::Read;
use bzip2::read::BzDecoder;

use quick_xml::events::Event;
use quick_xml::reader::Reader;
use time::{OffsetDateTime, macros::offset, format_description};
use std::collections::HashMap;


#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct OvalRhel {
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
    affected: Option<Vec<Affected>>,
    reference: Option<Vec<Reference>>,
    description: Option<String>,
    advisory: Option<Vec<Advisory>>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct Affected {
    #[serde(rename = "@family")]
    family: Option<String>,
    platform: Option<Vec<Platform>>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct Platform {
    platform: Option<String>
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
    affected_cpe_list: Option<Vec<AffectedCpeList>>,
    affected: Option<String>
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
    #[serde(rename = "@cvss2")]
    cvss2: Option<String>,
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
    #[serde(rename = "$value")]
    bugzilla: Option<String>
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
    #[serde(rename = "@criteria")]
    criteria2: Option<Vec<Criteria2>>
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
    #[serde(rename = "@criterion")]
    criterion2: Option<Vec<Criterion2>>
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
    // let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
    // client_options.app_name = Some("My App".to_string());

    // let mongo_client = MongoClient::with_options(client_options).unwrap();

    // for db_name in mongo_client.list_database_names(None, None).await.unwrap() {
    //     println!("list DB: {}", db_name);
    // }

    // let db = mongo_client.database("OvalRHEL");

    // for collection_name in db.list_collection_names(None).await.unwrap() {
    //     println!("list Collection: {}", collection_name);
    // }

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let mut rhel_ver = vec![];
    for i in 6..10 {
        rhel_ver.push(i);
    }

    for v in rhel_ver {
        let v: &str = &v.to_string();
        let url = String::from("https://access.redhat.com/security/data/oval/v2/RHEL") + v + "/rhel-" + v + ".oval.xml.bz2";
        let res = client.get(url.parse().unwrap()).await.unwrap();
        let resp = hyper::body::to_bytes(res.into_body()).await.unwrap();

        let mut gz = BzDecoder::new(&resp[..]);
        let mut resp_body = String::new();
        gz.read_to_string(&mut resp_body).unwrap();

//
        // (example)
        //  <hoge>foo</hoge>
        //  <hoge>  ... Event::Start
        //  foo     ... Event::Text
        //  </hoge> ... Event::End

        #[derive(Debug, Clone)]
        struct TableStat {
            index: u32,
            description: Vec<String>,
            cve: Vec<Vec<String>>
        }

        let mut reader = Reader::from_str(&resp_body);
        let mut buf = Vec::new();
        let mut count = 0;
        let mut skip_buf = Vec::new();
        let mut found_tables = Vec::new();
        let mut custom_entities: HashMap<String, String> = HashMap::new();
        
        loop {
            match reader.read_event_into(&mut buf).unwrap() {
                Event::Start(e) => {
                    if let b"definition" = e.name().as_ref() {
                        count += 1;
                        let mut stats = TableStat {
                            index: count,
                            description: vec![],
                            cve: vec![]
                        };

                        loop {
                            skip_buf.clear();
                            match reader.read_event_into(&mut skip_buf).unwrap() {
                                Event::Start(e) => match e.name().as_ref() {
                                    b"description" => {
                                        let txt = reader
                                            .read_text(e.name())
                                            .expect("Cannot decode text value");

                                        stats.description.push(
                                            txt.to_string()
                                        );
                                    }
                                    b"cve" => {
                                        let attr = e
                                            .attributes()
                                            .map(|a| {
                                                a.unwrap()
                                                    .decode_and_unescape_value_with(&reader, |ent| {
                                                        custom_entities.get(ent).map(|s| s.as_str())
                                                    })
                                                    .unwrap()
                                                    .into_owned()
                                            })
                                            .collect::<Vec<_>>();

                                        stats.cve.push(
                                            attr
                                        );
                                    }
                                    _ => {}
                                },
                                Event::End(e) => {
                                    if e.name().as_ref() == b"definition" {
                                        found_tables.push(stats);
                                        break;
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
                Event::Eof => break,
                _ => {}
            }
            buf.clear();
        }

        println!("{:?}", found_tables);
//

        let family = String::from("redhat");
        let osver = String::from(v);

        let utc = OffsetDateTime::now_utc();
        let jct = utc.to_offset(offset!(+9));
        let format = format_description::parse(
            "[year]-[month]-[day] [hour]:[minute]:[second]"
        ).unwrap();

        let timestamp = jct.format(&format).unwrap();

        // println!("{:?},{:?},{:?}", family, osver, timestamp);



        // let oval_rhel: OvalRhel = from_str(&resp_body).unwrap();
        // let a = oval_rhel.definitions.definition.iter();

        // for i in a {
        //     println!("{:?}", i);
        // }

        // let col = String::from("RHEL") + v;
        // let typed_collection = db.collection::<Definition>(&col);
        
        // let filter = doc! {};
        // let delete_result = typed_collection.delete_many(filter, None).await.unwrap();
        // println!("Deleted {} documents, col:{}", delete_result.deleted_count, col);
        
        // for i in 0..oval_rhel.definitions.definition.len() {
        //     let insert_result = typed_collection.insert_one(oval_rhel.definitions.definition[i].clone(), None).await.unwrap();
        //     println!("document ID:{}, col:{}", insert_result.inserted_id, col);
        // }
    }

    Ok(())
}
