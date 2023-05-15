use time::{OffsetDateTime, macros::offset};
use hyper::{Client, Uri};
use hyper_tls::HttpsConnector;
use quick_xml::de::from_str;
use serde::{Deserialize, Serialize};
use mongodb::{options::ClientOptions, Client as MongoClient, bson::doc};
use std::clone::Clone;
use serde_json::{Result};

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct Rdf {
    item: Vec<Items>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct Items {
    #[serde(rename = "@about")]
    about: String,
	title: String,
	link: String,
	description: String,
	identifier: String,
	references: Option<Vec<References>>,
	cpe: Option<Vec<Cpe>>,
	cvss: Option<Vec<Cvss>>,
	date: String,
	issued: String,
	modified: String,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct References {
    #[serde(rename = "@source")]
    source: Option<String>,
    #[serde(rename = "@id")]
    id: Option<String>,
    #[serde(rename = "@title")]
    title: Option<String>,
    #[serde(rename = "$value")]
    references: Option<String>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct Cpe {
    #[serde(rename = "@version")]
    version: Option<String>,
    #[serde(rename = "@vendor")]
    vendor: Option<String>,
    #[serde(rename = "@product")]
    product: Option<String>,
    #[serde(rename = "$value")]
    cpe: Option<String>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct Cvss {
    #[serde(rename = "@version")]
    version: Option<String>,
    #[serde(rename = "@score")]
    score: Option<String>,
    #[serde(rename = "@type")]
    ty: Option<String>,
    #[serde(rename = "@severity")]
    severity: Option<String>,
    #[serde(rename = "@vector")]
    vector: Option<String>,
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
    let db = mongo_client.database("Jvn");

    // データベースのコレクション名を列挙する
    for collection_name in db.list_collection_names(None).await.unwrap() {
        println!("list Collection: {}", collection_name);
    }


    // HTTPS Client //
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    // 新着情報
    let new = async {
        let resp = client.get(Uri::from_static("https://jvndb.jvn.jp/ja/rss/jvndb_new.rdf")).await?;
        hyper::body::to_bytes(resp.into_body()).await
    };
    // 新着/更新情報
    let update = async {
        let resp = client.get(Uri::from_static("https://jvndb.jvn.jp/ja/rss/jvndb.rdf")).await?;
        hyper::body::to_bytes(resp.into_body()).await
    };

    let (newjvn, updatejvn) = futures::try_join!(new, update).unwrap();

    let newjvn_body = String::from_utf8(newjvn.to_vec()).expect("response was not valid utf-8");
    let jvn_new: Rdf = from_str(&newjvn_body).unwrap();

    let updatejvn_body = String::from_utf8(updatejvn.to_vec()).expect("response was not valid utf-8");
    let jvn_update: Rdf = from_str(&updatejvn_body).unwrap();


    // データベース内のコレクションへのハンドルを取得する
    let typed_collection = db.collection::<Items>("JvnNew");

    // doc!{} ...全件取得
    let filter = doc! {};

    // .delete_many() ...削除
    let delete_result = typed_collection.delete_many(filter, None).await.unwrap();
    println!("Deleted {} documents, col:JvnNew", delete_result.deleted_count);

    // ドキュメント挿入
    for i in 0..jvn_new.item.len() {
        let insert_result = typed_collection.insert_one(jvn_new.item[i].clone(), None).await.unwrap();
        println!("document ID:{}, col:JvnNew", insert_result.inserted_id);
    }

    // 以下、同上
    let typed_collection = db.collection::<Items>("JvnUpdate");
    
    let filter = doc! {};
    let delete_result = typed_collection.delete_many(filter, None).await.unwrap();
    println!("Deleted {} documents, col:JvnUpdate", delete_result.deleted_count);

    for i in 0..jvn_update.item.len() {
        let insert_result = typed_collection.insert_one(jvn_update.item[i].clone(), None).await.unwrap();
        println!("document ID:{}, col:JvnUpdate", insert_result.inserted_id);
    }

    // 年別情報
    let utc = OffsetDateTime::now_utc();
    let jct = utc.to_offset(offset!(+9));
    let year = jct.year();

    let mut year_vec = vec![];
    for i in 1998..year+1 {
        year_vec.push(i);
    }

    for y in year_vec {
        let y: &str = &y.to_string();
        let url = String::from("https://jvndb.jvn.jp/ja/rss/years/jvndb_") + y + ".rdf";
        let res = client.get(url.parse().unwrap()).await.unwrap();
        let resp = hyper::body::to_bytes(res.into_body()).await.unwrap();
        let resp_body = String::from_utf8(resp.to_vec()).expect("response was not valid utf-8");
        let jvn_year: Rdf = from_str(&resp_body).unwrap();

        // 以下、同上
        let col = String::from("Jvn") + y;

        let typed_collection = db.collection::<Items>(&col);
        
        let filter = doc! {};
        let delete_result = typed_collection.delete_many(filter, None).await.unwrap();
        println!("Deleted {} documents, col:{}", delete_result.deleted_count, col);
        
        for i in 0..jvn_year.item.len() {
            let insert_result = typed_collection.insert_one(jvn_year.item[i].clone(), None).await.unwrap();
            println!("document ID:{}, col:{}", insert_result.inserted_id, col);
        }
    }

    Ok(())
}
