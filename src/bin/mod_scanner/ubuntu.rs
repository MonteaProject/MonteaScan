use time::{OffsetDateTime, macros::offset, format_description};
use ssh2::Session;
use serde_derive::{Deserialize, Serialize};
use serde_json::{Result};
use std::io::{Read, Write};
use std::path::PathBuf;

// #[derive(Deserialize, Serialize, Debug)]
// struct ScanServerList {
//   server: Vec<Server>,
// }

// #[derive(Deserialize, Serialize, Debug)]
// struct Server {
//   user : String,
//   host : String,
//   port : String,
//   key  : String
// }

#[derive(Deserialize, Serialize, Debug)]
struct ScanResult {
  time    : String,
  hostname: String,
  ip      : Vec<String>,
  os      : UbuntuVersion,
  kernel  : String,
  pkg     : Vec<PkgList>
}

#[derive(Deserialize, Serialize, Debug)]
struct PkgList {
  pkgname    : String,
  pkgver     : String,
  update_flag: String,
  upver      : String,
  pkgarch    : String
}

#[derive(Deserialize, Serialize, Debug)]
struct Update {
  update: Vec<UpdateList>
}

#[derive(Deserialize, Serialize, Debug)]
struct UpdateList {
  name: String,
  ver : String,
  arch: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct UbuntuVersion {
  distributor: String,
  description: String,
  release:     String,
  codename:    String,
}


pub fn main(user: &str, prikey: PathBuf, host_port: String) -> Result<()> {
  // let result_dir = String::from("./src/scan_result/");
  // let result_dirpath = Path::new(&result_dir);

  // if result_dirpath.is_dir() {
  //   println!("Remove dir... {:?}", result_dir);
  //   std::fs::remove_dir_all(&result_dir).unwrap();
  // }

  // std::fs::create_dir_all(&result_dir).unwrap();

  // let config_file: String = String::from("./src/config/test.json");

  // let cnf: ScanServerList = {
  //   let cnf: String = std::fs::read_to_string(&config_file).unwrap();
  //   serde_json::from_str::<ScanServerList>(&cnf).unwrap()
  // };

  // for index in 0..cnf.server.len() {
  //   let user: &String = &cnf.server[index].user;
  //   let host: &String = &cnf.server[index].host;
  //   let port: &String = &cnf.server[index].port;
  //   let key:  &String = &cnf.server[index].key;
  //   let prikey: std::path::PathBuf = std::path::PathBuf::from(key);

  //   let host_port: String = host.to_owned() + ":" + port;

    // hostname
    let mut sess: Session = Session::new().unwrap();
    let tcp: std::net::TcpStream = std::net::TcpStream::connect(host_port.clone()).unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    sess.userauth_pubkey_file(user, None, prikey.as_path(), None).unwrap();
    let mut ch: ssh2::Channel = sess.channel_session().unwrap();
    ch.exec("hostname").unwrap();
    let mut hostname: String = String::new();
    ch.read_to_string(&mut hostname).unwrap();
    ch.wait_close().expect("Close SSH Connection Failed...");

    if ch.exit_status().unwrap() == 0 {
    } else {
      println!("command hostname Failed");
    }

    // time
    let utc: OffsetDateTime = OffsetDateTime::now_utc();
    let jct: OffsetDateTime = utc.to_offset(offset!(+9));
    let format: Vec<format_description::FormatItem<'_>> = format_description::parse(
      "[year]-[month]-[day] [hour]:[minute]:[second]"
    ).unwrap();
    let time: String = jct.format(&format).unwrap();

    // kernel
    let mut sess: Session = Session::new().unwrap();
    let tcp: std::net::TcpStream = std::net::TcpStream::connect(host_port.clone()).unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    sess.userauth_pubkey_file(user, None, prikey.as_path(), None).unwrap();
    let mut ch: ssh2::Channel = sess.channel_session().unwrap();
    ch.exec("uname -r").unwrap();
    let mut kernel: String = String::new();
    ch.read_to_string(&mut kernel).unwrap();
    ch.wait_close().expect("Close SSH Connection Failed...");
    
    if ch.exit_status().unwrap() == 0 {
    } else {
      println!("command uname -r Failed");
    }

    // OS
    let mut sess: Session = Session::new().unwrap();
    let tcp: std::net::TcpStream = std::net::TcpStream::connect(host_port.clone()).unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    sess.userauth_pubkey_file(user, None, prikey.as_path(), None).unwrap();
    let mut ch: ssh2::Channel = sess.channel_session().unwrap();
    ch.exec("lsb_release -idrc").unwrap();
    let mut ubuntu: String = String::new();
    ch.read_to_string(&mut ubuntu).unwrap();

    let mut distributor: String = "Null".to_string();
    let mut description: String = "Null".to_string();
    let mut release:     String = "Null".to_string();
    let mut codename:    String = "Null".to_string();
    let v: Vec<&str> = ubuntu.lines().collect();
    
    for i in v {
      let s1: &str = i.trim();
      let s2: Vec<&str> = s1.split(':').collect();
      
      if s2.len() == 2 {
        match s2[0] {
          "Distributor ID" => distributor = s2[1].to_string().replace('\t', ""),
          "Description"    => description = s2[1].to_string().replace('\t', ""),
          "Release"        => release     = s2[1].to_string().replace('\t', ""),
          "Codename"       => codename    = s2[1].to_string().replace('\t', ""),
          _                => println!("Not match any pattern..."),
        }
      } else {
        println!("lsb_release len Failed...");
      }
    }

    let os: UbuntuVersion = UbuntuVersion {
      distributor,
      description,
      release,
      codename,
    };

    ch.wait_close().expect("Close SSH Connection Failed...");
    
    if ch.exit_status().unwrap() == 0 {
    } else {
      println!("command lsb_release Failed");
    }

    let mut localinfo: ScanResult = ScanResult {
      time,
      os,
      kernel: kernel.replace('\n', ""),
      hostname: hostname.replace('\n', ""),
      ip: vec![],
      pkg: vec![]
    };

    // ip addr
    let mut sess: Session = Session::new().unwrap();
    let tcp: std::net::TcpStream = std::net::TcpStream::connect(host_port.clone()).unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    sess.userauth_pubkey_file(user, None, prikey.as_path(), None).unwrap();
    let mut ch: ssh2::Channel = sess.channel_session().unwrap();
    ch.exec("/sbin/ip -o addr").unwrap();
    let mut ip: String = String::new();
    ch.read_to_string(&mut ip).unwrap();
    let v: Vec<&str> = ip.lines().collect();
    for i in &v {
      let t: &str = i.trim();
      let u: Vec<&str> = t.split_whitespace().collect::<Vec<&str>>();
      let ipaddr: String = String::from(u[1]) + "_!_" + u[3];
      localinfo.ip.push(ipaddr);
    }
    ch.wait_close().expect("Close SSH Connection Failed...");

    if ch.exit_status().unwrap() == 0 {
    } else {
      println!("command ip addr Failed");
    }

    // --installed
    let mut sess: Session = Session::new().unwrap();
    let tcp: std::net::TcpStream = std::net::TcpStream::connect(host_port).unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    sess.userauth_pubkey_file(user, None, prikey.as_path(), None).unwrap();
    let mut ch: ssh2::Channel = sess.channel_session().unwrap();
    ch.exec("apt list --installed\n\"").unwrap();
    let mut s: String = String::new();
    ch.read_to_string(&mut s).unwrap();

    let v: Vec<&str> = s.lines().collect();
    for i in &v {
      let t: &str = i.trim();
      let s1: Vec<&str> = t.split_whitespace().collect::<Vec<&str>>();

      if s1.len() == 6 {
        let ver:   &str = s1[1];
        let arch:  &str = s1[2];
        let upver: &str = s1[5];
        let s2:  Vec<_> = s1[0].split('/').collect();
        let name:  &str = s2[0];
        localinfo.pkg.push(PkgList { pkgname: name.to_string(), pkgver: ver.to_string(), update_flag: "〇".to_string(), upver: upver.to_string(), pkgarch: arch.to_string()});
      } else if s1.len() == 4 {
        let ver:  &str = s1[1];
        let arch: &str = s1[2];
        let s2: Vec<_> = s1[0].split('/').collect();
        let name: &str = s2[0];
        localinfo.pkg.push(PkgList { pkgname: name.to_string(), pkgver: ver.to_string(), update_flag: "-".to_string(), upver: ver.to_string(), pkgarch: arch.to_string()});
      } else {
        println!("--installed split Failed...");
      }
    }
    ch.wait_close().expect("Close SSH Connection Failed...");
    
    if ch.exit_status().unwrap() == 0 {
    } else {
      println!("command dnf rpm -qa Failed");
    }

    let filename:   String = hostname.replace('\n', "") + ".json";
    let result_dir: String = String::from("./src/scan_result/");
    let full_path:  String = String::from(&result_dir) + &filename;

    let serialized: String   = serde_json::to_string(&localinfo).unwrap();
    let mut w: std::fs::File = std::fs::OpenOptions::new()
      .write(true)
      .create(true)
      .open(full_path).unwrap();
    w.write_all(serialized.as_bytes()).expect("Failed to Write scan_result...");
  // }

  Ok(())
}