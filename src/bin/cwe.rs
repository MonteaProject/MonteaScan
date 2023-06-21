use time::{OffsetDateTime, macros::offset, format_description};
use anyhow::Result;
use tokio::fs::File;
use tokio::io;
use zip::read::ZipArchive;
use std::fs::{File as fsFile};
use std::io::Read;
use std::path::Path;


#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
  let utc: OffsetDateTime = OffsetDateTime::now_utc();
  let jct: OffsetDateTime = utc.to_offset(offset!(+9));
  let format: Vec<format_description::FormatItem<'_>> = format_description::parse("[year][month][day]").unwrap();
  let time: String = jct.format(&format)?;

  let dir = String::from("./src/cwe/");
  let dir_path = Path::new(&dir);

  if dir_path.is_dir() {
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
      // let _ = std::io::copy(&mut file, &mut std::io::stdout());
      file.read_to_string(&mut s)?;
  }

  // println!("{:?}", s);

  Ok(())
}