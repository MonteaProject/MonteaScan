use crate::{Vulns, ScanResult};

use anyhow::Result;
use time::{OffsetDateTime, macros::offset, format_description};
use serde::{Deserialize, Serialize};
use serde_json::{Value, Value::Null};
use std::fs::File;
use std::io::Write;


//////////////////////////////////////////////////////////
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct OvalDB {
  #[serde(rename = "@id")]
  id:       Option<String>,
  #[serde(rename = "@class")]
  class:    Option<String>,
  metadata: Option<Metadata>,
  criteria: Option<Criteria>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct Metadata {
  title:       Option<String>,
  affected:    Option<Affected>,
  reference:   Option<Vec<Reference>>,
  description: Option<String>,
  advisory:    Option<Advisory>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct Affected {
  #[serde(rename = "@family")]
  family:   Option<String>,
  platform: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct Reference {
  #[serde(rename = "@ref_id")]
  ref_id:  Option<String>,
  #[serde(rename = "@ref_url")]
  ref_url: Option<String>,
  #[serde(rename = "@source")]
  source:  Option<String>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct Advisory {
  rights:          Option<String>,
  component:       Option<String>,
  current_version: Option<String>,
  cve:             Option<Vec<Cve>>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct Cve {
  #[serde(rename = "@cvss_score")]
  cvss_score:  Option<String>,
  #[serde(rename = "@cvss_vector")]
  cvss_vector: Option<String>,
  #[serde(rename = "@href")]
  href:        Option<String>,
  #[serde(rename = "@severity")]
  severity:    Option<String>,
  #[serde(rename = "@public")]
  public:      Option<String>,
  #[serde(rename = "@usns")]
  usns:        Option<String>,
  #[serde(rename = "$value")]
  cve:         Option<String>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct Criteria {
  extend_definition: Option<Extend>,
  criteria:          Option<Criteria2>,
  
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct Extend {
  #[serde(rename = "@definition_ref")]
  definition_ref:      Option<String>,
  #[serde(rename = "@comment")]
  comment:             Option<String>,
  #[serde(rename = "@applicability_check")]
  applicability_check: Option<String>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct Criteria2 {
  #[serde(rename = "@operator")]
  operator:  Option<String>,
  criterion: Option<Vec<Criterion>>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct Criterion {
  #[serde(rename = "@comment")]
  comment:  Option<String>,
  #[serde(rename = "@test_ref")]
  test_ref: Option<String>
}
//////////////////////////////////////////////////////////


pub async fn main(url: String, scan_r: ScanResult, f: String, result_dir: String) -> Result<()> {
  let mut vulns_vec: Vec<Vulns> = Vec::new();
  let mut detect_flag: usize    = 0;

  let response: reqwest::Response  = reqwest::get(&url).await?;
  let bytes: actix_web::web::Bytes = response.bytes().await?;
  let data: String = String::from_utf8(bytes.to_vec())?;

  let s: Vec<Vec<OvalDB>> = serde_json::from_str(&data)?;

  for scan_p in &scan_r.pkg {
    let utc: OffsetDateTime = OffsetDateTime::now_utc();
    let jct: OffsetDateTime = utc.to_offset(offset!(+9));
    let format: Vec<format_description::FormatItem<'_>> = format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]")?;

    let time: String = jct.format(&format)?;
    let hostname: String = String::from(&scan_r.hostname).replace('\n', "");
    let ip: &Vec<String> = &scan_r.ip;
    let os: String = String::from(&scan_r.os).replace('\n', "");
    let kernel: String = String::from(&scan_r.kernel).replace('\n', "");

    for v1 in s.clone() {
      for v in v1 {
        let mut comment_vec: Vec<String> = Vec::new();

        if let Some(s) = v.criteria {
          if let Some(s1) = s.criteria {
            if let Some(s2) = s1.criterion {
              for s3 in s2 {
                if let Some(s4) = s3.comment {
                  comment_vec.push(s4);
                }
              }
            }
          }
        }

        for comment in &comment_vec {
          // "(CVE-2014-2686) ansible package in bionic, is related to the CVE in some way and has been fixed (note: '2.5.1+dfsg-1ubuntu0.1')."
          let c: Vec<&str> = comment.split("package in").collect();
          if c.len() == 2 {
            // "(CVE-2014-2686) ansible
            let s1: Vec<&str> = c[0].split_whitespace().collect::<Vec<&str>>();
            let pkg: &str     = s1[1].trim();

            if pkg == scan_p.pkgname {
              let s1: String = String::from(&scan_p.pkgver) + &scan_p.pkgrelease;

              // bionic, is related to the CVE in some way and has been fixed (note: '2.5.1+dfsg-1ubuntu0.1')."
              let s2: Vec<&str> = c[1].trim().split("note:").collect();
              if s2.len() == 2 {
                // '2.5.1+dfsg-1ubuntu0.1')."
                let s3: Vec<&str> = s2[1].trim().split(')').collect();
                if s3.len() == 2 {
                  // '2.5.1+dfsg-1ubuntu0.1'
                  let mut ver: String = s3[0].trim().replace('\'', "");

                  let r1: Vec<&str> = ver.split(':').collect();
                  if r1.len() == 2 {
                    if r1[0] == "0" || r1[0] == "none" {
                      // r1[0]: 0, r1[1]: 2.5.1-1ubuntu0.1
                      ver = r1[1].to_string();
                    } else {
                      // r1[0]: 1, r1[1]: 2.5.1-1ubuntu0.1
                      let r2: Vec<&str> = r1[1].split('-').collect();
                      if r2.len() == 2 {
                        // r2[0]: 2.5.1, r2[1]: 1ubuntu0.1
                        let r3: &str = r1[0]; // [epoch:]
                        let r4: &str = r2[1]; // [-debian-revision]
                        ver = r3.to_string() + r4;
                      } else {
                        // r1[0]: 1, r1[1]: 2.5.1
                        ver = r1[0].to_string();
                      }
                    }
                  }
              
                  if ver == s1 {
                    let mut issued: String = "-".to_string();
                    let mut impact: String = "-".to_string();
                    let mut cveid:  String = "-".to_string();
                    let updated:    String = "-".to_string();
                    let cwe_oval:   String = "-".to_string();
                    let cwe_name:   String = "-".to_string();
                    let cwe_url: Vec<String> = vec!["-".to_string(); 0];
                    let mut cvssv3_oval: String = "-".to_string();

                    if let Some(m) = v.metadata.clone() {
                      if let Some(m1) = m.advisory {
                        if let Some(m2) = m1.cve {
                          for m3 in m2 {
                            if let Some(m4) = m3.severity {
                              let s1 = m4.replace('"', "");
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
                            if let Some(m5) = m3.cve {
                              cveid = m5.replace('"', "");
                            }
                            if let Some(m6) = m3.cvss_score {
                              cvssv3_oval = m6.replace('"', "");
                            }
                            if let Some(m7) = m3.public {
                              issued = m7.replace('"', "");
                            }

                            let vulns_list: Vulns = Vulns {
                              time     : time.clone(),
                              hostname : hostname.clone(),
                              ip       : ip.clone(),
                              os       : os.clone(),
                              kernel   : kernel.clone(),
                              cveid       : cveid.clone(),
                              impact      : impact.clone(),
                              cvssv3_oval : cvssv3_oval.clone(),
                              cwe_oval    : cwe_oval.clone(),
                              issued      : issued.clone(),
                              updated     : updated.clone(),
                              pkgname     : scan_p.pkgname.clone(),
                              pkgver      : scan_p.pkgver.clone(),
                              pkgrelease  : scan_p.pkgrelease.clone(),
                              update_flag : scan_p.update_flag.clone(),
                              upver       : scan_p.upver.clone(),
                              uprelease   : scan_p.uprelease.clone(),
                              pkgarch     : scan_p.pkgarch.clone(),
                              cwe_name : cwe_name.clone(),
                              cwe_url  : cwe_url.clone(),
                              // oval : v.clone(),
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
    }
    if vulns_vec.len() == detect_flag {
      let issued:       String = "-".to_string();
      let updated:      String = "-".to_string();
      let impact:       String = "-".to_string();
      let cveid:        String = "-".to_string();
      let cwe_oval:     String = "-".to_string();
      let cwe_name:     String = "-".to_string();
      let cwe_url: Vec<String> = vec!["-".to_string(); 0];
      let cvssv3_oval:  String = "-".to_string();
      
      let vulns_list: Vulns = Vulns {
        time     : time.clone(),
        hostname : hostname.clone(),
        ip       : ip.clone(),
        os       : os.clone(),
        kernel   : kernel.clone(),
        cveid       : cveid.clone(),
        impact      : impact.clone(),
        cvssv3_oval : cvssv3_oval.clone(),
        cwe_oval    : cwe_oval.clone(),
        issued      : issued.clone(),
        updated     : updated.clone(),
        pkgname     : scan_p.pkgname.clone(),
        pkgver      : scan_p.pkgver.clone(),
        pkgrelease  : scan_p.pkgrelease.clone(),
        update_flag : scan_p.update_flag.clone(),
        upver       : scan_p.upver.clone(),
        uprelease   : scan_p.uprelease.clone(),
        pkgarch     : scan_p.pkgarch.clone(),
        cwe_name    : cwe_name.clone(),
        cwe_url     : cwe_url.clone(),
        // oval : Null
      };
      vulns_vec.push(vulns_list);
    } else {
      detect_flag = vulns_vec.len();
    }
  }

  let d_file: Vec<&str> = f.split('/').collect();
  let d_index: usize = d_file.len()-1;

  let filename: String = String::from(d_file[d_index]);
  let full_path: String = String::from(&result_dir) + &filename;

  let serialized: String = serde_json::to_string(&vulns_vec)?;
  let mut w: File = std::fs::OpenOptions::new()
    .write(true)
    .create(true)
    .open(full_path)?;
  w.write_all(serialized.as_bytes())?;

  println!("finished: {:?}", f);

  Ok(())
}