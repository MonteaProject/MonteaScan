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

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct RhelDefinition {
  #[serde(rename = "@id")]
  id:       Option<String>,
  #[serde(rename = "@class")]
  class:    Option<String>,
  metadata: Option<RhelMetadata>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct RhelMetadata {
  title:       Option<String>,
  affected:    Option<RhelAffected>,
  reference:   Option<Vec<RhelReference>>,
  description: Option<String>,
  advisory:    Option<RhelAdvisory>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct RhelAffected {
  #[serde(rename = "@family")]
  family:   Option<String>,
  platform: Option<Vec<String>>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct RhelReference {
  #[serde(rename = "@ref_id")]
  ref_id:  Option<String>,
  #[serde(rename = "@ref_url")]
  ref_url: Option<String>,
  #[serde(rename = "@source")]
  source:  Option<String>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct RhelAdvisory {
  #[serde(rename = "@from")]
  from:              Option<String>,
  severity:          Option<String>,
  rights:            Option<String>,
  issued:            Option<RhelIssued>,
  updated:           Option<RhelUpdated>,
  cve:               Option<Vec<RhelCve>>,
  bugzilla:          Option<Vec<RhelBugzilla>>,
  affected_cpe_list: Option<RhelAffectedCpeList>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct RhelIssued {
  #[serde(rename = "@date")]
  date: Option<String>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct RhelUpdated {
  #[serde(rename = "@date")]
  date: Option<String>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct RhelCve {
  #[serde(rename = "@cvss2")]
  cvss2:  Option<String>,
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
struct RhelBugzilla {
  #[serde(rename = "@href")]
  href:     Option<String>,
  #[serde(rename = "@id")]
  id:       Option<String>,
  #[serde(rename = "$value")]
  bugzilla: Option<String>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct RhelAffectedCpeList {
  cpe: Option<Vec<String>>
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
        if let Some(v) = oval[0]["criteria"]["criteria"].as_array() {
          for y in 0..v.len() {
            if let Some(v) = oval[0]["criteria"]["criteria"][y]["criterion"].as_array() {
              let mut comment_vec: Vec<&str> = Vec::new();

              for z in 0..v.len() {
                if let Some(v) = oval[0]["criteria"]["criteria"][y]["criterion"][z]["@comment"].as_str() {
                  comment_vec.push(v);
                }
              }

              let count: usize = comment_vec.len();
              if count == 3 {
                let c: Vec<&str> = comment_vec[1].split("is earlier than").collect();

                if c.len() == 2 {
                  let pkg: &str = c[0].trim();
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
                      let mut issued: String = "-".to_string();
                      if oval[0]["metadata"]["advisory"]["issued"]["@date"] != Null {
                        issued = oval[0]["metadata"]["advisory"]["issued"]["@date"].to_string().replace('"', "");
                      }

                      let mut updated: String = "-".to_string();
                      if oval[0]["metadata"]["advisory"]["updated"]["@date"] != Null {
                        updated = oval[0]["metadata"]["advisory"]["updated"]["@date"].to_string().replace('"', "");
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

                      if oval[0]["metadata"]["advisory"]["cve"] != Null {
                        if let Some(v) = oval[0]["metadata"]["advisory"]["cve"].as_array() {
                          for y in 0..v.len() {
                            if oval[0]["metadata"]["advisory"]["cve"][y]["@impact"] != Null {
                              let s1 = oval[0]["metadata"]["advisory"]["cve"][y]["@impact"].to_string().replace('"', "");
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
        
                            if oval[0]["metadata"]["advisory"]["cve"][y]["$value"] != Null {
                              cveid = oval[0]["metadata"]["advisory"]["cve"][y]["$value"].to_string().replace('"', "");
                            }
        
                            if oval[0]["metadata"]["advisory"]["cve"][y]["@cvss3"] != Null {
                              let s1 = oval[0]["metadata"]["advisory"]["cve"][y]["@cvss3"].to_string().replace('"', "");
                              let s2: Vec<&str> = s1.split('/').collect();
                              cvssv3_oval = s2[0].to_string();
                            }
        
                            if oval[0]["metadata"]["advisory"]["cve"][y]["@cwe"] != Null {
                              let s = oval[0]["metadata"]["advisory"]["cve"][y]["@cwe"].to_string().replace('"', "");

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

                            let rhel_oval: Vec<RhelDefinition> = serde_json::from_value(oval.clone())?;
                            let rhel_value: Value = serde_json::to_value(rhel_oval)?;
                            
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
                              detect:      rhel_value.clone()
                            };
                            vulns_vec.push(vulns_list);
                          }
                        }
                      } else {
                        let rhel_oval: Vec<RhelDefinition> = serde_json::from_value(oval.clone())?;
                        let rhel_value: Value = serde_json::to_value(rhel_oval)?;

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
                          detect:      rhel_value.clone()
                        };
                        vulns_vec.push(vulns_list);
                      }
                      break;
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