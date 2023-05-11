// mod server;

// fn main() {
//     server::main();
// }

use hyper::Client;
use hyper_tls::HttpsConnector;
use quick_xml::de::from_str;
use serde::{Deserialize, Serialize};
use mongodb::{options::ClientOptions, Client as MongoClient, bson::doc};
use std::clone::Clone;
use serde_json::{Result};
use std::io::Read;
use bzip2::read::BzDecoder;

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
    metadata: Metadata,
    criteria: Option<Criteria>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct Metadata {
    title: Option<String>,
    affected: Option<Vec<Affected>>,
    reference: Option<Vec<Reference>>,
    description: String,
    advisory: Advisory,
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
    cves: Option<Cves>,
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
struct Cves {
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

//

use time::{OffsetDateTime, macros::offset, format_description};

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct Root {
    family: String,
    osversion: String,
    definitions: Vec<Definitions>,
    timestamp: String
}


//

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    // timestamp
    let utc = OffsetDateTime::now_utc();
    let jct = utc.to_offset(offset!(+9));
    let format = format_description::parse(
        "[year]-[month]-[day] [hour]:[minute]:[second]"
    ).unwrap();
    let time = jct.format(&format).unwrap();

    // https client
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    for v in 6..10 {
        let v: &str = &v.to_string();
        let url = String::from("https://access.redhat.com/security/data/oval/v2/RHEL") + v + "/rhel-" + v + ".oval.xml.bz2";
        let res = client.get(url.parse().unwrap()).await.unwrap();
        let resp = hyper::body::to_bytes(res.into_body()).await.unwrap();

        let mut gz = BzDecoder::new(&resp[..]);
        let mut resp_body = String::new();
        gz.read_to_string(&mut resp_body).unwrap();

        let oval_rhel: OvalRhel = from_str(&resp_body).unwrap();

        for i in 0..oval_rhel.definitions.definition.len() {
            let description = &oval_rhel.definitions.definition[i].metadata.description;
            if description.find("** REJECT **").is_some() {
                continue;
            }

        //     let a = &oval_rhel.definitions.definition[i].metadata.advisory.cves;

        //     let cvss2 = &oval_rhel.definitions.definition[i].metadata.advisory.cves.cvss2;
        //     println!("{:?}", cvss2);
        //     let cvss3 = &oval_rhel.definitions.definition[i].metadata.advisory.cves.cvss3;
        //     println!("{:?}", cvss3);
        //     let cwe = &oval_rhel.definitions.definition[i].metadata.advisory.cves.cwe;
        //     println!("{:?}", cwe);
        //     let href = &oval_rhel.definitions.definition[i].metadata.advisory.cves.href;
        //     println!("{:?}", href);
        //     let impact = &oval_rhel.definitions.definition[i].metadata.advisory.cves.impact;
        //     println!("{:?}", impact);
        //     let public = &oval_rhel.definitions.definition[i].metadata.advisory.cves.public;
        //     println!("{:?}", public);
        //     let cve = &oval_rhel.definitions.definition[i].metadata.advisory.cves.cve;
        //     println!("{:?}", cve);
        }




                // let cves = vec![];
                // for c in d.Advisory.Cves {
                //     cves.push(
                //         c.CveID,
                //         c.Cvss2,
                //         c.cvss3,
                //         c.cwe,
                //         c.impact,
                //         c.href,
                //         c.public
                //     )
                // }

                // let ref = vec![];
                // for r in d.References {
                //     ref.push(
                //         r.Source,
                //         r.RefID,
                //         r.RefURL
                //     )
                // }

                // let cpe = vec![];
                // for p in d.Advisory.AffectedCPEList {
                //     cpe.push(p)
                // }

                // let bs = vec![];
                // for b in d.Advisory.Bugzillas {
                //     bs.push(
                //         b.ID,
                //         b.URL,
                //         b.Title
                //     )
                // }

                // let issued = d.Advisory.Issued.Date;
                // let updated = d.Advisory.Updated.Date;

                // let def = OVALREHL {
                //     DefinitionID: d.ID,
                //     Title: d.Title,
                //     Description: d.Description,
                //     Advisory: Advisory {
                //         Severity: d.Advisory.Severity,
                //         Cves: cves,
                //         Bugzilla: bs,
                //         AffectedCPEList: cpe,
                //         Issued: issued,
                //         Updated: updated
                //     },
                //     Debian: nil,
                //     AffectedPacks: pkg,
                //     References: rs
            //     };
            // }

        // let root = Root {
        //     family: String::from("RHEL"),
        //     osversion: String::from(v),
        //     definitions: def,
        //     timestamp: time.clone()
        // };

        // println!("{:?}", root);
    }

    

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

    // let https = HttpsConnector::new();
    // let client = Client::builder().build::<_, hyper::Body>(https);

    // let mut rhel_ver = vec![];
    // for i in 6..10 {
    //     rhel_ver.push(i);
    // }

    // for v in rhel_ver {
    //     let v: &str = &v.to_string();
    //     let url = String::from("https://access.redhat.com/security/data/oval/v2/RHEL") + v + "/rhel-" + v + ".oval.xml.bz2";
    //     let res = client.get(url.parse().unwrap()).await.unwrap();
    //     let resp = hyper::body::to_bytes(res.into_body()).await.unwrap();

    //     let mut gz = BzDecoder::new(&resp[..]);
    //     let mut resp_body = String::new();
    //     gz.read_to_string(&mut resp_body).unwrap();

    //     let oval_rhel: OvalRhel = from_str(&resp_body).unwrap();

    //     let col = String::from("RHEL") + v;
    //     let typed_collection = db.collection::<Definition>(&col);
        
    //     let filter = doc! {};
    //     let delete_result = typed_collection.delete_many(filter, None).await.unwrap();
    //     println!("Deleted {} documents, col:{}", delete_result.deleted_count, col);
        
    //     for i in 0..oval_rhel.definitions.definition.len() {
    //         let insert_result = typed_collection.insert_one(oval_rhel.definitions.definition[i].clone(), None).await.unwrap();
    //         println!("document ID:{}, col:{}", insert_result.inserted_id, col);
    //     }
    // }

    Ok(())
}
