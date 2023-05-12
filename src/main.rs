use hyper::Client;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value, Value::Null};

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

    for d in oval_vec {
        let epty_vec: Vec<Value> = Vec::new();
        let mut result_vec: Vec<String> = Vec::new();

        let j = d[0]["criteria"]["criteria"][0]["criterion"].as_array().unwrap_or_else(|| &epty_vec);

        for i in j {
            let comment = i["@comment"].as_str().unwrap();
            result_vec.push(comment.to_string());
        }

        let a = result_vec.len();
        if a == 3 {
            let reg = &result_vec[0];
            let regular: &str = &reg[0..7];

            let rev = &result_vec[0];
            let reverse: String = rev.chars().rev().collect::<String>();
            let reverse_c = &reverse[0..8];
            
            if regular == "Module" && reverse_c == "delbane" {
                let label = String::from("Module is enabled");
            }
        }
        
        if d[0]["criteria"]["criteria"][1]["criterion"] != Null {
            let ept_vec: Vec<Value> = Vec::new();
            let mut result_vec: Vec<String> = Vec::new();
            let j = d[0]["criteria"]["criteria"][1]["criterion"].as_array().unwrap_or_else(|| &ept_vec);
            for i in j{
                let comment = i["@comment"].as_str().unwrap();
                result_vec.push(comment.to_string());
            }
            let a = result_vec.len();
            if a == 3 {
                println!("{:?}", result_vec);
            }
        }
        if d[1] != Null {
            println!("code[388]:定義されていない新しい値が追加されています: {:?}", d[1]);
        }
    }

    Ok(())
}
