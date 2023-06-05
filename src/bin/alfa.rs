use std::{vec, path::PathBuf, fs::File};
use hyper::Client;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value, Value::Null};
use std::io::{BufReader, Write};
use time::{OffsetDateTime, macros::offset, format_description};


#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct ScanResult {
    time:     String,
    hostname: String,
    ip:       Vec<String>,
    os:       String,
    kernel:   String,
    pkg:      Vec<PkgList>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct PkgList {
    pkgname:    String,
    pkgver:     String,
    pkgrelease: String,
    upver:      String,
    uprelease:  String,
    pkgarch:    String
}

//
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct Vulns {
    vulns: Vec<VulnsList>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct VulnsList {
    time:     String,
    hostname: String,
    ip:       Vec<String>,
    os:       String,
    kernel:   String,
    pkg:      DetectList
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct DetectList {
    pkgname:    String,
    pkgver:     String,
    pkgrelease: String,
    upver:      String,
    uprelease:  String,
    pkgarch:    String,
    detect:     Value
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
    let ext: Vec<&str> = f.split('.').collect();
    let index = ext.len() -1;

    if ext[index] == "json" {
      file_vec.push(f);
    }
  }

  for f in file_vec {
    println!("load file: {:?}", f);

    let mut vulns_vec = Vulns {
      vulns: vec![]
    };

    let file = match File::open(&f) {
      Ok(i) => i,
      Err(err) => panic!("File Open ERROR... {:?}", err),
    };

    let buf = BufReader::new(file);
    let scan_r: ScanResult = serde_json::from_reader(buf).unwrap();

    let release = &scan_r.os.split_whitespace().collect::<Vec<_>>();

    // RockyLinux 9
    if release[0] == "Rocky" && release[1] == "Linux" && release[2] == "release" {
      let majorver: Vec<&str> = release[3].split('.').collect();
      // 9
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
                    let v: Vec<&str> = ver.split(':').collect();

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
                      let hostname = String::from(&scan_r.hostname).replace('\n', "");

                      //ip
                      let ip = &scan_r.ip;

                      //os
                      let os = String::from(&scan_r.os).replace('\n', "");

                      //kernel
                      let kernel = String::from(&scan_r.kernel).replace('\n', "");

                      let detect_list = DetectList {
                        pkgname:    scan_p.pkgname.clone(),
                        pkgver:     scan_p.pkgver.clone(),
                        pkgrelease: scan_p.pkgrelease.clone(),
                        upver:      scan_p.upver.clone(),
                        uprelease:  scan_p.uprelease.clone(),
                        pkgarch:    scan_p.pkgarch.clone(),
                        detect:     d.clone()
                      };

                      let vulns_list = VulnsList {
                        time,
                        hostname,
                        ip:  scan_r.ip.clone(),
                        os,
                        kernel,
                        pkg: detect_list
                      };

                      vulns_vec.vulns.push(vulns_list);
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
                    let v: Vec<&str> = ver.split(':').collect();

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
                      let hostname = String::from(&scan_r.hostname).replace('\n', "");

                      //ip
                      let ip = &scan_r.ip;

                      //os
                      let os = String::from(&scan_r.os).replace('\n', "");

                      //kernel
                      let kernel = String::from(&scan_r.kernel).replace('\n', "");

                      let detect_list = DetectList {
                        pkgname:    scan_p.pkgname.clone(),
                        pkgver:     scan_p.pkgver.clone(),
                        pkgrelease: scan_p.pkgrelease.clone(),
                        upver:      scan_p.upver.clone(),
                        uprelease:  scan_p.uprelease.clone(),
                        pkgarch:    scan_p.pkgarch.clone(),
                        detect:     d.clone()
                      };

                      let vulns_list = VulnsList {
                        time,
                        hostname,
                        ip:  scan_r.ip.clone(),
                        os,
                        kernel,
                        pkg: detect_list
                      };

                      vulns_vec.vulns.push(vulns_list);
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

      // 8
      if majorver[0] == "8" {
        let url = String::from("http://127.0.0.1:7878/rhel8/");
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
                    let v: Vec<&str> = ver.split(':').collect();

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
                      let hostname = String::from(&scan_r.hostname).replace('\n', "");

                      //ip
                      let ip = &scan_r.ip;

                      //os
                      let os = String::from(&scan_r.os).replace('\n', "");

                      //kernel
                      let kernel = String::from(&scan_r.kernel).replace('\n', "");

                      let detect_list = DetectList {
                        pkgname:    scan_p.pkgname.clone(),
                        pkgver:     scan_p.pkgver.clone(),
                        pkgrelease: scan_p.pkgrelease.clone(),
                        upver:      scan_p.upver.clone(),
                        uprelease:  scan_p.uprelease.clone(),
                        pkgarch:    scan_p.pkgarch.clone(),
                        detect:     d.clone()
                      };

                      let vulns_list = VulnsList {
                        time,
                        hostname,
                        ip:  scan_r.ip.clone(),
                        os,
                        kernel,
                        pkg: detect_list
                      };

                      vulns_vec.vulns.push(vulns_list);
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
                    let v: Vec<&str> = ver.split(':').collect();

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
                      let hostname = String::from(&scan_r.hostname).replace('\n', "");

                      //ip
                      let ip = &scan_r.ip;

                      //os
                      let os = String::from(&scan_r.os).replace('\n', "");

                      //kernel
                      let kernel = String::from(&scan_r.kernel).replace('\n', "");

                      let detect_list = DetectList {
                        pkgname:    scan_p.pkgname.clone(),
                        pkgver:     scan_p.pkgver.clone(),
                        pkgrelease: scan_p.pkgrelease.clone(),
                        upver:      scan_p.upver.clone(),
                        uprelease:  scan_p.uprelease.clone(),
                        pkgarch:    scan_p.pkgarch.clone(),
                        detect:     d.clone()
                      };

                      let vulns_list = VulnsList {
                        time,
                        hostname,
                        ip:  scan_r.ip.clone(),
                        os,
                        kernel,
                        pkg: detect_list
                      };

                      vulns_vec.vulns.push(vulns_list);
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

    // AlmaLinux
    if release[0] == "AlmaLinux" && release[1] == "release" {
      let majorver: Vec<&str> = release[2].split('.').collect();
      // 9
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
                    let v: Vec<&str> = ver.split(':').collect();

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
                      let hostname = String::from(&scan_r.hostname).replace('\n', "");

                      //ip
                      let ip = &scan_r.ip;

                      //os
                      let os = String::from(&scan_r.os).replace('\n', "");

                      //kernel
                      let kernel = String::from(&scan_r.kernel).replace('\n', "");

                      let detect_list = DetectList {
                        pkgname:    scan_p.pkgname.clone(),
                        pkgver:     scan_p.pkgver.clone(),
                        pkgrelease: scan_p.pkgrelease.clone(),
                        upver:      scan_p.upver.clone(),
                        uprelease:  scan_p.uprelease.clone(),
                        pkgarch:    scan_p.pkgarch.clone(),
                        detect:     d.clone()
                      };

                      let vulns_list = VulnsList {
                        time,
                        hostname,
                        ip:  scan_r.ip.clone(),
                        os,
                        kernel,
                        pkg: detect_list
                      };

                      vulns_vec.vulns.push(vulns_list);
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
                    let v: Vec<&str> = ver.split(':').collect();

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
                      let hostname = String::from(&scan_r.hostname).replace('\n', "");

                      //ip
                      let ip = &scan_r.ip;

                      //os
                      let os = String::from(&scan_r.os).replace('\n', "");

                      //kernel
                      let kernel = String::from(&scan_r.kernel).replace('\n', "");

                      let detect_list = DetectList {
                        pkgname:    scan_p.pkgname.clone(),
                        pkgver:     scan_p.pkgver.clone(),
                        pkgrelease: scan_p.pkgrelease.clone(),
                        upver:      scan_p.upver.clone(),
                        uprelease:  scan_p.uprelease.clone(),
                        pkgarch:    scan_p.pkgarch.clone(),
                        detect:     d.clone()
                      };

                      let vulns_list = VulnsList {
                        time,
                        hostname,
                        ip:  scan_r.ip.clone(),
                        os,
                        kernel,
                        pkg: detect_list
                      };

                      vulns_vec.vulns.push(vulns_list);
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

      // 8
      if majorver[0] == "8" {
        let url = String::from("http://127.0.0.1:7878/rhel8/");
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
                    let v: Vec<&str> = ver.split(':').collect();

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
                      let hostname = String::from(&scan_r.hostname).replace('\n', "");

                      //ip
                      let ip = &scan_r.ip;

                      //os
                      let os = String::from(&scan_r.os).replace('\n', "");

                      //kernel
                      let kernel = String::from(&scan_r.kernel).replace('\n', "");

                      let detect_list = DetectList {
                        pkgname:    scan_p.pkgname.clone(),
                        pkgver:     scan_p.pkgver.clone(),
                        pkgrelease: scan_p.pkgrelease.clone(),
                        upver:      scan_p.upver.clone(),
                        uprelease:  scan_p.uprelease.clone(),
                        pkgarch:    scan_p.pkgarch.clone(),
                        detect:     d.clone()
                      };

                      let vulns_list = VulnsList {
                        time,
                        hostname,
                        ip:  scan_r.ip.clone(),
                        os,
                        kernel,
                        pkg: detect_list
                      };

                      vulns_vec.vulns.push(vulns_list);
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
                    let v: Vec<&str> = ver.split(':').collect();

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
                      let hostname = String::from(&scan_r.hostname).replace('\n', "");

                      //ip
                      let ip = &scan_r.ip;

                      //os
                      let os = String::from(&scan_r.os).replace('\n', "");

                      //kernel
                      let kernel = String::from(&scan_r.kernel).replace('\n', "");

                      let detect_list = DetectList {
                        pkgname:    scan_p.pkgname.clone(),
                        pkgver:     scan_p.pkgver.clone(),
                        pkgrelease: scan_p.pkgrelease.clone(),
                        upver:      scan_p.upver.clone(),
                        uprelease:  scan_p.uprelease.clone(),
                        pkgarch:    scan_p.pkgarch.clone(),
                        detect:     d.clone()
                      };

                      let vulns_list = VulnsList {
                        time,
                        hostname,
                        ip:  scan_r.ip.clone(),
                        os,
                        kernel,
                        pkg: detect_list
                      };

                      vulns_vec.vulns.push(vulns_list);
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
      let majorver: Vec<&str> = release[3].split('.').collect();
      // 8
      if majorver[0] == "8" {
        let url = String::from("http://127.0.0.1:7878/rhel8/");
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
                    let v: Vec<&str> = ver.split(':').collect();

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
                      let hostname = String::from(&scan_r.hostname).replace('\n', "");

                      //ip
                      let ip = &scan_r.ip;

                      //os
                      let os = String::from(&scan_r.os).replace('\n', "");

                      //kernel
                      let kernel = String::from(&scan_r.kernel).replace('\n', "");

                      let detect_list = DetectList {
                        pkgname:    scan_p.pkgname.clone(),
                        pkgver:     scan_p.pkgver.clone(),
                        pkgrelease: scan_p.pkgrelease.clone(),
                        upver:      scan_p.upver.clone(),
                        uprelease:  scan_p.uprelease.clone(),
                        pkgarch:    scan_p.pkgarch.clone(),
                        detect:     d.clone()
                      };

                      let vulns_list = VulnsList {
                        time,
                        hostname,
                        ip:  scan_r.ip.clone(),
                        os,
                        kernel,
                        pkg: detect_list
                      };

                      vulns_vec.vulns.push(vulns_list);
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
                    let v: Vec<&str> = ver.split(':').collect();

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
                      let hostname = String::from(&scan_r.hostname).replace('\n', "");

                      //ip
                      let ip = &scan_r.ip;

                      //os
                      let os = String::from(&scan_r.os).replace('\n', "");

                      //kernel
                      let kernel = String::from(&scan_r.kernel).replace('\n', "");

                      let detect_list = DetectList {
                        pkgname:    scan_p.pkgname.clone(),
                        pkgver:     scan_p.pkgver.clone(),
                        pkgrelease: scan_p.pkgrelease.clone(),
                        upver:      scan_p.upver.clone(),
                        uprelease:  scan_p.uprelease.clone(),
                        pkgarch:    scan_p.pkgarch.clone(),
                        detect:     d.clone()
                      };

                      let vulns_list = VulnsList {
                        time,
                        hostname,
                        ip:  scan_r.ip.clone(),
                        os,
                        kernel,
                        pkg: detect_list
                      };

                      vulns_vec.vulns.push(vulns_list);
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

      // 7
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
                    let v: Vec<&str> = ver.split(':').collect();

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
                      let hostname = String::from(&scan_r.hostname).replace('\n', "");

                      //ip
                      let ip = &scan_r.ip;

                      //os
                      let os = String::from(&scan_r.os).replace('\n', "");

                      //kernel
                      let kernel = String::from(&scan_r.kernel).replace('\n', "");

                      let detect_list = DetectList {
                        pkgname:    scan_p.pkgname.clone(),
                        pkgver:     scan_p.pkgver.clone(),
                        pkgrelease: scan_p.pkgrelease.clone(),
                        upver:      scan_p.upver.clone(),
                        uprelease:  scan_p.uprelease.clone(),
                        pkgarch:    scan_p.pkgarch.clone(),
                        detect:     d.clone()
                      };

                      let vulns_list = VulnsList {
                        time,
                        hostname,
                        ip:  scan_r.ip.clone(),
                        os,
                        kernel,
                        pkg: detect_list
                      };

                      vulns_vec.vulns.push(vulns_list);
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
                    let v: Vec<&str> = ver.split(':').collect();

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
                      let hostname = String::from(&scan_r.hostname).replace('\n', "");

                      //ip
                      let ip = &scan_r.ip;

                      //os
                      let os = String::from(&scan_r.os).replace('\n', "");

                      //kernel
                      let kernel = String::from(&scan_r.kernel).replace('\n', "");

                      let detect_list = DetectList {
                        pkgname:    scan_p.pkgname.clone(),
                        pkgver:     scan_p.pkgver.clone(),
                        pkgrelease: scan_p.pkgrelease.clone(),
                        upver:      scan_p.upver.clone(),
                        uprelease:  scan_p.uprelease.clone(),
                        pkgarch:    scan_p.pkgarch.clone(),
                        detect:     d.clone()
                      };

                      let vulns_list = VulnsList {
                        time,
                        hostname,
                        ip:  scan_r.ip.clone(),
                        os,
                        kernel,
                        pkg: detect_list
                      };

                      vulns_vec.vulns.push(vulns_list);
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

      // 6
      if majorver[0] == "6" {
        let url = String::from("http://127.0.0.1:7878/rhel6/");
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
                    let v: Vec<&str> = ver.split(':').collect();

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
                      let hostname = String::from(&scan_r.hostname).replace('\n', "");

                      //ip
                      let ip = &scan_r.ip;

                      //os
                      let os = String::from(&scan_r.os).replace('\n', "");

                      //kernel
                      let kernel = String::from(&scan_r.kernel).replace('\n', "");

                      let detect_list = DetectList {
                        pkgname:    scan_p.pkgname.clone(),
                        pkgver:     scan_p.pkgver.clone(),
                        pkgrelease: scan_p.pkgrelease.clone(),
                        upver:      scan_p.upver.clone(),
                        uprelease:  scan_p.uprelease.clone(),
                        pkgarch:    scan_p.pkgarch.clone(),
                        detect:     d.clone()
                      };

                      let vulns_list = VulnsList {
                        time,
                        hostname,
                        ip:  scan_r.ip.clone(),
                        os,
                        kernel,
                        pkg: detect_list
                      };

                      vulns_vec.vulns.push(vulns_list);
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
                    let v: Vec<&str> = ver.split(':').collect();

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
                      let hostname = String::from(&scan_r.hostname).replace('\n', "");

                      //ip
                      let ip = &scan_r.ip;

                      //os
                      let os = String::from(&scan_r.os).replace('\n', "");

                      //kernel
                      let kernel = String::from(&scan_r.kernel).replace('\n', "");

                      let detect_list = DetectList {
                        pkgname:    scan_p.pkgname.clone(),
                        pkgver:     scan_p.pkgver.clone(),
                        pkgrelease: scan_p.pkgrelease.clone(),
                        upver:      scan_p.upver.clone(),
                        uprelease:  scan_p.uprelease.clone(),
                        pkgarch:    scan_p.pkgarch.clone(),
                        detect:     d.clone()
                      };

                      let vulns_list = VulnsList {
                        time,
                        hostname,
                        ip:  scan_r.ip.clone(),
                        os,
                        kernel,
                        pkg: detect_list
                      };

                      vulns_vec.vulns.push(vulns_list);
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

    std::fs::create_dir_all("./src/vulns_result").unwrap();
    let filename = String::from(d_file[d_index]);
    let dir = String::from("./src/vulns_result/") + &filename;

    let serialized = serde_json::to_string(&vulns_vec).unwrap();
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