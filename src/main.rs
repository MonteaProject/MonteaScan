use time::{OffsetDateTime, macros::offset};
use hyper::Client;
use hyper_tls::HttpsConnector;
use serde::{Deserialize, Serialize};
use mongodb::{options::ClientOptions, Client as MongoClient, bson::doc};
use std::clone::Clone;
use serde_json::{Result};
use std::io::Read;
use bzip2::read::BzDecoder;
use hyper_proxy::{Proxy, ProxyConnector, Intercept};
use headers::Authorization;

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
        let url = String::from("https://www.redhat.com/security/data/oval/com.redhat.rhsa-RHEL") + v + ".xml.bz2";
        let res = client.get(url.parse().unwrap()).await.unwrap();
        let resp = hyper::body::to_bytes(res.into_body()).await.unwrap();

        let mut gz = BzDecoder::new(&resp[..]);
        let mut resp_body = String::new();
        gz.read_to_string(&mut resp_body).unwrap();

        println!("{:?}", resp_body);
        
        // let oval_rhel: Nvd = serde_json::from_str(&resp_body).unwrap();

        // let col = String::from("OvalRHEL") + v;
        // let typed_collection = db.collection::<CVE_Items>(&col);
        
        // let filter = doc! {};
        // let delete_result = typed_collection.delete_many(filter, None).await.unwrap();
        // println!("Deleted {} documents, col:{}", delete_result.deleted_count, col);
        
        // for i in 0..oval_rhel.CVE_Items.len() {
        //     let insert_result = typed_collection.insert_one(oval_rhel.CVE_Items[i].clone(), None).await.unwrap();
        //     println!("document ID:{}, col:{}", insert_result.inserted_id, col);
        // }
    }

    Ok(())
}
