use anyhow::Result;
use tokio::fs::File;
use tokio::io;
use std::fs::{File as fsFile};
use std::io::{Read, Write};
use std::path::Path;
use std::clone::Clone;
use serde::{Deserialize, Serialize};
use quick_xml::de::from_str;
use zip::read::ZipArchive;


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

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
  let dir = String::from("./src/cwe/");
  let dir_path = Path::new(&dir);

  if dir_path.is_dir() {
    println!("Remove dir... {:?}", dir);
    std::fs::remove_dir_all(&dir)?;
  }
  
  let response = reqwest::get("https://cwe.mitre.org/data/xml/cwec_latest.xml.zip").await?;
  let bytes = response.bytes().await?;

  std::fs::create_dir_all(&dir)?;
  let cwe_zip: String = String::from("cwe") + ".zip";
  let full_path: String = String::from(&dir) + &cwe_zip;

  let mut file = File::create(&full_path).await?;

  io::copy(&mut bytes.as_ref(), &mut file).await?;

  let archive = fsFile::open(&full_path)?;
  let mut zip = ZipArchive::new(archive)?;

  let mut s = String::new();

  for i in 0..zip.len()
  {
    let mut file = zip.by_index(i)?;
    file.read_to_string(&mut s)?;
  }

  let data: Cwe = from_str(&s)?;

  let cwe_json: String = String::from("cwe") + ".json";
  let cwe_fullpath: String = String::from(&dir) + &cwe_json;

  let serialized: String = serde_json::to_string(&data)?;
  let mut w = std::fs::OpenOptions::new()
    .write(true)
    .create(true)
    .open(cwe_fullpath)?;
  w.write_all(serialized.as_bytes())?;

  Ok(())
}