use serde_derive::{Deserialize, Serialize};
use ssh2::Session;
use std::io::{Read, Write};
use time::{OffsetDateTime, macros::offset, format_description};
use serde_json::{Result};


#[derive(Deserialize, Serialize, Debug)]
struct ScanServerList {
  server: Vec<Server>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Server {
  user : String,
  host: String,
  port: String,
  key : String
}

#[derive(Deserialize, Serialize, Debug)]
struct ScanResult {
  time: String,
  hostname: String,
  ip: Vec<String>,
  os: String,
  kernel: String,
  pkg: Vec<PkgList>
}

#[derive(Deserialize, Serialize, Debug)]
struct PkgList {
  pkgname: String,
  pkgver: String,
  pkgrelease: String,
  upver: String,
  uprelease: String,
  pkgarch: String
}

#[derive(Deserialize, Serialize, Debug)]
struct Update {
  update: Vec<UpdateList>
}

#[derive(Deserialize, Serialize, Debug)]
struct UpdateList {
  name: String,
  ver: String,
  repo: String
}


fn main() -> Result<()> {
  let filename = String::from("config") + ".json";
  let dir = String::from("./src/config/") + &filename;

  let cnf = {
    let cnf = std::fs::read_to_string(&dir).unwrap();
    serde_json::from_str::<ScanServerList>(&cnf).unwrap()
  };

  for index in 0..cnf.server.len() {
    let user    = &cnf.server[index].user;
    let host   = &cnf.server[index].host;
    let port   = &cnf.server[index].port;
    let key    = &cnf.server[index].key;
    let prikey = std::path::PathBuf::from(key);

    let host_port = host.to_owned() + ":" + port;

    // hostname
    let mut sess = Session::new().unwrap();
    let tcp = std::net::TcpStream::connect(host_port.clone()).unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    sess.userauth_pubkey_file(user, None, prikey.as_path(), None).unwrap();
    let mut ch = sess.channel_session().unwrap();
    ch.exec("hostname").unwrap();
    let mut hostname = String::new();
    ch.read_to_string(&mut hostname).unwrap();
    ch.wait_close().expect("Close SSH Connection Failed...");

    if ch.exit_status().unwrap() == 0 {
    } else {
      println!("command hostname Failed");
    }

    // time
    let utc = OffsetDateTime::now_utc();
    let jct = utc.to_offset(offset!(+9));
    let format = format_description::parse(
      "[year]-[month]-[day] [hour]:[minute]:[second]"
    ).unwrap();
    let time = jct.format(&format).unwrap();

    // kernel
    let mut sess = Session::new().unwrap();
    let tcp = std::net::TcpStream::connect(host_port.clone()).unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    sess.userauth_pubkey_file(user, None, prikey.as_path(), None).unwrap();
    let mut ch = sess.channel_session().unwrap();
    ch.exec("uname -r").unwrap();
    let mut kernel = String::new();
    ch.read_to_string(&mut kernel).unwrap();
    ch.wait_close().expect("Close SSH Connection Failed...");
    
    if ch.exit_status().unwrap() == 0 {
    } else {
      println!("command uname -r Failed");
    }

    // OS
    let mut sess = Session::new().unwrap();
    let tcp = std::net::TcpStream::connect(host_port.clone()).unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    sess.userauth_pubkey_file(user, None, prikey.as_path(), None).unwrap();
    let mut ch = sess.channel_session().unwrap();
    ch.exec("cat /etc/system-release").unwrap();
    let mut os = String::new();
    ch.read_to_string(&mut os).unwrap();
    ch.wait_close().expect("Close SSH Connection Failed...");
    
    if ch.exit_status().unwrap() == 0 {
    } else {
      println!("command system-release Failed");
    }

    let mut localinfo = ScanResult {
      time,
      os,
      kernel,
      hostname: hostname.clone(),
      ip: vec![],
      pkg: vec![]
    };

    // ip addr
    let mut sess = Session::new().unwrap();
    let tcp = std::net::TcpStream::connect(host_port.clone()).unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    sess.userauth_pubkey_file(user, None, prikey.as_path(), None).unwrap();
    let mut ch = sess.channel_session().unwrap();
    ch.exec("/sbin/ip -o addr").unwrap();
    let mut ip = String::new();
    ch.read_to_string(&mut ip).unwrap();
    let v: Vec<&str> = ip.lines().collect();
    for i in &v {
      let t = i.trim();
      let u = t.split_whitespace().collect::<Vec<&str>>();
      let ipaddr = String::from(u[1]) + "_!_" + u[3];
      localinfo.ip.push(ipaddr);
    }
    ch.wait_close().expect("Close SSH Connection Failed...");

    if ch.exit_status().unwrap() == 0 {
    } else {
      println!("command ip addr Failed");
    }

    // makecache
    let mut sess = Session::new().unwrap();
    let tcp = std::net::TcpStream::connect(host_port.clone()).unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    sess.userauth_pubkey_file(user, None, prikey.as_path(), None).unwrap();
    let mut ch = sess.channel_session().unwrap();
    ch.exec("dnf makecache --assumeyes").unwrap();
    let mut makecache = String::new();
    ch.read_to_string(&mut makecache).unwrap();
    ch.wait_close().expect("Close SSH Connection Failed...");
    
    if ch.exit_status().unwrap() == 0 {
    } else {
      println!("command dnf makecache Failed");
    }

    // check-update
    let mut updateinfo = Update {
      update: vec![]
    };

    let mut sess = Session::new().unwrap();
    let tcp = std::net::TcpStream::connect(host_port.clone()).unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    sess.userauth_pubkey_file(user, None, prikey.as_path(), None).unwrap();
    let mut ch = sess.channel_session().unwrap();
    ch.exec("dnf check-update --assumeyes").unwrap();
    let mut check_update = String::new();
    ch.read_to_string(&mut check_update).unwrap();

    let v: Vec<&str> = check_update.lines().collect();
    for i in &v {
      let t = i.trim();
      let u = t.split_whitespace().collect::<Vec<&str>>();

      if u.len() == 3 {
        let name = u[0];
        let ver  = u[1];
        let repo = u[2];
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
    let mut sess = Session::new().unwrap();
    let tcp = std::net::TcpStream::connect(host_port.clone()).unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    sess.userauth_pubkey_file(user, None, prikey.as_path(), None).unwrap();
    let mut ch = sess.channel_session().unwrap();
    ch.exec("rpm -qa --queryformat \"%{NAME} %{EPOCHNUM} %{VERSION} %{RELEASE} %{ARCH}\n\"").unwrap();
    let mut s = String::new();
    ch.read_to_string(&mut s).unwrap();

    let v: Vec<&str> = s.lines().collect();
    for i in &v {
      let t = i.trim();
      let u = t.split_whitespace().collect::<Vec<&str>>();
      if u.len() == 5 {
        if u[1] == "0" || u[1] == "none" {
          let name    = u[0];
          let ver     = u[2];
          let release = u[3];
          let arch    = u[4];

          let name_arch = String::from(name) + "." + arch;
          let mut b = Vec::new();
          for i in &updateinfo.update {
            if name_arch == i.name {
              let c: Vec<&str> = i.ver.split('-').collect();
              if c.len() == 2 {
                b.push(c);
              } else {
                println!("バージョン番号とリリース番号の分割に失敗しました...");
              }
            }
          }
          if b.is_empty() {
            localinfo.pkg.push(PkgList { pkgname: name.to_string(), pkgver: ver.to_string(), pkgrelease: release.to_string(), upver: ver.to_string(), uprelease: release.to_string(), pkgarch: arch.to_string()});
          } else {
            localinfo.pkg.push(PkgList { pkgname: name.to_string(), pkgver: ver.to_string(), pkgrelease: release.to_string(), upver: b[0][0].to_string(), uprelease: b[0][1].to_string(), pkgarch: arch.to_string()});
          }
        } else {
          let name    = u[0];
          let ver     = u[1];
          let release = u[3];
          let arch    = u[4];
          
          let name_arch = String::from(name) + "." + arch;
          let mut b = Vec::new();
          for i in &updateinfo.update {
            if name_arch == i.name {
              let c: Vec<&str> = i.ver.split('-').collect();
              if c.len() == 2 {
                b.push(c);
              } else {
                println!("バージョン番号とリリース番号の分割に失敗しました...");
              }
            }
          }
          if b.is_empty() {
            localinfo.pkg.push(PkgList { pkgname: name.to_string(), pkgver: ver.to_string(), pkgrelease: release.to_string(), upver: ver.to_string(), uprelease: release.to_string(), pkgarch: arch.to_string()});
          } else {
            localinfo.pkg.push(PkgList { pkgname: name.to_string(), pkgver: ver.to_string(), pkgrelease: release.to_string(), upver: b[0][0].to_string(), uprelease: b[0][1].to_string(), pkgarch: arch.to_string()});
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

    std::fs::create_dir_all("./src/scan_result/").unwrap();
    let filename = hostname.replace('\n', "") + ".json";
    let dir = String::from("./src/scan_result/") + &filename;

    let serialized = serde_json::to_string(&localinfo).unwrap();
    let mut w = std::fs::OpenOptions::new()
      .write(true)
      .create(true)
      .open(dir).unwrap();
    w.write_all(serialized.as_bytes()).expect("Failed to Write scan_result...");
  }

  Ok(())
}