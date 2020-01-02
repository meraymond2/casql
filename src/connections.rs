use crate::errors::CasErr;
use crate::opts::PartialConnOpts;

use directories::ProjectDirs;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::fs::{DirBuilder, File};
use std::io::prelude::*;
use std::io::ErrorKind;
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
      Err(e) => return Err(CasErr::from(e)),
    },
    Err(ref e) if e.kind() == ErrorKind::PermissionDenied => return Err(CasErr::FilePermissions),
    Err(e) => return Err(CasErr::from(e)),
  };
  let mut contents = String::new();

  file.read_to_string(&mut contents).unwrap();
  toml::from_str(&contents).map_err(|e| CasErr::InvalidConfigToml(format!("{}", e)))
}

// fn write_conns(table: HashMap<String, PartialConnOpts>) {
//   let dirs = ProjectDirs::from("", "", "Casql").expect("Dirs failed");
//   let conns_path = dirs.config_dir().join(FILENAME);
//   let mut file = OpenOptions::new()
//     .write(true)
//     .truncate(true)
//     .open(conns_path)
//     .expect("Should be able to open file as writeable");

//   let toml = toml::to_string(&table).expect("Config should be tomlable");
//   let bytes = toml.into_bytes();
//   file
//     .write_all(&bytes)
//     .expect("File should write correctly.");
// }

// pub fn save(name: &str, opts: PartialConnOpts) {
//   let mut connection_map = read_conns();

//   connection_map.insert(name.to_owned(), opts);
//   write_conns(connection_map);
//   println!("Connection {} saved.", name);
// }

pub fn list() {
  match read_conns() {
    Ok(connection_map) => {
      let keys = connection_map.keys();

      println!("Connections:");
      for k in keys {
        println!("\t{}", k);
      }
    }
    Err(e) => eprintln!("{}", e),
  }
}

// pub fn describe(name: &str) {
//   let connection_map = read_conns();
//   match connection_map.get(name) {
//     Some(connection) => println!("Connection: {:?}", connection),
//     None => println!("Connection not found."),
//   }
// }

// pub fn load(name: &str) -> PartialConnOpts {
//   let mut connection_map = read_conns();
//   match connection_map.remove(name) { // is there a better way to get the owned value?
//     Some(connection) => connection,
//     None => panic!("Connection not found."),
//   }
// }

// pub fn delete(name: &str) {
//   let mut connection_map = read_conns();
//   match connection_map.remove(name) {
//     Some(_) => {
//       write_conns(connection_map);
//       println!("Connection deleted: {}", name);
//     }
//     None => println!("Connection not found."),
//   }
// }
