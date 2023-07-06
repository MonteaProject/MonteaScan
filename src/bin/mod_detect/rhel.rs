use crate::{Vulns, ScanResult, CweResult, Cwe, Oval, Reference as OvalReference, Cve as OvalCve, Cvss as OvalCvss, Advisory as OvalAdvisory, Bugzilla as OvalBugzilla};

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
  criteria: Option<Criteria>
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
  platform: Option<Vec<String>>
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
struct Bugzilla {
  #[serde(rename = "@href")]
  href:     Option<String>,
  #[serde(rename = "@id")]
  id:       Option<String>,
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
  criterion: Option<Vec<Criterion2>>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct Criterion2 {
  #[serde(rename = "@comment")]
  comment:  Option<String>,
  #[serde(rename = "@test_ref")]
  test_ref: Option<String>
}
//////////////////////////////////////////////////////////


pub async fn main(url: String, scan_r: ScanResult, f: String, result_dir: String) -> Result<Vec<CweResult>>  {
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

    let mut ref_id  : String = "-".to_string();
    let mut ref_url : String = "-".to_string();
    let mut source  : String = "-".to_string();

    let mut cve_score  : String = "-".to_string();
    let mut oval_cwe   : String = "-".to_string();
    let mut href       : String = "-".to_string();
    let mut cve_impact : String = "-".to_string();
    let mut public     : String = "-".to_string();

    let mut cvss_score : String = "-".to_string();
    let mut vector     : String = "-".to_string();

    let mut from     : String = "-".to_string();
    let mut severity : String = "-".to_string();
    let mut rights   : String = "-".to_string();
    let mut oval_issued  : String = "-".to_string();
    let mut oval_updated : String = "-".to_string();

    let mut bugzi_href : String = "-".to_string();
    let mut id         : String = "-".to_string();
    let mut bugzi_des  : String = "-".to_string();

    let mut init_platform : String = "-".to_string();
    let mut init_cpe      : String = "-".to_string();

    let mut title    : String = "-".to_string();
    let mut family   : String = "-".to_string();
    
    let mut description : String = "-".to_string();
    
    let mut platform : Vec<String> = vec![init_platform; 0];
    let mut cpe      : Vec<String> = vec![init_cpe; 0];

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
            for s2 in s1 {
              if let Some(s3) = s2.criterion {
                for s4 in s3 {
                  if let Some(s5) = s4.comment {
                    comment_vec.push(s5);
                  }
                }
              }
            }
          }
          if let Some(ss) = s.criterion {
            for ss1 in ss {
              if let Some(ss2) = ss1.comment {
                comment_vec.push(ss2);
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
                let mut issued:   String = "-".to_string();
                let mut updated:  String = "-".to_string();
                let mut impact:   String = "-".to_string();
                let mut cveid:    String = "-".to_string();
                let mut cwe_oval: String = "-".to_string();
                let mut cwe_name: String = "-".to_string();
                let mut cwe_url: Vec<String> = vec!["-".to_string(); 0];
                let mut cvssv3_oval: String = "-".to_string();

                let cwe_read: String = String::from("./src/cwe/cwe.json");
                let cwe: Cwe = {
                  let cwe: String = std::fs::read_to_string(&cwe_read)?;
                  serde_json::from_str::<Cwe>(&cwe)?
                };
                
                if let Some(m) = v.metadata.clone() {
                  if let Some(r) = m.title {
                    title = r;
                  }
                  if let Some(r) = m.description {
                    description = r;
                  }
                  if let Some(r) = m.reference {
                    for r1 in r {
                      if let Some(r2) = r1.ref_id {
                        ref_id = r2;
                      }
                      if let Some(r3) = r1.ref_url {
                        ref_url = r3;
                      }
                      if let Some(r4) = r1.source {
                        source = r4;
                      }
                      let oval_ref: OvalReference = OvalReference {
                        ref_id : ref_id.clone(),
                        ref_url: ref_url.clone(),
                        source : source.clone(),
                      };
                      reference.push(oval_ref);
                    }
                  }
                  if let Some(r) = m.affected {
                    if let Some(r1) = r.family {
                      family = r1;
                    }
                    if let Some(r2) = r.platform {
                      for r3 in r2 {
                        init_platform = r3;
                        platform.push(init_platform);
                      }
                    }
                  }
                  if let Some(m1) = m.advisory {
                    if let Some(r) = m1.from {
                      from = r;
                    }
                    if let Some(r1) = m1.severity {
                      severity = r1;
                    }
                    if let Some(r2) = m1.rights {
                      rights = r2;
                    }
                    if let Some(r3) = m1.bugzilla {
                      for r4 in r3 {
                        if let Some(r5) = r4.bugzilla {
                          bugzi_des = r5;
                        }
                        if let Some(r6) = r4.href {
                          bugzi_href = r6;
                        }
                        if let Some(r7) = r4.id {
                          id = r7;
                        }
                        let oval_bugzi: OvalBugzilla = OvalBugzilla {
                          href       : bugzi_href.clone(),
                          id         : id.clone(),
                          description: bugzi_des.clone(),
                        };
                        bugzilla.push(oval_bugzi);
                      }
                    }
                    if let Some(r8) = m1.affected_cpe_list {
                      if let Some(r9) = r8.cpe {
                        for r10 in r9 {
                          init_cpe = r10;
                          cpe.push(init_cpe);
                        }
                      }
                    }
                    if let Some(m2) = m1.issued {
                      if let Some(m3) = m2.date {
                        issued = m3.replace('"', "");
                        oval_issued = m3.replace('"', "");
                      }
                    }
                    if let Some(m4) = m1.updated {
                      if let Some(m5) = m4.date {
                        updated = m5.replace('"', "");
                        oval_updated = m5.replace('"', "");
                      }
                    }
                    if let Some(m6) = m1.cve {
                      for m7 in m6 {
                        if let Some(r) = m7.href {
                          href = r;
                        }
                        if let Some(r1) = m7.public {
                          public = r1;
                        }
                        if let Some(m8) = m7.impact {
                          let s1 = m8.replace('"', "");
                          cve_impact = m8.replace('"', "");
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
                        if let Some(m8) = m7.cve {
                          cveid = m8.replace('"', "");
                          cve_score = m8.replace('"', "");
                        }
                        if let Some(m9) = m7.cvss3 {
                          let s1 = m9.replace('"', "");
                          vector      = s1.to_string();
                          let s2: Vec<&str> = s1.split('/').collect();
                          cvssv3_oval = s2[0].to_string();
                          cvss_score  = s2[0].to_string();
                        }
                        if let Some(m10) = m7.cwe {
                          let s = m10.replace('"', "");
                          let s5: Vec<&str> = s.split("->").collect();
                          if s5.len() > 1 {
                            let s6 = s5.len()-1;
                            cwe_oval = s5[s6].to_string();
                            oval_cwe = s5[s6].to_string();
                          } else {
                            cwe_oval = s.clone();
                            oval_cwe = s.clone();
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
                          oval : oval.clone(),
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

  Ok(cwe_vec)
}