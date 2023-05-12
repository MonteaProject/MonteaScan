use std::vec;
use hyper::Client;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value, Value::Null};

#[derive(Debug, Clone)]
struct OvalPkg {
    pkg: Vec<String>,
    ver: Vec<String>
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let url = String::from("http://127.0.0.1:7878/rhel6/");
    let client = Client::new();

    let res = client.get(url.parse().unwrap()).await.unwrap();
    let resp = hyper::body::to_bytes(res.into_body()).await.unwrap();
    let data = String::from_utf8(resp.to_vec()).expect("response was not valid utf-8");

    let v: Value = serde_json::from_str(&data).unwrap();

    let empty_vec: Vec<Value> = Vec::new();
    let oval_vec = v.as_array().unwrap_or_else(|| &empty_vec);

    let mut ovalpkg = OvalPkg {
        pkg: vec![],
        ver: vec![]
    };

    for d in oval_vec {
        if d[0]["criteria"]["criteria"][0]["criterion"] != Null {
            let epty_vec: Vec<Value> = Vec::new();
            let mut result_vec: Vec<String> = Vec::new();

            let j = d[0]["criteria"]["criteria"][0]["criterion"].as_array().unwrap_or_else(|| &epty_vec);

            for i in j {
                let comment = i["@comment"].as_str().unwrap();
                result_vec.push(comment.to_string());
            }

            let count = result_vec.len();
            if count == 3 {
                let b: Vec<&str> = result_vec[1].split("is earlier than").collect();

                if b.len() == 2{
                    let pkg = b[0].trim();
                    let ver = b[1].trim();
                    ovalpkg.pkg.push(pkg.to_string());
                    ovalpkg.ver.push(ver.to_string());
                }
            }
        }
        
        if d[0]["criteria"]["criteria"][1]["criterion"] != Null {
            let epty_vec: Vec<Value> = Vec::new();
            let mut result_vec: Vec<String> = Vec::new();

            let j = d[0]["criteria"]["criteria"][1]["criterion"].as_array().unwrap_or_else(|| &epty_vec);

            for i in j{
                let comment = i["@comment"].as_str().unwrap();
                result_vec.push(comment.to_string());
            }

            let count = result_vec.len();
            if count == 3 {
                let b: Vec<&str> = result_vec[1].split("is earlier than").collect();

                if b.len() == 2{
                    let pkg = b[0].trim();
                    let ver = b[1].trim();
                    ovalpkg.pkg.push(pkg.to_string());
                    ovalpkg.ver.push(ver.to_string());
                }
            }
        }

        if d[1] != Null {
            println!("code[388]:定義されていない新しい値が追加されています: {:?}", d[1]);
        }
    }

    println!("{:#?}", ovalpkg);

    Ok(())
}
