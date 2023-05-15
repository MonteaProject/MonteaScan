use std::{vec, path::PathBuf, fs::File, io::BufRead};
use hyper::Client;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value, Value::Null};
use std::io::{BufReader, Read, Write};
use time::{OffsetDateTime, macros::offset, format_description};


#[derive(Deserialize, Serialize, Debug)]
struct ScanResult {
    time: String,
    hostname: String,
    ip: Vec<String>,
    os: String,
    kernel: String,
    update: Vec<UpdateList>,
    pkg: Vec<PkgList>
}

#[derive(Deserialize, Serialize, Debug)]
struct UpdateList {
    name: String,
    ver: String,
    repo: String
}

#[derive(Deserialize, Serialize, Debug)]
struct PkgList {
    pkgname: String,
    pkgver: String,
    pkgrelease: String,
    pkgarch: String
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct VlunsList {
    detect: Vec<VlunsDetect>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct VlunsDetect {
    time: String,
    hostname: String,
    ip: Vec<String>,
    os: String,
    kernel: String,
    oval: Value
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct Vluns {
    time: String,
    hostname: String,
    ip: Vec<String>,
    os: String,
    kernel: String,
    detect: Ovalinfo
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct Ovalinfo {
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
    affected: Option<Affected>,
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
    criterion: Option<Vec<Criterion2>>
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
    println!("start...");

    let mut file_vec: Vec<String> = Vec::new();

    let path = String::from("./src/scan_result/");
    let dir = PathBuf::from(path);
    let files = dir.read_dir().expect("code[387]: フォルダが存在しません.");

    for file in files {
        let f = file.iter().map(|x| x.path().to_string_lossy().into_owned()).collect::<String>();
        let ext: Vec<&str> = f.split(".").collect();
        let index = ext.len() -1;

        if ext[index] == "json" {
            file_vec.push(f);
        }
    }

    for f in file_vec {
        println!("load file: {:?}", f);

        let mut vluns_l = VlunsList {
            detect: vec![]
        };

        let file = match File::open(&f) {
            Ok(i) => i,
            Err(err) => panic!("File Open ERROR... {:?}", err),
        };

        let buf = BufReader::new(file);
        let scan_r: ScanResult = serde_json::from_reader(buf).unwrap();

        let release = &scan_r.os.split_whitespace().collect::<Vec<_>>();

        // RockyLinux
        if release[0] == "Rocky" && release[1] == "Linux" && release[2] == "release" {
            let majorver: Vec<&str> = release[3].split(".").collect();
            if majorver[0] == "9" {
                let url = String::from("http://127.0.0.1:7878/rhel9/");
                let client = Client::new();

                let res = client.get(url.parse().unwrap()).await.unwrap();
                let resp = hyper::body::to_bytes(res.into_body()).await.unwrap();
                let data = String::from_utf8(resp.to_vec()).expect("response was not valid utf-8");

                let v: Value = serde_json::from_str(&data).unwrap();

                let empty_vec: Vec<Value> = Vec::new();
                let oval_vec = v.as_array().unwrap_or_else(|| &empty_vec);

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

                                for scan_p in &scan_r.pkg {
                                    if pkg == scan_p.pkgname {
                                        let v: Vec<&str> = ver.split(":").collect();
        
                                        let mut p = String::from(&scan_p.pkgver);
                                        p += "-";
                                        p += &scan_p.pkgrelease;
                                        
                                        if v[1] == p {
                                            //time
                                            let utc = OffsetDateTime::now_utc();
                                            let jct = utc.to_offset(offset!(+9));
                                            let format = format_description::parse(
                                                "[year]-[month]-[day] [hour]:[minute]:[second]"
                                            ).unwrap();
                                            let time = jct.format(&format).unwrap();

                                            //hostname
                                            let hostname = String::from(&scan_r.hostname).replace("\n", "");

                                            //ip
                                            let ip = &scan_r.ip;

                                            //os
                                            let os = String::from(&scan_r.os).replace("\n", "");

                                            //kernel
                                            let kernel = String::from(&scan_r.kernel).replace("\n", "");

                                            let mut vluns = VlunsDetect {
                                                time: time,
                                                hostname: hostname,
                                                ip: scan_r.ip.clone(),
                                                os: os,
                                                kernel: kernel,
                                                oval: d.clone()
                                            };

                                            vluns_l.detect.push(vluns);
                                        }
                                    }
                                }
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

                                for scan_p in &scan_r.pkg {
                                    if pkg == scan_p.pkgname {
                                        let v: Vec<&str> = ver.split(":").collect();
        
                                        let mut p = String::from(&scan_p.pkgver);
                                        p += "-";
                                        p += &scan_p.pkgrelease;
                                        
                                        if v[1] == p {
                                            //time
                                            let utc = OffsetDateTime::now_utc();
                                            let jct = utc.to_offset(offset!(+9));
                                            let format = format_description::parse(
                                                "[year]-[month]-[day] [hour]:[minute]:[second]"
                                            ).unwrap();
                                            let time = jct.format(&format).unwrap();

                                            //hostname
                                            let hostname = String::from(&scan_r.hostname).replace("\n", "");

                                            //ip
                                            let ip = &scan_r.ip;

                                            //os
                                            let os = String::from(&scan_r.os).replace("\n", "");

                                            //kernel
                                            let kernel = String::from(&scan_r.kernel).replace("\n", "");

                                            let mut vluns = VlunsDetect {
                                                time: time,
                                                hostname: hostname,
                                                ip: scan_r.ip.clone(),
                                                os: os,
                                                kernel: kernel,
                                                oval: d.clone()
                                            };

                                            vluns_l.detect.push(vluns);
                                        }
                                    }
                                }
                            }
                        }
                    }

                    if d[1] != Null {
                        println!("code[388]: 定義されていない新しい値が追加されています: {:?}", d[1]);
                    }
                }
            }
        }

        // CentOS
        if release[0] == "CentOS" && release[1] == "Linux" && release[2] == "release" {
            let majorver: Vec<&str> = release[3].split(".").collect();
            if majorver[0] == "7" {
                let url = String::from("http://127.0.0.1:7878/rhel7/");
                let client = Client::new();

                let res = client.get(url.parse().unwrap()).await.unwrap();
                let resp = hyper::body::to_bytes(res.into_body()).await.unwrap();
                let data = String::from_utf8(resp.to_vec()).expect("response was not valid utf-8");

                let v: Value = serde_json::from_str(&data).unwrap();

                let empty_vec: Vec<Value> = Vec::new();
                let oval_vec = v.as_array().unwrap_or_else(|| &empty_vec);

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

                                for scan_p in &scan_r.pkg {
                                    if pkg == scan_p.pkgname {
                                        let v: Vec<&str> = ver.split(":").collect();
        
                                        let mut p = String::from(&scan_p.pkgver);
                                        p += "-";
                                        p += &scan_p.pkgrelease;
                                        
                                        if v[1] == p {
                                            //time
                                            let utc = OffsetDateTime::now_utc();
                                            let jct = utc.to_offset(offset!(+9));
                                            let format = format_description::parse(
                                                "[year]-[month]-[day] [hour]:[minute]:[second]"
                                            ).unwrap();
                                            let time = jct.format(&format).unwrap();

                                            //hostname
                                            let hostname = String::from(&scan_r.hostname).replace("\n", "");

                                            //ip
                                            let ip = &scan_r.ip;

                                            //os
                                            let os = String::from(&scan_r.os).replace("\n", "");

                                            //kernel
                                            let kernel = String::from(&scan_r.kernel).replace("\n", "");

                                            let mut vluns = VlunsDetect {
                                                time: time,
                                                hostname: hostname,
                                                ip: scan_r.ip.clone(),
                                                os: os,
                                                kernel: kernel,
                                                oval: d.clone()
                                            };

                                            vluns_l.detect.push(vluns);
                                        }
                                    }
                                }
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

                                for scan_p in &scan_r.pkg {
                                    if pkg == scan_p.pkgname {
                                        let v: Vec<&str> = ver.split(":").collect();
        
                                        let mut p = String::from(&scan_p.pkgver);
                                        p += "-";
                                        p += &scan_p.pkgrelease;
                                        
                                        if v[1] == p {
                                            //time
                                            let utc = OffsetDateTime::now_utc();
                                            let jct = utc.to_offset(offset!(+9));
                                            let format = format_description::parse(
                                                "[year]-[month]-[day] [hour]:[minute]:[second]"
                                            ).unwrap();
                                            let time = jct.format(&format).unwrap();

                                            //hostname
                                            let hostname = String::from(&scan_r.hostname).replace("\n", "");

                                            //ip
                                            let ip = &scan_r.ip;

                                            //os
                                            let os = String::from(&scan_r.os).replace("\n", "");

                                            //kernel
                                            let kernel = String::from(&scan_r.kernel).replace("\n", "");

                                            let mut vluns = VlunsDetect {
                                                time: time,
                                                hostname: hostname,
                                                ip: scan_r.ip.clone(),
                                                os: os,
                                                kernel: kernel,
                                                oval: d.clone()
                                            };

                                            vluns_l.detect.push(vluns);
                                        }
                                    }
                                }
                            }
                        }
                    }

                    if d[1] != Null {
                        println!("code[388]: 定義されていない新しい値が追加されています: {:?}", d[1]);
                    }
                }
            }
        }

        let d_file: Vec<&str> = f.split("/").collect();
        let d_index = d_file.len()-1;

        std::fs::create_dir_all("./src/vluns_result").unwrap();
        let filename = String::from(d_file[d_index]);
        let dir = String::from("./src/vluns_result/") + &filename;

        let serialized = serde_json::to_string(&vluns_l).unwrap();
        let mut w = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(dir).unwrap();
        w.write_all(serialized.as_bytes());


        println!("finished: {:?}", f);
    }
    println!("finished...");

    Ok(())
}
