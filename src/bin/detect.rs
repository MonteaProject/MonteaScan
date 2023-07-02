use crate::mod_detect::alma::main   as detect_alma;
use crate::mod_detect::rhel::main   as detect_rhel;
use crate::mod_detect::rocky::main  as detect_rocky;
use crate::mod_detect::ubuntu::main as detect_ubuntu;

use time::{OffsetDateTime, macros::offset, format_description};
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value, Value::Null};
use std::{path::PathBuf, fs::File};
use std::io::{BufReader, Write};
use std::path::Path;
use std::option::Option;


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
  pkgname:     String,
  pkgver:      String,
  pkgrelease:  String,
  update_flag: String,
  upver:       String,
  uprelease:   String,
  pkgarch:     String
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
#[allow(non_snake_case)]
struct Cwe {
  Weaknesses: Weaknesses
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
#[allow(non_snake_case)]
struct Weaknesses {
  Weakness: Vec<Weakness>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct Weakness {
  #[serde(rename = "@ID")]
  id: Option<String>,
  #[serde(rename = "@Name")]
  name: Option<String>
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
struct CweResult {
  time:     String,
  hostname: String,
  ip:       Vec<String>,
  os:       String,
  kernel:   String,
  cwe_id:   String,
  cwe_name: String,
}


#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
  println!("start...");

  let mut file_vec: Vec<String> = Vec::new();
  let mut cwe_vec: Vec<CweResult> = Vec::new();

  let scan_path: String = String::from("./src/scan_result/");
  let scan_dir: PathBuf = PathBuf::from(scan_path);
  let files: std::fs::ReadDir = scan_dir.read_dir().expect("code[387]: フォルダが存在しません.");

  // cwe
  let cwe_dir = String::from("./src/cwe_result/");
  let cwe_dirpath = Path::new(&cwe_dir);

  if cwe_dirpath.is_dir() {
    println!("Remove dir... {:?}", cwe_dir);
    std::fs::remove_dir_all(&cwe_dir).unwrap();
  }
  std::fs::create_dir_all(&cwe_dir).unwrap();

  // vulns
  let result_dir = String::from("./src/vulns_result/");
  let result_dirpath = Path::new(&result_dir);

  if result_dirpath.is_dir() {
    println!("Remove dir... {:?}", result_dir);
    std::fs::remove_dir_all(&result_dir).unwrap();
  }
  std::fs::create_dir_all(&result_dir).unwrap();

  for file in files {
    let f: String = file.iter().map(|x| x.path().to_string_lossy().into_owned()).collect::<String>();
    let ext: Vec<&str> = f.split('.').collect();
    let index: usize = ext.len() -1;

    if ext[index] == "json" {
      file_vec.push(f);
    }
  }

  for f in file_vec {
    println!("load file: {:?}", f);

    let file: File = match File::open(&f) {
      Ok(i) => i,
      Err(err) => panic!("File Open ERROR... {:?}", err),
    };

    let buf: BufReader<File> = BufReader::new(file);
    let scan_r: ScanResult = serde_json::from_reader(buf).unwrap();

    let release: &Vec<&str> = &scan_r.os.split_whitespace().collect::<Vec<_>>();

    // Ubuntu
    // Ubuntu 22.04.2 LTS
    if release[0] == "Ubuntu" {
      let s1: Vec<&str> = release[1].split('.').collect();
      let s2: String = s1[0].to_string() + s1[1];
      if s2 == "14.04" {
        let url: String = String::from("http://127.0.0.1:7878/trusty/");
        detect_ubuntu(url);
      }
      else if s2 == "16.04" {
        let url: String = String::from("http://127.0.0.1:7878/xenial/");
        detect_ubuntu(url);
      }
      else if s2 == "18.04" {
        let url: String = String::from("http://127.0.0.1:7878/bionic/");
        detect_ubuntu(url);
      }
      else if s2 == "20.04" {
        let url: String = String::from("http://127.0.0.1:7878/focal/");
        detect_ubuntu(url);
      }
      else if s2 == "22.04" {
        let url: String = String::from("http://127.0.0.1:7878/jammy/");
        detect_ubuntu(url);
      }
      else if s2 == "22.10" {
        let url: String = String::from("http://127.0.0.1:7878/kinetic/");
        detect_ubuntu(url);
      }
      else if s2 == "23.04" {
        let url: String = String::from("http://127.0.0.1:7878/lunar/");
        detect_ubuntu(url);
      }
      else {
        println!("未対応バージョン...");
      }
    }
    // AlmaLinux
    // AlmaLinux release 8.3 (Purple Manul)
    else if release[0] == "AlmaLinux" && release[1] == "release" {
      let m: Vec<&str> = release[2].split('.').collect();
      if m[0] == "8" {
        let url: String = String::from("http://127.0.0.1:7878/alma8/");
        detect_alma(url);
      }
      else if m[0] == "9" {
        let url: String = String::from("http://127.0.0.1:7878/alma9/");
        detect_alma(url);
      }
      else {
        println!("未対応バージョン...");
      }
    }
    // RedHat
    // Red Hat Enterprise Linux release 8.2 (Ootpa)
    else if release[0] == "Red" && release[1] == "Hat" && release[2] == "Enterprise"  && release[3] == "Linux"  && release[4] == "release" {
      let m: Vec<&str> = release[5].split('.').collect();
      if m[0] == "6" {
        let url: String = String::from("http://127.0.0.1:7878/rhel6/");
        detect_rhel(url);
      }
      else if m[0] == "7" {
        let url: String = String::from("http://127.0.0.1:7878/rhel7/");
        detect_rhel(url);
      }
      else if m[0] == "8" {
        let url: String = String::from("http://127.0.0.1:7878/rhel8/");
        detect_rhel(url);
      }
      else if m[0] == "9" {
        let url: String = String::from("http://127.0.0.1:7878/rhel9/");
        detect_rhel(url);
      }
      else {
        println!("未対応バージョン...");
      }
    }
    // RockyLinux
    // Rocky Linux release 9.1 (Blue Onyx)
    else if release[0] == "Rocky" && release[1] == "Linux" && release[2] == "release" {
      let m: Vec<&str> = release[3].split('.').collect();
      if m[0] == "8" {
        let url: String = String::from("http://127.0.0.1:7878/rocky8/");
        detect_rocky(url);
      }
      else if m[0] == "9" {
        let url: String = String::from("http://127.0.0.1:7878/rocky9/");
        detect_rocky(url);
      }
      else {
        println!("未対応バージョン...");
      }
    }
    else {
      println!("未対応OS...");
      continue;
    }
  }
  // CWE
  let utc: OffsetDateTime = OffsetDateTime::now_utc();
  let jct: OffsetDateTime = utc.to_offset(offset!(+9));
  let format: Vec<format_description::FormatItem<'_>> = format_description::parse("[year][month][day]").unwrap();
  let time_ymd: String = jct.format(&format).unwrap();

  let cwe_write: String = String::from("cwe_") + &time_ymd + ".json";
  let full_path: String = String::from(&cwe_dir) + &cwe_write;

  let serialized: String = serde_json::to_string(&cwe_vec).unwrap();
  let mut w: std::fs::File = std::fs::OpenOptions::new()
    .write(true)
    .create(true)
    .open(full_path).unwrap();
  w.write_all(serialized.as_bytes()).expect("Failed to Write cwe_result...");

  println!("finished...");

  Ok(())
}