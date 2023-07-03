use crate::ScanResult;

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


pub async fn main(url: String, scan_r: ScanResult, f: String, result_dir: String) -> Result<()> {
  let mut vulns_vec: Vec<Vulns> = Vec::new();

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
          for x in 0..v.len() {
            let mut comment_vec: Vec<String> = Vec::new();

            if let Some(v) = oval[x]["criteria"]["criteria"]["criterion"].as_array() {
              for y in 0..v.len() {
                if let Some(v) = oval[x]["criteria"]["criteria"]["criterion"][y]["@comment"].as_str() {
                  comment_vec.push(v.to_string());
                }
              }
            }
            
            for comment in comment_vec {
              // "(CVE-2014-2686) ansible package in bionic, is related to the CVE in some way and has been fixed (note: '2.5.1+dfsg-1ubuntu0.1')."
              let c: Vec<&str> = comment.split("package in").collect();

              if c.len() == 2 {
                // "(CVE-2014-2686) ansible
                let s1: Vec<&str> = c[0].split_whitespace().collect::<Vec<&str>>();
                let pkg: &str = s1[1].trim();

                // bionic, is related to the CVE in some way and has been fixed (note: '2.5.1+dfsg-1ubuntu0.1')."
                let s2: Vec<&str> = c[1].trim().split("note:").collect();
                if s2.len() == 2 {
                  // '2.5.1+dfsg-1ubuntu0.1')."
                  let s3: Vec<&str> = s2[1].trim().split(')').collect();
                  
                  if s3.len() == 2 {
                    // '2.5.1+dfsg-1ubuntu0.1'
                    let ver: String = s3[0].trim().replace('\'', "");

                    if pkg == scan_p.pkgname {
                      // let v: Vec<&str> = ver.split(':').collect();
  
                      // 2:6.2.1+dfsg-3ubuntu1
                      let s1: String = String::from(&scan_p.pkgver);
                      // let s2: Vec<&str> = s1.split(':').collect();
                      
                      if ver == s1 {
                        let mut issued: String = "-".to_string();
                        let mut impact: String = "-".to_string();
                        let mut cveid:  String = "-".to_string();
                        let mut cvssv3_oval: String = "-".to_string();

                        let updated:  String = "-".to_string();
                        let cwe_oval: String = "-".to_string();
                        let cwe_name: String = "-".to_string();
                        let cwe_url_vec: Vec<String> = vec!["-".to_string(); 0];

                        if oval[x]["metadata"]["advisory"]["cve"] != Null {
                          if let Some(v) = oval[x]["metadata"]["advisory"]["cve"].as_array() {
                            for y in 0..v.len() {
                              if oval[x]["metadata"]["advisory"]["cve"][y]["@severity"] != Null {
                                let s1 = oval[x]["metadata"]["advisory"]["cve"][y]["@severity"].to_string().replace('"', "");
                                match &s1[..] {
                                  "critical" => {
                                    impact = "Critical".to_string();
                                  }
                                  "high" => {
                                    impact = "High".to_string();
                                  }
                                  "medium" => {
                                    impact = "Medium".to_string();
                                  }
                                  "low" => {
                                    impact = "Low".to_string();
                                  }
                                  "Critical" => {
                                    impact = "Critical".to_string();
                                  }
                                  "High" => {
                                    impact = "High".to_string();
                                  }
                                  "Medium" => {
                                    impact = "Medium".to_string();
                                  }
                                  "Low" => {
                                    impact = "Low".to_string();
                                  }
                                  _ => {
                                    impact = "-".to_string();
                                  }
                                }
                              }

                              if oval[x]["metadata"]["advisory"]["cve"][y]["$value"] != Null {
                                cveid = oval[x]["metadata"]["advisory"]["cve"][y]["$value"].to_string().replace('"', "");
                              }

                              if oval[x]["metadata"]["advisory"]["cve"][y]["cvss_score"] != Null {
                                cvssv3_oval = oval[x]["metadata"]["advisory"]["cve"][y]["cvss_score"].to_string().replace('"', "");
                              }

                              if oval[x]["metadata"]["advisory"]["cve"][y]["public"] != Null {
                                issued = oval[x]["metadata"]["advisory"]["cve"][y]["public"].to_string().replace('"', "");
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
      }
      else {
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

  Ok(())
}