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
  pkgname    : String,
  pkgver     : String,
  pkgrelease : String,
  update_flag: String,
  upver      : String,
  uprelease  : String,
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
  repo: String
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

  if ch.exit_status()? == 0 {
  } else {
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
  
  if ch.exit_status()? == 0 {
  } else {
    println!("command uname -r Failed");
  }

  // OS
  let mut sess: Session = Session::new()?;
  let tcp: std::net::TcpStream = std::net::TcpStream::connect(host_port.clone())?;
  sess.set_tcp_stream(tcp);
  sess.handshake()?;
  sess.userauth_pubkey_file(user, None, prikey.as_path(), None)?;
  let mut ch: ssh2::Channel = sess.channel_session()?;
  ch.exec("cat /etc/system-release")?;
  let mut os: String = String::new();
  ch.read_to_string(&mut os)?;
  ch.wait_close().expect("Close SSH Connection Failed...");
  
  if ch.exit_status()? == 0 {
  } else {
    println!("command system-release Failed");
  }

  let mut localinfo: ScanResult = ScanResult {
    time,
    os,
    kernel,
    hostname: hostname.clone(),
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

  if ch.exit_status()? == 0 {
  } else {
    println!("command ip addr Failed");
  }

  // makecache
  let mut sess: Session = Session::new()?;
  let tcp: std::net::TcpStream = std::net::TcpStream::connect(host_port.clone())?;
  sess.set_tcp_stream(tcp);
  sess.handshake()?;
  sess.userauth_pubkey_file(user, None, prikey.as_path(), None)?;
  let mut ch: ssh2::Channel = sess.channel_session()?;
  ch.exec("dnf makecache --assumeyes")?;
  let mut makecache: String = String::new();
  ch.read_to_string(&mut makecache)?;
  ch.wait_close().expect("Close SSH Connection Failed...");
  
  if ch.exit_status()? == 0 {
  } else {
    println!("command dnf makecache Failed");
  }

  // check-update
  let mut updateinfo: Update = Update {
    update: vec![]
  };

  let mut sess: Session = Session::new()?;
  let tcp: std::net::TcpStream = std::net::TcpStream::connect(host_port.clone())?;
  sess.set_tcp_stream(tcp);
  sess.handshake()?;
  sess.userauth_pubkey_file(user, None, prikey.as_path(), None)?;
  let mut ch: ssh2::Channel = sess.channel_session()?;
  ch.exec("dnf check-update --assumeyes")?;
  let mut check_update: String = String::new();
  ch.read_to_string(&mut check_update)?;

  let v: Vec<&str> = check_update.lines().collect();
  for i in &v {
    let t: &str = i.trim();
    let u: Vec<&str> = t.split_whitespace().collect::<Vec<&str>>();

    if u.len() == 3 {
      let name: &str = u[0];
      let ver: &str  = u[1];
      let repo: &str = u[2];
      updateinfo.update.push(UpdateList { name: name.to_string(), ver: ver.to_string(), repo: repo.to_string() });
    } else {
      println!("len 3 Failed");
    }
  }
  ch.wait_close().expect("Close SSH Connection Failed...");
  
  if ch.exit_status()? == 0 {
  } else {
    println!("command dnf check-update Failed");
  }

  // pkg
  let mut sess: Session = Session::new()?;
  let tcp: std::net::TcpStream = std::net::TcpStream::connect(host_port)?;
  sess.set_tcp_stream(tcp);
  sess.handshake()?;
  sess.userauth_pubkey_file(user, None, prikey.as_path(), None)?;
  let mut ch: ssh2::Channel = sess.channel_session()?;
  ch.exec("rpm -qa --queryformat \"%{NAME} %{EPOCHNUM} %{VERSION} %{RELEASE} %{ARCH}\n\"")?;
  let mut s: String = String::new();
  ch.read_to_string(&mut s)?;

  let v: Vec<&str> = s.lines().collect();
  for i in &v {
    let t: &str = i.trim();
    let u: Vec<&str> = t.split_whitespace().collect::<Vec<&str>>();
    if u.len() == 5 {
      if u[1] == "0" || u[1] == "none" {
        let name: &str    = u[0];
        let ver: &str     = u[2];
        let release: &str = u[3];
        let arch: &str    = u[4];

        let name_arch: String = String::from(name) + "." + arch;
        let mut update_vec: Vec<Vec<&str>> = Vec::new();
        for i in &updateinfo.update {
          if name_arch == i.name {
            let c: Vec<&str> = i.ver.split('-').collect();
            if c.len() == 2 {
              update_vec.push(c);
            } else {
              println!("バージョン番号とリリース番号の分割に失敗しました...");
            }
          }
        }

        let no: String = "-".to_string();
        let yes:String = "〇".to_string();

        if update_vec.is_empty() {
          localinfo.pkg.push(PkgList { pkgname: name.to_string(), pkgver: ver.to_string(), pkgrelease: release.to_string(), update_flag: no, upver: ver.to_string(), uprelease: release.to_string(), pkgarch: arch.to_string()});
        } else {
          localinfo.pkg.push(PkgList { pkgname: name.to_string(), pkgver: ver.to_string(), pkgrelease: release.to_string(), update_flag: yes, upver: update_vec[0][0].to_string(), uprelease: update_vec[0][1].to_string(), pkgarch: arch.to_string()});
        }
      } else {
        let name: &str    = u[0];
        let ver: &str     = u[1];
        let release: &str = u[3];
        let arch: &str    = u[4];
        
        let name_arch: String = String::from(name) + "." + arch;
        let mut update_vec: Vec<Vec<&str>> = Vec::new();
        for i in &updateinfo.update {
          if name_arch == i.name {
            let c: Vec<&str> = i.ver.split('-').collect();
            if c.len() == 2 {
              update_vec.push(c);
            } else {
              println!("バージョン番号とリリース番号の分割に失敗しました...");
            }
          }
        }

        let no: String = "-".to_string();
        let yes:String = "〇".to_string();

        if update_vec.is_empty() {
          localinfo.pkg.push(PkgList { pkgname: name.to_string(), pkgver: ver.to_string(), pkgrelease: release.to_string(), update_flag: no, upver: ver.to_string(), uprelease: release.to_string(), pkgarch: arch.to_string()});
        } else {
          localinfo.pkg.push(PkgList { pkgname: name.to_string(), pkgver: ver.to_string(), pkgrelease: release.to_string(), update_flag: yes, upver: update_vec[0][0].to_string(), uprelease: update_vec[0][1].to_string(), pkgarch: arch.to_string()});
        }
      }
    } else {
      println!("len 5 Failed");
    }
  }
  ch.wait_close().expect("Close SSH Connection Failed...");
  
  if ch.exit_status()? == 0 {
  } else {
    println!("command dnf rpm -qa Failed");
  }

  let filename:   String = hostname.replace('\n', "") + ".json";
  let result_dir: String = String::from("./src/scan_result/");
  let full_path:  String = String::from(&result_dir) + &filename;

  let serialized: String = serde_json::to_string(&localinfo)?;
  let mut w: std::fs::File = std::fs::OpenOptions::new()
    .write(true)
    .create(true)
    .open(full_path)?;
  w.write_all(serialized.as_bytes()).expect("Failed to Write scan_result...");

  Ok(())
}