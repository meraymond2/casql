use crate::model::PartialConnOpts;

use directories::ProjectDirs;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::prelude::*;

pub const CONNS_FN: &str = "connections.toml";

fn read_conns() -> HashMap<String, PartialConnOpts> {
  let dirs = ProjectDirs::from("", "", "Casql").expect("Dirs failed");
  let conns_path = dirs.config_dir().join(CONNS_FN);
  let mut file = std::fs::File::open(conns_path).unwrap();
  let mut contents = String::new();

  file.read_to_string(&mut contents).unwrap();
  toml::from_str(&contents).expect("Invalid toml in file")
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

pub fn save(name: String, opts: PartialConnOpts) {
  let mut connection_map = read_conns();

  connection_map.insert(name.clone(), opts);
  write_conns(connection_map);
  println!("Connection {:?} saved.", name);
}

pub fn list() {
  let connection_map = read_conns();
  let keys = connection_map.keys();

  println!("Connections {:?}:", keys);
}

pub fn describe(name: String) {
  let connection_map = read_conns();
  match connection_map.get(&name) {
    Some(connection) => println!("Connection: {:?}", connection),
    None => println!("Connection not found."),
  }
}

pub fn delete(name: String) {
  let mut connection_map = read_conns();
  match connection_map.remove(&name) {
    Some(connection) => {
      write_conns(connection_map);
      println!("Connection delete: {:?}", connection);
    }
    None => println!("Connection not found."),
  }
}
