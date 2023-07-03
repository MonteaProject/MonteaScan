// 機能：サポートされていないOSをスキャンした際の動作

mod mod_scanner;
use crate::mod_scanner::rhel::main   as scanner_rhel;
use crate::mod_scanner::ubuntu::main as scanner_ubuntu;

use anyhow::{Result, anyhow};
use std::path::{Path,PathBuf};
use std::fs::{read_to_string, remove_dir_all, create_dir_all};
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct ScanServerList {
  server: Vec<Server>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Server {
  user : String,
  host : String,
  port : String,
  key  : String,
  os   : String,
}


fn main() -> Result<()> {
  let result_dir: String    = String::from("./src/scan_result/");
  let result_dirpath: &Path = Path::new(&result_dir);
  
  if result_dirpath.is_dir() {
    println!("Remove dir... {:?}", result_dir);
    remove_dir_all(&result_dir)?;
  }
  create_dir_all(&result_dir)?;

  let config_file: String = String::from("./src/config/config.json");
  let cnf: ScanServerList = {
    let cnf: String = read_to_string(&config_file)?;
    serde_json::from_str::<ScanServerList>(&cnf)?
  };

  for index in 0..cnf.server.len() {
    let user: &String = &cnf.server[index].user;
    let host: &String = &cnf.server[index].host;
    let port: &String = &cnf.server[index].port;
    let key:  &String = &cnf.server[index].key;
    let os:   &String = &cnf.server[index].os;
    let host_port: String = host.to_owned() + ":" + port;
    let prikey: PathBuf   = PathBuf::from(key);

    match &os[..] {
      "RedHat" => {
        if let Err(e) = scanner_rhel(user, prikey, host_port) {
          println!("{:#}", e);
        }
      },
      "AlmaLinux" => {
        if let Err(e) = scanner_rhel(user, prikey, host_port) {
          println!("{:#}", e);
        }
      },
      "RockyLinux" => {
        if let Err(e) = scanner_rhel(user, prikey, host_port) {
          println!("{:#}", e);
        }
      },
      "Ubuntu" => {
        if let Err(e) = scanner_ubuntu(user, prikey, host_port) {
          println!("{:#}", e);
        }
      }
      _ => return Err(anyhow!("OS Not Supported..."))
    };
  }

  Ok(())
}