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
  from:     Option<String>,
  severity: Option<String>,
  rights:   Option<String>,
  issued:   Option<Issued>,
  updated:  Option<Updated>,
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

    let cve_score  : String = "-".to_string();
    let oval_cwe   : String = "-".to_string();
    let href       : String = "-".to_string();
    let cve_impact : String = "-".to_string();
    let public     : String = "-".to_string();

    let cvss_score : String = "-".to_string();
    let vector     : String = "-".to_string();

    let mut from     : String = "-".to_string();
    let mut severity : String = "-".to_string();
    let mut rights   : String = "-".to_string();
    let mut oval_issued  : String = "-".to_string();
    let mut oval_updated : String = "-".to_string();

    let bugzi_href : String = "-".to_string();
    let id         : String = "-".to_string();
    let bugzi_des  : String = "-".to_string();

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
      href       : bugzi_href,
      id,
      description: bugzi_des,
    };
    let bugzilla : Vec<OvalBugzilla> = vec![oval_bugzi; 1];


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
                let cveid:        String = "-".to_string();
                let cvssv3_oval:  String = "-".to_string();
                let cwe_oval:     String = "-".to_string();
                let cwe_name:     String = "-".to_string();
                let cwe_url: Vec<String> = vec!["-".to_string(); 1];

                let mut issued : String = "-".to_string();
                let mut updated: String = "-".to_string();
                let mut impact : String = "-".to_string();

                if let Some(m) = v.metadata.clone() {
                  if let Some(r) = m.reference {
                    for r1 in r {
                      if let Some(r2) = r1.ref_id {
                        ref_id  = r2;
                      }
                      if let Some(r3) = r1.ref_url {
                        ref_url = r3;
                      }
                      if let Some(r4) = r1.source {
                        source  = r4;
                      }
                      let oval_ref: OvalReference = OvalReference {
                        ref_id : ref_id.clone(),
                        ref_url: ref_url.clone(),
                        source : source.clone(),
                      };
                      reference.push(oval_ref);
                    }
                  }
                  if let Some(r5) = m.affected {
                    if let Some(r6) = r5.family {
                      family = r6;
                    }
                    if let Some(r7) = r5.platform {
                      for r8 in r7 {
                        init_platform = r8;
                        platform.push(init_platform);
                      }
                    }
                  }
                  if let Some(r9) = m.description {
                    description = r9;
                  }
                  if let Some(r10) = m.title {
                    title = r10;
                  }
                  if let Some(m1) = m.advisory {
                    if let Some(r5) = m1.affected_cpe_list {
                      if let Some(r6) = r5.cpe {
                        for r7 in r6 {
                          init_cpe = r7;
                          cpe.push(init_cpe);
                        }
                      }
                    }
                    if let Some(r8) = m1.from {
                      from = r8;
                    }
                    if let Some(r9) = m1.rights {
                      rights = r9;
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
                    if let Some(m6) = m1.severity {
                      let m7 = m6.replace('"', "");
                      severity = m6.replace('"', "");
                      match &m7[..] {
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