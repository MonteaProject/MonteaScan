use crate::{Vulns, ScanResult, CweResult, Cwe};

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
  criteria: Option<Criteria>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct Metadata {
  title:       Option<String>,
  reference:   Option<Vec<Reference>>,
  description: Option<String>,
  advisory:    Option<Advisory>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct Reference {
  #[serde(rename = "@ref_id")]
  ref_id:  Option<String>,
  #[serde(rename = "@ref_url")]
  ref_url: Option<String>,
  #[serde(rename = "@source")]
  source:  Option<String>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct Advisory {
  #[serde(rename = "@from")]
  from:              Option<String>,
  severity:          Option<String>,
  rights:            Option<String>,
  issued:            Option<Issued>,
  updated:           Option<Updated>,
  cve:               Option<Vec<Cve>>,
  bugzilla:          Option<Vec<Bugzilla>>,
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
  #[serde(rename = "@cvss3")]
  cvss3:  Option<String>,
  #[serde(rename = "@cwe")]
  cwe:    Option<String>,
  #[serde(rename = "@href")]
  href:   Option<String>,
  #[serde(rename = "@impact")]
  impact: Option<String>,
  #[serde(rename = "@public")]
  public: Option<String>,
  #[serde(rename = "$value")]
  cve:    Option<String>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct Bugzilla {
  #[serde(rename = "@href")]
  href: Option<String>,
  #[serde(rename = "@id")]
  id:   Option<String>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct AffectedCpeList {
  cpe: Option<Vec<String>>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct Criteria {
  #[serde(rename = "@operator")]
  operator:  Option<String>,
  criterion: Option<Vec<Criterion>>,
  criteria:  Option<Vec<Criteria2>>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct Criterion {
  #[serde(rename = "@comment")]
  comment:  Option<String>,
  #[serde(rename = "@test_ref")]
  test_ref: Option<String>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct Criteria2 {
  #[serde(rename = "@operator")]
  operator:  Option<String>,
  criterion: Option<Vec<Criterion2>>,
  criteria:  Option<Vec<Criteria3>>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct Criterion2 {
  #[serde(rename = "@comment")]
  comment:  Option<String>,
  #[serde(rename = "@test_ref")]
  test_ref: Option<String>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct Criteria3 {
  #[serde(rename = "@operator")]
  operator: Option<String>,
  criteria: Option<Vec<Criteria4>>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct Criteria4 {
  #[serde(rename = "@operator")]
  operator:  Option<String>,
  criterion: Option<Vec<Criterion3>>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct Criterion3 {
  #[serde(rename = "@comment")]
  comment:  Option<String>,
  #[serde(rename = "@test_ref")]
  test_ref: Option<String>
}
//////////////////////////////////////////////////////////


pub async fn main(url: String, scan_r: ScanResult, f: String, result_dir: String) -> Result<Vec<CweResult>> {
  let mut vulns_vec: Vec<Vulns> = Vec::new();
  let mut detect_flag: usize    = 0;
  let mut cwe_vec: Vec<CweResult> = Vec::new();

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
          if let Some(sss) = s.criterion {
            for sss1 in sss {
              if let Some(sss2) = sss1.comment {
                comment_vec.push(sss2)
              }
            }
          }
          if let Some(s1) = s.criteria {
            for s2 in s1 {
              if let Some(ss) = s2.criterion {
                for ss1 in ss {
                  if let Some(ss2) = ss1.comment {
                    comment_vec.push(ss2);
                  }
                }
              }
              if let Some(s3) = s2.criteria {
                for s4 in s3 {
                  if let Some(s5) = s4.criteria {
                    for s6 in s5 {
                      if let Some(s7) = s6.criterion {
                        for s8 in s7 {
                          if let Some(s9) = s8.comment {
                            comment_vec.push(s9);
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

        for comment in &comment_vec {
          let c: Vec<&str> = comment.split("is earlier than").collect();

          if c.len() == 2 {
            let pkg: &str       = c[0].trim();
            let mut ver: String = c[1].trim().to_string();

            if pkg == scan_p.pkgname {
              let r1: Vec<&str> = ver.split(':').collect();
              if r1.len() == 2 {
                if r1[0] == "0" || r1[0] == "none" {
                  ver = r1[1].to_string();
                } else {
                  let r2: Vec<&str> = r1[1].split('-').collect();
                  if r2.len() == 2 {
                    let r3: &str = r1[0];
                    let r4: &str = r2[1];
                    ver = r3.to_string() + r4;
                  } else {
                    ver = r1[0].to_string();
                  }
                }
              }

              let mut p: String = String::from(&scan_p.pkgver);
              p += "-";
              p += &scan_p.pkgrelease;

              if ver == p {
                let cvssv3_oval:  String = "-".to_string();
                let mut issued:   String = "-".to_string();
                let mut updated:  String = "-".to_string();
                let mut impact:   String = "-".to_string();
                let mut cveid:    String = "-".to_string();
                let mut cwe_oval: String = "-".to_string();
                let mut cwe_name: String = "-".to_string();
                let mut cwe_url: Vec<String> = vec!["-".to_string(); 0];

                let cwe_read: String = String::from("./src/cwe/cwe.json");
                let cwe: Cwe = {
                  let cwe: String = std::fs::read_to_string(&cwe_read)?;
                  serde_json::from_str::<Cwe>(&cwe)?
                };

                if let Some(m) = v.metadata.clone() {
                  if let Some(m1) = m.advisory {
                    if let Some(m2) = m1.issued {
                      if let Some(m3) = m2.date {
                        issued = m3.replace('"', "");
                      }
                    }
                    if let Some(m4) = m1.updated {
                      if let Some(m5) = m4.date {
                        updated = m5.replace('"', "");
                      }
                    }
                    if let Some(m6) = m1.cve {
                      for m7 in m6 {
                        if let Some(m8) = m7.impact {
                          let s1 = m8.replace('"', "");
                          match &s1[..] {
                            "Critical" => {
                              impact = "Critical".to_string();
                            }
                            "Important" => {
                              impact = "High".to_string();
                            }
                            "Moderate" => {
                              impact = "Medium".to_string();
                            }
                            "Low" => {
                              impact = "Low".to_string();
                            }
                            "critical" => {
                              impact = "Critical".to_string();
                            }
                            "important" => {
                              impact = "High".to_string();
                            }
                            "moderate" => {
                              impact = "Medium".to_string();
                            }
                            "low" => {
                              impact = "Low".to_string();
                            }
                            _ => {
                              impact = "-".to_string();
                            }
                          }
                        }
                        if let Some(m9) = m7.cve {
                          cveid = m9.replace('"', "");
                        }
                        if let Some(m10) = m7.cwe {
                          let s = m10.replace('"', "");
                          let s5: Vec<&str> = s.split("->").collect();
                          if s5.len() > 1 {
                            let s6 = s5.len()-1;
                            cwe_oval = s5[s6].to_string();
                          } else {
                            cwe_oval = s;
                          }

                          let s1: &String   = &cwe_oval.replace("CWE-", "");
                          let s2: &String   = &s1.replace('(', "");
                          let s3: &String   = &s2.replace(')', "");
                          let s4: Vec<&str> = s3.split('|').collect();
                          for i in s4 {
                            let url = String::from("https://cwe.mitre.org/data/definitions/") + i + ".html";
                            cwe_url.push(url);
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

  Ok(cwe_vec)
}