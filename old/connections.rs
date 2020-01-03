use crate::model::PartialConnOpts;

use directories::ProjectDirs;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::prelude::*;

const CONNS_FN: &str = "connections.toml";

fn read_conns() -> HashMap<String, PartialConnOpts> {
  let dirs = ProjectDirs::from("", "", "Casql").expect("TODO: How can dirs fail?");
  let conns_path = dirs.config_dir().join(CONNS_FN);
  let mut file = std::fs::File::open(conns_path).expect("TODO: what if file doesn't exist?");
  let mut contents = String::new();

  file.read_to_string(&mut contents).expect("TODO: How can this fail?");
  toml::from_str(&contents).expect("TODO: Invalid toml in file")
}

fn write_conns(table: HashMap<String, PartialConnOpts>) {
  let dirs = ProjectDirs::from("", "", "Casql").expect("Dirs failed");
  let conns_path = dirs.config_dir().join(CONNS_FN);
  let mut file = OpenOptions::new()
    .write(true)
    .truncate(true)
    .open(conns_path)
    .expect("Should be able to open file as writeable");

  let toml = toml::to_string(&table).expect("Config should be tomlable");
  let bytes = toml.into_bytes();
  file
    .write_all(&bytes)
    .expect("File should write correctly.");
}

pub fn save(name: &str, opts: PartialConnOpts) {
  let mut connection_map = read_conns();

  connection_map.insert(name.to_owned(), opts);
  write_conns(connection_map);
  println!("Connection {} saved.", name);
}

pub fn list() {
  let connection_map = read_conns();
  let keys = connection_map.keys();

  println!("Connections:");
  for k in keys {
    println!("\t{}", k);
  }
}

pub fn describe(name: &str) {
  let connection_map = read_conns();
  match connection_map.get(name) {
    Some(connection) => println!("Connection: {:?}", connection),
    None => println!("Connection not found."),
  }
}

pub fn load(name: &str) -> PartialConnOpts {
  let mut connection_map = read_conns();
  match connection_map.remove(name) { // is there a better way to get the owned value?
    Some(connection) => connection,
    None => panic!("Connection not found."),
  }
}

pub fn delete(name: &str) {
  let mut connection_map = read_conns();
  match connection_map.remove(name) {
    Some(_) => {
      write_conns(connection_map);
      println!("Connection deleted: {}", name);
    }
    None => println!("Connection not found."),
  }
}