use crate::errors::CasErr;
use crate::opts::PartialConnOpts;

use directories::ProjectDirs;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::fs::{DirBuilder, File};
use std::io::prelude::*;
use std::io::ErrorKind;
use std::io::{self, Write};
use std::path::Path;

const FILENAME: &str = "connections.toml";

fn initialise(dir_path: &Path) -> Result<File, CasErr> {
  DirBuilder::new()
    .recursive(true)
    .create(dir_path)
    .and_then(|_| {
      OpenOptions::new()
        .read(true)
        .write(true)
        .create_new(true)
        .open(dir_path.join(FILENAME))
    })
    .map_err(CasErr::from)
}

fn read_conns() -> Result<HashMap<String, PartialConnOpts>, CasErr> {
  let dirs = ProjectDirs::from("", "", "Casql").ok_or(CasErr::NoHomeDir)?;
  let conns_path = dirs.config_dir().join(FILENAME);
  let mut file = match OpenOptions::new().read(true).open(&conns_path) {
    Ok(f) => f,
    Err(ref e) if e.kind() == ErrorKind::NotFound => match initialise(dirs.config_dir()) {
      Ok(f) => f,
      Err(e) => return Err(e.into()),
    },
    Err(ref e) if e.kind() == ErrorKind::PermissionDenied => return Err(CasErr::FilePermissions),
    Err(e) => return Err(e.into()),
  };
  let mut contents = String::new();

  file.read_to_string(&mut contents).map_err(CasErr::from)?;
  toml::from_str(&contents)
    .map_err(|e| CasErr::InvalidConfigToml(format!("Could not parse connections.toml: {}", e)))
}

fn write_conns(table: HashMap<String, PartialConnOpts>) -> Result<(), CasErr> {
  let dirs = ProjectDirs::from("", "", "Casql").ok_or(CasErr::NoHomeDir)?;
  let conns_path = dirs.config_dir().join(FILENAME);
  let mut file = OpenOptions::new()
    .write(true)
    .truncate(true)
    .open(conns_path)?;

  let toml = toml::to_string(&table)
    .map_err(|e| CasErr::InvalidConfigToml(format!("Could not serialize config to TOML: {}", e)))?;
  let bytes = toml.into_bytes();
  file.write_all(&bytes).map_err(CasErr::from)
}

pub fn save(name: String, opts: PartialConnOpts) -> Result<(), CasErr> {
  let mut connection_map = read_conns()?;
  connection_map.insert(name.clone(), opts);
  write_conns(connection_map)?;
  println!("Connection {} saved.", name);
  Ok(())
}

pub fn list() -> Result<(), CasErr> {
  let connection_map = read_conns()?;
  let keys = connection_map.keys();
  let stdout = io::stdout();
  let mut handle = stdout.lock();
  writeln!(handle, "Connections")?;
  keys
    .into_iter()
    .fold(Ok(()), |acc, k| acc.and(writeln!(handle, "\t{}", k)))
    .map_err(CasErr::from)
}

pub fn describe(name: String) -> Result<(), CasErr> {
  let connection_map = read_conns()?;
  match connection_map.get(&name) {
    Some(connection) => {
      println!("Connection: {:?}", connection);
      Ok(())
    }
    None => Err(CasErr::ConnNotFound),
  }
}

// pub fn load(name: &str) -> PartialConnOpts {
//   let mut connection_map = read_conns();
//   match connection_map.remove(name) { // is there a better way to get the owned value?
//     Some(connection) => connection,
//     None => panic!("Connection not found."),
//   }
// }

pub fn delete(name: String) -> Result<(), CasErr> {
  let mut connection_map = read_conns()?;
  match connection_map.remove(&name) {
    Some(_) => {
      write_conns(connection_map)?;
      println!("Connection deleted: {}", name);
      Ok(())
    }
    None => Err(CasErr::ConnNotFound),
  }
}
