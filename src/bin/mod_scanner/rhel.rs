use time::{OffsetDateTime, macros::offset, format_description};
use ssh2::Session;
use serde_derive::{Deserialize, Serialize};
use serde_json::{Result};
use std::io::{Read, Write};
use std::path::Path;


#[derive(Deserialize, Serialize, Debug)]
struct ScanServerList {
  server: Vec<Server>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Server {
  user : String,
  host : String,
  port : String,
  key  : String
}

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

pub fn main() -> Result<()> {
  let result_dir = String::from("./src/scan_result/");
  let result_dirpath = Path::new(&result_dir);

  if result_dirpath.is_dir() {
    println!("Remove dir... {:?}", result_dir);
    std::fs::remove_dir_all(&result_dir).unwrap();
  }

  std::fs::create_dir_all(&result_dir).unwrap();

  let config_file: String = String::from("./src/config/config.json");

  let cnf: ScanServerList = {
    let cnf: String = std::fs::read_to_string(&config_file).unwrap();
    serde_json::from_str::<ScanServerList>(&cnf).unwrap()
  };

  for index in 0..cnf.server.len() {
    let user: &String = &cnf.server[index].user;
    let host: &String = &cnf.server[index].host;
    let port: &String = &cnf.server[index].port;
    let key:  &String = &cnf.server[index].key;
    let prikey: std::path::PathBuf = std::path::PathBuf::from(key);

    let host_port: String = host.to_owned() + ":" + port;

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
    ch.exec("cat /etc/system-release").unwrap();
    let mut os: String = String::new();
    ch.read_to_string(&mut os).unwrap();
    ch.wait_close().expect("Close SSH Connection Failed...");
    
    if ch.exit_status().unwrap() == 0 {
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

    // makecache
    let mut sess: Session = Session::new().unwrap();
    let tcp: std::net::TcpStream = std::net::TcpStream::connect(host_port.clone()).unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    sess.userauth_pubkey_file(user, None, prikey.as_path(), None).unwrap();
    let mut ch: ssh2::Channel = sess.channel_session().unwrap();
    ch.exec("dnf makecache --assumeyes").unwrap();
    let mut makecache: String = String::new();
    ch.read_to_string(&mut makecache).unwrap();
    ch.wait_close().expect("Close SSH Connection Failed...");
    
    if ch.exit_status().unwrap() == 0 {
    } else {
      println!("command dnf makecache Failed");
    }

    // check-update
    let mut updateinfo: Update = Update {
      update: vec![]
    };

    let mut sess: Session = Session::new().unwrap();
    let tcp: std::net::TcpStream = std::net::TcpStream::connect(host_port.clone()).unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    sess.userauth_pubkey_file(user, None, prikey.as_path(), None).unwrap();
    let mut ch: ssh2::Channel = sess.channel_session().unwrap();
    ch.exec("dnf check-update --assumeyes").unwrap();
    let mut check_update: String = String::new();
    ch.read_to_string(&mut check_update).unwrap();

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
    
    if ch.exit_status().unwrap() == 0 {
    } else {
      println!("command dnf check-update Failed");
    }

    // pkg
    let mut sess: Session = Session::new().unwrap();
    let tcp: std::net::TcpStream = std::net::TcpStream::connect(host_port.clone()).unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    sess.userauth_pubkey_file(user, None, prikey.as_path(), None).unwrap();
    let mut ch: ssh2::Channel = sess.channel_session().unwrap();
    ch.exec("rpm -qa --queryformat \"%{NAME} %{EPOCHNUM} %{VERSION} %{RELEASE} %{ARCH}\n\"").unwrap();
    let mut s: String = String::new();
    ch.read_to_string(&mut s).unwrap();

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
    
    if ch.exit_status().unwrap() == 0 {
    } else {
      println!("command dnf rpm -qa Failed");
    }

    let filename: String = hostname.replace('\n', "") + ".json";
    let full_path: String = String::from(&result_dir) + &filename;

    let serialized: String = serde_json::to_string(&localinfo).unwrap();
    let mut w: std::fs::File = std::fs::OpenOptions::new()
      .write(true)
      .create(true)
      .open(full_path).unwrap();
    w.write_all(serialized.as_bytes()).expect("Failed to Write scan_result...");
  }

  Ok(())
}