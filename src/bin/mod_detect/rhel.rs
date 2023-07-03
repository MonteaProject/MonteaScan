use crate::{ScanResult, Cwe, CweResult};

use anyhow::Result;
use time::{OffsetDateTime, macros::offset, format_description};
use serde::{Deserialize, Serialize};
use serde_json::{Value, Value::Null};
use std::fs::File;
use std::io::Write;


#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct Vulns {
  time:        String,
  hostname:    String,
  ip:          Vec<String>,
  os:          String,
  kernel:      String,
  issued:      String,
  updated:     String,
  impact:      String,
  cveid:       String,
  cwe_oval:    String,
  cvssv3_oval: String,
  cwe_name:    String,
  cwe_url_vec: Vec<String>,
  pkgname:     String,
  pkgver:      String,
  pkgrelease:  String,
  update_flag: String,
  upver:       String,
  uprelease:   String,
  pkgarch:     String,
  detect:      Value
}


pub async fn main(url: String, scan_r: ScanResult, f: String, result_dir: String) -> Result<Vec<CweResult>>  {
  let mut vulns_vec: Vec<Vulns> = Vec::new();
  let mut cwe_vec: Vec<CweResult> = Vec::new();

  let response = reqwest::get(&url).await?;
  let bytes = response.bytes().await?;
  let data: String = String::from_utf8(bytes.to_vec())?;

  let v: Value = serde_json::from_str(&data)?;

  if let Some(oval_vec) = v.as_array() {
    let mut detect_flag: usize = 0;

    for scan_p in &scan_r.pkg {
      let utc: OffsetDateTime = OffsetDateTime::now_utc();
      let jct: OffsetDateTime = utc.to_offset(offset!(+9));
      let format: Vec<format_description::FormatItem<'_>> = format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]")?;

      let time: String = jct.format(&format)?;
      let hostname: String = String::from(&scan_r.hostname).replace('\n', "");
      let ip: &Vec<String> = &scan_r.ip;
      let os: String = String::from(&scan_r.os).replace('\n', "");
      let kernel: String = String::from(&scan_r.kernel).replace('\n', "");

      for oval in oval_vec {
        if let Some(v) = oval.as_array() {
          for i in 0..v.len() {
            if let Some(v) = oval[i]["criteria"]["criteria"].as_array() {
              for i in 0..v.len() {
                if let Some(v) = oval[i]["criteria"]["criteria"][i]["criterion"].as_array() {
                  let mut comment_vec: Vec<String> = Vec::new();

                  for i in 0..v.len() {
                    if let Some(v) = oval[i]["criteria"]["criteria"][i]["criterion"][i]["@comment"].as_str() {
                      comment_vec.push(v.to_string());
                    }
                  }

                  let count: usize = comment_vec.len();
                  if count == 3 {
                    let result: Vec<&str> = comment_vec[1].split("is earlier than").collect();
                    if result.len() == 2 {
                      let pkg: &str = result[0].trim();
                      let ver: &str = result[1].trim();

                      if pkg == scan_p.pkgname {
                        let v: Vec<&str> = ver.split(':').collect();

                        let mut p: String = String::from(&scan_p.pkgver);
                        p += "-";
                        p += &scan_p.pkgrelease;
                        
                        if v[1] == p {
                          let mut issued: String = "-".to_string();
                          if oval[i]["metadata"]["advisory"]["issued"]["@date"] != Null {
                            issued = oval[i]["metadata"]["advisory"]["issued"]["@date"].to_string().replace('"', "");
                          }

                          let mut updated: String = "-".to_string();
                          if oval[i]["metadata"]["advisory"]["updated"]["@date"] != Null {
                            updated = oval[i]["metadata"]["advisory"]["updated"]["@date"].to_string().replace('"', "");
                          }

                          let mut impact:      String = "-".to_string();
                          let mut cveid:       String = "-".to_string();
                          let mut cvssv3_oval: String = "-".to_string();
                          let mut cwe_oval:    String = "-".to_string();
                          let mut cwe_name:    String = "-".to_string();
                          let mut cwe_url_vec: Vec<String> = vec!["-".to_string(); 0];

                          let cwe_read: String = String::from("./src/cwe/cwe.json");
                          let cwe: Cwe = {
                            let cwe: String = std::fs::read_to_string(&cwe_read)?;
                            serde_json::from_str::<Cwe>(&cwe)?
                          };

                          if oval[i]["metadata"]["advisory"]["cve"] != Null {
                            if let Some(v) = oval[i]["metadata"]["advisory"]["cve"].as_array() {
                              for i in 0..v.len() {
                                if oval[i]["metadata"]["advisory"]["cve"][i]["@impact"] != Null {
                                  impact = oval[i]["metadata"]["advisory"]["cve"][i]["@impact"].to_string().replace('"', "");
                                }
            
                                if oval[i]["metadata"]["advisory"]["cve"][i]["$value"] != Null {
                                  cveid = oval[i]["metadata"]["advisory"]["cve"][i]["$value"].to_string().replace('"', "");
                                }
            
                                if oval[i]["metadata"]["advisory"]["cve"][i]["@cvss3"] != Null {
                                  let s1 = oval[i]["metadata"]["advisory"]["cve"][i]["@cvss3"].to_string().replace('"', "");
                                  let s2: Vec<&str> = s1.split('/').collect();
                                  cvssv3_oval = s2[0].to_string();
                                }
            
                                if oval[i]["metadata"]["advisory"]["cve"][i]["@cwe"] != Null {
                                  cwe_oval = oval[i]["metadata"]["advisory"]["cve"][i]["@cwe"].to_string().replace('"', "");
            
                                  let s1: &String   = &cwe_oval.replace("CWE-", "");
                                  let s2: &String   = &s1.replace('(', "");
                                  let s3: &String   = &s2.replace(')', "");
                                  let s4: Vec<&str> = s3.split('|').collect();
                                  for i in s4 {
                                    let cwe_url = String::from("https://cwe.mitre.org/data/definitions/") + i + ".html";
                                    cwe_url_vec.push(cwe_url);
                                  }
                                  
                                  for i in 0..cwe.Weaknesses.Weakness.len() {
                                    let cwe_id = &cwe.Weaknesses.Weakness[i].id.clone().unwrap_or(0.to_string());
            
                                    if s1 == cwe_id {
                                      cwe_name = cwe.Weaknesses.Weakness[i].name.clone().unwrap_or("None".to_string());
            
                                      let cwe_list: CweResult = CweResult{
                                        time:     time.clone(),
                                        hostname: hostname.clone(),
                                        ip:       ip.clone(),
                                        os:       os.clone(),
                                        kernel:   kernel.clone(),
                                        cwe_id:   cwe_id.clone(),
                                        cwe_name: cwe_name.clone()
                                      };
                                      cwe_vec.push(cwe_list);
                                    }
                                  }
                                }
                                
                                let vulns_list: Vulns = Vulns {
                                  time:        time.clone(),
                                  hostname:    hostname.clone(),
                                  ip:          ip.clone(),
                                  os:          os.clone(),
                                  kernel:      kernel.clone(),
                                  issued:      issued.clone(),
                                  updated:     updated.clone(),
                                  impact:      impact.clone(),
                                  cveid:       cveid.clone(),
                                  cwe_oval:    cwe_oval.clone(),
                                  cwe_name:    cwe_name.clone(),
                                  cwe_url_vec: cwe_url_vec.clone(),
                                  cvssv3_oval: cvssv3_oval.clone(),
                                  pkgname:     scan_p.pkgname.clone(),
                                  pkgver:      scan_p.pkgver.clone(),
                                  pkgrelease:  scan_p.pkgrelease.clone(),
                                  update_flag: scan_p.update_flag.clone(),
                                  upver:       scan_p.upver.clone(),
                                  uprelease:   scan_p.uprelease.clone(),
                                  pkgarch:     scan_p.pkgarch.clone(),
                                  detect:      oval.clone()
                                };
                                vulns_vec.push(vulns_list);
                              }
                            }
                          } else {
                            let vulns_list: Vulns = Vulns {
                              time:        time.clone(),
                              hostname:    hostname.clone(),
                              ip:          ip.clone(),
                              os:          os.clone(),
                              kernel:      kernel.clone(),
                              issued:      issued.clone(),
                              updated:     updated.clone(),
                              impact:      impact.clone(),
                              cveid:       cveid.clone(),
                              cwe_oval:    cwe_oval.clone(),
                              cwe_name:    cwe_name.clone(),
                              cwe_url_vec: cwe_url_vec.clone(),
                              cvssv3_oval: cvssv3_oval.clone(),
                              pkgname:     scan_p.pkgname.clone(),
                              pkgver:      scan_p.pkgver.clone(),
                              pkgrelease:  scan_p.pkgrelease.clone(),
                              update_flag: scan_p.update_flag.clone(),
                              upver:       scan_p.upver.clone(),
                              uprelease:   scan_p.uprelease.clone(),
                              pkgarch:     scan_p.pkgarch.clone(),
                              detect:      oval.clone()
                            };
                            vulns_vec.push(vulns_list);
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }

      if vulns_vec.len() == detect_flag {

        let issued:      String = "-".to_string();
        let updated:     String = "-".to_string();
        let impact:      String = "-".to_string();
        let cveid:       String = "-".to_string();
        let cwe_oval:    String = "-".to_string();
        let cwe_name:    String = "-".to_string();
        let cwe_url_vec: Vec<String> = vec!["-".to_string(); 0];
        let cvssv3_oval: String = "-".to_string();
        
        let vulns_list: Vulns = Vulns {
          time:        time.clone(),
          hostname:    hostname.clone(),
          ip:          ip.clone(),
          os:          os.clone(),
          kernel:      kernel.clone(),
          issued:      issued.clone(),
          updated:     updated.clone(),
          impact:      impact.clone(),
          cveid:       cveid.clone(),
          cwe_oval:    cwe_oval.clone(),
          cwe_name:    cwe_name.clone(),
          cwe_url_vec: cwe_url_vec.clone(),
          cvssv3_oval: cvssv3_oval.clone(),
          pkgname:     scan_p.pkgname.clone(),
          pkgver:      scan_p.pkgver.clone(),
          pkgrelease:  scan_p.pkgrelease.clone(),
          update_flag: scan_p.update_flag.clone(),
          upver:       scan_p.upver.clone(),
          uprelease:   scan_p.uprelease.clone(),
          pkgarch:     scan_p.pkgarch.clone(),
          detect:      Null
        };

        vulns_vec.push(vulns_list);
      } else {
        detect_flag = vulns_vec.len();
      }
    }
  }

  let d_file: Vec<&str> = f.split('/').collect();
  let d_index: usize = d_file.len()-1;

  let filename: String = String::from(d_file[d_index]);
  let full_path: String = String::from(&result_dir) + &filename;

  let serialized = serde_json::to_string(&vulns_vec)?;
  let mut w: File = std::fs::OpenOptions::new()
    .write(true)
    .create(true)
    .open(full_path)?;
  w.write_all(serialized.as_bytes())?;

  println!("finished: {:?}", f);

  Ok(cwe_vec)
}