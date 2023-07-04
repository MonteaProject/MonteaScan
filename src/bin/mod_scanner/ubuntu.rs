use anyhow::Result;
use time::{OffsetDateTime, macros::offset, format_description};
use ssh2::Session;
use serde_derive::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::path::PathBuf;


#[derive(Deserialize, Serialize, Debug)]
struct ScanResult {
  time    : String,
  hostname: String,
  ip      : Vec<String>,
  os      : String,
  kernel  : String,
  pkg     : Vec<PkgList>
}

#[derive(Deserialize, Serialize, Debug)]
struct PkgList {
  pkgname:     String,
  pkgver:      String,
  pkgrelease:  String,
  update_flag: String,
  upver:       String,
  uprelease:   String,
  pkgarch:     String
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


pub fn main(user: &str, prikey: PathBuf, host_port: String) -> Result<()> {
  // hostname
  let mut sess: Session = Session::new()?;
  let tcp: std::net::TcpStream = std::net::TcpStream::connect(host_port.clone())?;
  sess.set_tcp_stream(tcp);
  sess.handshake()?;
  sess.userauth_pubkey_file(user, None, prikey.as_path(), None)?;
  let mut ch: ssh2::Channel = sess.channel_session()?;
  ch.exec("hostname")?;
  let mut hostname: String = String::new();
  ch.read_to_string(&mut hostname)?;
  ch.wait_close().expect("Close SSH Connection Failed...");

  if ch.exit_status()? != 0 {
    println!("command hostname Failed");
  }

  // time
  let utc: OffsetDateTime = OffsetDateTime::now_utc();
  let jct: OffsetDateTime = utc.to_offset(offset!(+9));
  let format: Vec<format_description::FormatItem<'_>> = format_description::parse(
    "[year]-[month]-[day] [hour]:[minute]:[second]"
  )?;
  let time: String = jct.format(&format)?;

  // kernel
  let mut sess: Session = Session::new()?;
  let tcp: std::net::TcpStream = std::net::TcpStream::connect(host_port.clone())?;
  sess.set_tcp_stream(tcp);
  sess.handshake()?;
  sess.userauth_pubkey_file(user, None, prikey.as_path(), None)?;
  let mut ch: ssh2::Channel = sess.channel_session()?;
  ch.exec("uname -r")?;
  let mut kernel: String = String::new();
  ch.read_to_string(&mut kernel)?;
  ch.wait_close().expect("Close SSH Connection Failed...");
  
  if ch.exit_status()? != 0 {
    println!("command uname -r Failed");
  }

  // OS
  let mut sess: Session = Session::new()?;
  let tcp: std::net::TcpStream = std::net::TcpStream::connect(host_port.clone())?;
  sess.set_tcp_stream(tcp);
  sess.handshake()?;
  sess.userauth_pubkey_file(user, None, prikey.as_path(), None)?;
  let mut ch: ssh2::Channel = sess.channel_session()?;
  ch.exec("lsb_release -d")?;
  let mut ubuntu: String = String::new();
  ch.read_to_string(&mut ubuntu)?;

  let s1: Vec<&str> = ubuntu.split(':').collect();
  let mut os: String = "Null".to_string();
  if s1.len() == 2 {
    os = s1[1].to_string().replace(['\t', '\n'], "")
  }

  ch.wait_close().expect("Close SSH Connection Failed...");
  
  if ch.exit_status()? != 0 {
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
  let mut sess: Session = Session::new()?;
  let tcp: std::net::TcpStream = std::net::TcpStream::connect(host_port.clone())?;
  sess.set_tcp_stream(tcp);
  sess.handshake()?;
  sess.userauth_pubkey_file(user, None, prikey.as_path(), None)?;
  let mut ch: ssh2::Channel = sess.channel_session()?;
  ch.exec("/sbin/ip -o addr")?;
  let mut ip: String = String::new();
  ch.read_to_string(&mut ip)?;
  let v: Vec<&str> = ip.lines().collect();
  for i in &v {
    let t: &str = i.trim();
    let u: Vec<&str> = t.split_whitespace().collect::<Vec<&str>>();
    let ipaddr: String = String::from(u[1]) + "_!_" + u[3];
    localinfo.ip.push(ipaddr);
  }
  ch.wait_close().expect("Close SSH Connection Failed...");

  if ch.exit_status()? != 0 {
    println!("command ip addr Failed");
  }

  // --installed
  let mut sess: Session = Session::new()?;
  let tcp: std::net::TcpStream = std::net::TcpStream::connect(host_port)?;
  sess.set_tcp_stream(tcp);
  sess.handshake()?;
  sess.userauth_pubkey_file(user, None, prikey.as_path(), None)?;
  let mut ch: ssh2::Channel = sess.channel_session()?;
  ch.exec("apt list --installed\n\"")?;
  let mut s: String = String::new();
  ch.read_to_string(&mut s)?;

  let v: Vec<&str> = s.lines().collect();
  for i in &v {
    let t: &str = i.trim();
    let s1: Vec<&str> = t.split_whitespace().collect::<Vec<&str>>();

    if s1.len() == 6 {
      let s3:  Vec<_> = s1[0].split('/').collect();
      let name:  &str = s3[0];
      let arch: &str  = s1[2];

      let mut ver:     &str = s1[1]; // [epoch:]upstream-version[-debian-revision]
      let mut release: &str = "-";

      let r2: Vec<&str> = s1[1].split(':').collect();
      if r2.len() == 2 {
        if r2[0] == "0" || r2[0] == "none" {
          let v: Vec<&str> = s1[1].split('-').collect();
          if v.len() == 2 {
            ver     = v[0];  // upstream-version
            release = v[1];  // [-debian-revision]
          }
        } else {
          let v: Vec<&str> = r2[1].split('-').collect();
          if v.len() == 2 {
            ver     = r2[0]; // [epoch:]
            release = v[1];  // [-debian-revision]
          }
        }
      } else {
        let v: Vec<&str> = s1[1].split('-').collect();
        if v.len() == 2 {
          ver     = v[0];  // upstream-version
          release = v[1];  // [-debian-revision]
        }
      }
      
      let mut upver:     &str = s1[5];
      let mut uprelease: &str = "-";
      let r1: Vec<&str> = s1[5].split(':').collect();
      if r1.len() == 2 {
        if r1[0] == "0" || r1[0] == "none" {
          let v: Vec<&str> = r1[1].split('-').collect();
          if v.len() == 2 {
            upver     = v[0];
            uprelease = v[1];
          }
        } else {
          let v: Vec<&str> = r1[1].split('-').collect();
          if v.len() == 2 {
            upver     = r1[0];
            uprelease = v[1];
          }
        }
      } else {
        let v: Vec<&str> = r1[1].split('-').collect();
        if v.len() == 2 {
          upver     = v[0];
          uprelease = v[1];
        }
      }
      localinfo.pkg.push(PkgList { pkgname: name.to_string(), pkgver: ver.to_string(), pkgrelease: release.to_string(), update_flag: "〇".to_string(), upver: upver.to_string(), uprelease: uprelease.to_string(), pkgarch: arch.to_string()});
    }
    else if s1.len() == 5 {
      // ["libmm-glib0/now", "1.20.0-1~ubuntu22.04.1", "amd64", "[インストール済み、1.20.0-1~ubuntu22.04.2", "にアップグレード可]"]
      let s3:  Vec<_> = s1[0].split('/').collect();
      let name:  &str = s3[0];
      let arch: &str  = s1[2];

      let mut ver:     &str = s1[1]; // [epoch:]upstream-version[-debian-revision]
      let mut release: &str = "-";

      let r2: Vec<&str> = s1[1].split(':').collect();
      if r2.len() == 2 {
        if r2[0] == "0" || r2[0] == "none" {
          let v: Vec<&str> = s1[1].split('-').collect();
          if v.len() == 2 {
            ver     = v[0];  // upstream-version
            release = v[1];  // [-debian-revision]
          }
        } else {
          let v: Vec<&str> = r2[1].split('-').collect();
          if v.len() == 2 {
            ver     = r2[0]; // [epoch:]
            release = v[1];  // [-debian-revision]
          }
        }
      } else {
        let v: Vec<&str> = s1[1].split('-').collect();
        if v.len() == 2 {
          ver     = v[0];  // upstream-version
          release = v[1];  // [-debian-revision]
        }
      }
      
      let upver:     &str = "-";
      let uprelease: &str = "-";
      // let r1: Vec<&str> = s1[5].split(':').collect();
      // if r1.len() == 2 {
      //   if r1[0] == "0" || r1[0] == "none" {
      //     let v: Vec<&str> = r1[1].split('-').collect();
      //     if v.len() == 2 {
      //       upver     = v[0];
      //       uprelease = v[1];
      //     }
      //   } else {
      //     let v: Vec<&str> = r1[1].split('-').collect();
      //     if v.len() == 2 {
      //       upver     = r1[0];
      //       uprelease = v[1];
      //     }
      //   }
      // } else {
      //   let v: Vec<&str> = r1[1].split('-').collect();
      //   if v.len() == 2 {
      //     upver     = v[0];
      //     uprelease = v[1];
      //   }
      // }
      localinfo.pkg.push(PkgList { pkgname: name.to_string(), pkgver: ver.to_string(), pkgrelease: release.to_string(), update_flag: "-".to_string(), upver: upver.to_string(), uprelease: uprelease.to_string(), pkgarch: arch.to_string()});
    }
    else if s1.len() == 4 {
      let s3: Vec<_> = s1[0].split('/').collect();
      let name: &str = s3[0];
      let arch: &str = s1[2];

      let mut ver:     &str = s1[1];
      let mut release: &str = "-";

      let r2: Vec<&str> = s1[1].split(':').collect();
      if r2.len() == 2 {
        if r2[0] == "0" || r2[0] == "none" {
          let v: Vec<&str> = s1[1].split('-').collect();
          if v.len() == 2 {
            ver     = v[0];
            release = v[1];
          }
        } else {
          let v: Vec<&str> = r2[1].split('-').collect();
          if v.len() == 2 {
            ver     = r2[0];
            release = v[1];
          }
        }
      } else {
        let v: Vec<&str> = s1[1].split('-').collect();
        if v.len() == 2 {
          ver     = v[0];
          release = v[1];
        }
      }
      localinfo.pkg.push(PkgList { pkgname: name.to_string(), pkgver: ver.to_string(), pkgrelease: release.to_string(), update_flag: "-".to_string(), upver: ver.to_string(), uprelease: release.to_string(), pkgarch: arch.to_string()});
    } else {
      println!("I1002...");
    }
  }
  ch.wait_close().expect("Close SSH Connection Failed...");
  
  if ch.exit_status()? != 2 {
    println!("command apt list --installed Failed {:?}", ch.exit_status());
  }

  let filename:   String = hostname.replace('\n', "") + ".json";
  let result_dir: String = String::from("./src/scan_result/");
  let full_path:  String = String::from(&result_dir) + &filename;

  let serialized: String   = serde_json::to_string(&localinfo)?;
  let mut w: std::fs::File = std::fs::OpenOptions::new()
    .write(true)
    .create(true)
    .open(full_path)?;
  w.write_all(serialized.as_bytes()).expect("Failed to Write scan_result...");

  Ok(())
}