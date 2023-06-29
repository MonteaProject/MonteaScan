mod mod_scanner;
use crate::mod_scanner::rhel::main as scanner_rhel;
use crate::mod_scanner::ubuntu::main as scanner_ubuntu;

fn main() {
  scanner_rhel();
  scanner_ubuntu();
}