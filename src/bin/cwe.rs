use time::{OffsetDateTime, macros::offset, format_description};
use serde::{Deserialize, Serialize};
use anyhow::Result;
use tokio::fs::File;
use tokio::io;
use zip::read::ZipArchive;
use std::fs::{File as fsFile};
use std::io::{Read, Write};
use std::path::Path;
use std::clone::Clone;
use quick_xml::de::from_str;


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
  let utc: OffsetDateTime = OffsetDateTime::now_utc();
  let jct: OffsetDateTime = utc.to_offset(offset!(+9));
  let format: Vec<format_description::FormatItem<'_>> = format_description::parse("[year][month][day]")?;
  let time: String = jct.format(&format)?;

  let dir = String::from("./src/cwe/");
  let dir_path = Path::new(&dir);

  if dir_path.is_dir() {
    println!("Remove dir... {:?}", dir);
    std::fs::remove_dir_all(&dir)?;
  }
  
  let response = reqwest::get("https://cwe.mitre.org/data/xml/cwec_latest.xml.zip").await?;
  let bytes = response.bytes().await?;

  std::fs::create_dir_all(&dir)?;
  let filename: String = String::from("cwe_") + &time + ".zip";
  let full_path: String = String::from(&dir) + &filename;

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

  let cwe_json: Cwe = from_str(&s)?;

  let cwe_dir = String::from("./src/cwe_result/");
  let cwe_dirpath = Path::new(&cwe_dir);

  if cwe_dirpath.is_dir() {
    println!("Remove dir... {:?}", cwe_dir);
    std::fs::remove_dir_all(&cwe_dir)?;
  }

  std::fs::create_dir_all(&cwe_dir)?;

  let cwe_file: String = String::from("cwe_") + &time + ".json";
  let cwe_fullpath: String = String::from(&cwe_dir) + &cwe_file;

  let serialized: String = serde_json::to_string(&cwe_json)?;
  let mut w = std::fs::OpenOptions::new()
    .write(true)
    .create(true)
    .open(cwe_fullpath)?;
  w.write_all(serialized.as_bytes())?;

  Ok(())
}