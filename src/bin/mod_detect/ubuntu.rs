use crate::{Vulns, Oval, Reference as OvalReference, Cve as OvalCve, Cvss as OvalCvss, Advisory as OvalAdvisory, Bugzilla as OvalBugzilla, ScanResult};

use anyhow::Result;
use time::{OffsetDateTime, macros::offset, format_description};
use serde::{Deserialize, Serialize};
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

    let mut ref_id  : String = "-".to_string();
    let mut ref_url : String = "-".to_string();
    let mut source  : String = "-".to_string();

    let mut cve_score  : String = "-".to_string();
    let oval_cwe   : String = "-".to_string();
    let mut href       : String = "-".to_string();
    let mut cve_impact : String = "-".to_string();
    let mut public     : String = "-".to_string();

    let mut cvss_score : String = "-".to_string();
    let mut vector     : String = "-".to_string();

    let from             : String = "-".to_string();
    let severity         : String = "-".to_string();
    let mut rights       : String = "-".to_string();
    let mut oval_issued  : String = "-".to_string();
    let oval_updated     : String = "-".to_string();

    let mut bugzi_href : String = "-".to_string();
    let mut id         : String = "-".to_string();
    let bugzi_des  : String = "-".to_string();

    let mut init_platform : String = "-".to_string();
    let init_cpe          : String = "-".to_string();

    let mut title    : String = "-".to_string();
    let mut family   : String = "-".to_string();
    
    let mut description : String = "-".to_string();
    
    let mut platform : Vec<String> = vec![init_platform; 0];
    let cpe          : Vec<String> = vec![init_cpe; 0];

    let oval_ref: OvalReference = OvalReference {
      ref_id : ref_id.clone(),
      ref_url: ref_url.clone(),
      source : source.clone(),
    };
    let mut reference : Vec<OvalReference> = vec![oval_ref; 0];

    let oval_bugzi: OvalBugzilla = OvalBugzilla {
      href       : bugzi_href.clone(),
      id         : id.clone(),
      description: bugzi_des.clone(),
    };
    let mut bugzilla : Vec<OvalBugzilla> = vec![oval_bugzi; 0];


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
                    let cwe_url: Vec<String> = vec!["-".to_string(); 1];
                    let mut cvssv3_oval: String = "-".to_string();

                    if let Some(m) = v.metadata.clone() {
                      if let Some(r) = m.reference {
                        for r1 in r {
                          if let Some(v) = r1.ref_id {
                            ref_id  = v;
                          }
                          if let Some(v) = r1.ref_url {
                            ref_url = v;
                          }
                          if let Some(v) = r1.source {
                            source  = v;
                          }
                          let oval_ref: OvalReference = OvalReference {
                            ref_id : ref_id.clone(),
                            ref_url: ref_url.clone(),
                            source : source.clone(),
                          };
                          reference.push(oval_ref);
                        }
                      }
                      if let Some(r2) = m.title {
                        title = r2.replace('"', "");
                      }
                      if let Some(r3) = m.affected {
                        if let Some(r4) = r3.family {
                          family = r4.replace('"', "");
                        }
                        if let Some(r5) = r3.platform {
                          for r6 in r5 {
                            init_platform = r6;
                            platform.push(init_platform);
                          }
                        }
                      }
                      if let Some(r7) = m.description {
                        description = r7;
                      }
                      if let Some(m1) = m.advisory {
                        if let Some(r) = m1.rights {
                          rights = r.replace('"', "");
                        }
                        if let Some(m2) = m1.cve {
                          for m3 in m2 {
                            if let Some(m4) = m3.severity {
                              let s1 = m4.replace('"', "");
                              cve_impact     = m4.replace('"', "");
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
                              cveid     = m5.replace('"', "");
                              cve_score = m5.replace('"', "");
                            }
                            if let Some(m6) = m3.cvss_score {
                              cvssv3_oval = m6.replace('"', "");
                              cvss_score  = m6.replace('"', "");
                            }
                            if let Some(m7) = m3.public {
                              let test = m7.replace('"', "");
                              let mut s1 = String::new();
                              for i in test.chars().enumerate() {
                                  match i.0 {
                                      // n if n < 1 => {continue}
                                      n if n > 3 => {break}
                                      _ => {s1.push(i.1)}
                                  }
                              }
                              let mut s2 = String::new();
                              for i in test.chars().enumerate() {
                                  match i.0 {
                                      n if n < 4 => {continue}
                                      n if n > 5 => {break}
                                      _ => {s2.push(i.1)}
                                  }
                              }
                              let mut s3 = String::new();
                              for i in test.chars().enumerate() {
                                  match i.0 {
                                      n if n < 6 => {continue}
                                      n if n > 8 => {break}
                                      _ => {s3.push(i.1)}
                                  }
                              }
                              issued = s1.clone() + "-" + &s2 + "-" + &s3;
                              oval_issued = s1.clone() + "-" + &s2 + "-" + &s3;
                              public = m7.replace('"', "");
                            }
                            if let Some(m8) = m3.cvss_vector {
                              // "5.9",
                              // "CVSS:3.0/AV:N/AC:H/PR:N/UI:N/S:U/C:N/I:N/A:H", -> 5.9/CVSS:3.0/AV:N/AC:H/PR:N/UI:N/S:U/C:N/I:N/A:H
                              let cvss: &str = &m8.replace('"', "");
                              vector = cvss_score.clone() + "/" + cvss;
                            }
                            if let Some(m9) = m3.href {
                              href = m9.replace('"', "");
                            }

                            if let Some(m10) = m3.usns {
                              let m11 = m10.replace('"', "");
                              let s5: Vec<&str> = m11.split(',').collect();
                              for i in s5 {
                                id = "USN-".to_string() + i;
                                bugzi_href = String::from("https://ubuntu.com/security/notices/USN-") + i + ".html";
                                // bugzi_des = "-".to_string();
                                let oval_bugzi: OvalBugzilla = OvalBugzilla {
                                  href       : bugzi_href.clone(),
                                  id         : id.clone(),
                                  description: bugzi_des.clone(),
                                };
                                bugzilla.push(oval_bugzi);
                              }
                            }

                            let cve : OvalCve = OvalCve {
                              score : cve_score.clone(),
                              cwe   : oval_cwe.clone(),
                              href  : href.clone(),
                              impact: cve_impact.clone(),
                              public: public.clone(),
                            };
                            let cvss : OvalCvss = OvalCvss {
                              score : cvss_score.clone(),
                              vector: vector.clone(),
                            };
                            let advisory : OvalAdvisory = OvalAdvisory {
                              from    : from.clone(),
                              severity: severity.clone(),
                              rights  : rights.clone(),
                              issued  : oval_issued.clone(),
                              updated : oval_updated.clone(),
                            };
                            let oval: Oval = Oval {
                              title:       title.clone(),
                              family:      family.clone(),
                              platform:    platform.clone(),
                              description: description.clone(),
                              reference:   reference.clone(),
                              cpe:         cpe.clone(),
                              cve:         cve.clone(),
                              cvss:        cvss.clone(),
                              advisory:    advisory.clone(),
                              bugzilla:    bugzilla.clone()
                            };

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
                              oval     : oval.clone(),
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

      let cve : OvalCve = OvalCve {
        score : cve_score,
        cwe   : oval_cwe,
        href,
        impact: cve_impact,
        public,
      };
      let cvss : OvalCvss = OvalCvss {
        score : cvss_score,
        vector,
      };
      let advisory : OvalAdvisory = OvalAdvisory {
        from,
        severity,
        rights,
        issued  : oval_issued,
        updated : oval_updated,
      };
      let oval: Oval = Oval {
        title      : title.clone(),
        family     : family.clone(),
        platform   : platform.clone(),
        description: description.clone(),
        reference  : reference.clone(),
        cpe        : cpe.clone(),
        cve        : cve.clone(),
        cvss       : cvss.clone(),
        advisory   : advisory.clone(),
        bugzilla   : bugzilla.clone()
      };
      
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
        oval,
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