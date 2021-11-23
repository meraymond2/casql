use crate::args::PartialConnectionParams;
use crate::cas_err::CasErr;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::Write;
use std::path;

const CONFIG_FILENAME: &'static str = "connections.toml";

fn initialise(dir_path: &path::PathBuf) -> Result<fs::File, CasErr> {
    fs::DirBuilder::new()
        .recursive(true)
        .create(dir_path)
        .and_then(|_| {
            fs::OpenOptions::new()
                .read(true)
                .write(true)
                .create_new(true)
                .open(dir_path.join(CONFIG_FILENAME))
        })
        .map_err(CasErr::from)
}

fn read_conns() -> Result<Option<HashMap<String, PartialConnectionParams>>, CasErr> {
    let path = config_path()?;
    let maybe_text = match fs::read_to_string(path) {
        Ok(string) => Ok(Some(string)),
        Err(ref err) if err.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(err) => Err(CasErr::from(err)),
    }?;
    if let Some(text) = maybe_text {
        match toml::from_str::<HashMap<String, PartialConnectionParams>>(text.as_str()) {
            Ok(x) => Ok(Some(x)),
            Err(e) => Err(CasErr::ConfigsErr(format!(
                "Could not parse config file: {}",
                e
            ))),
        }
    } else {
        Ok(None)
    }
}

// fn write_conns(table: HashMap<String, PartialConnOpts>) -> Result<(), CasErr> {
//     let dirs = ::from("", "", "Casql").ok_or(CasErr::NoHomeDir)?;
//     let conns_path = dirs.config_dir().join(FILENAME);
//     let mut file = OpenOptions::new()
//         .write(true)
//         .truncate(true)
//         .open(conns_path)?;
//
//     let toml = toml::to_string(&table).map_err(|e| {
//         CasErr::InvalidConfigToml(format!("Could not serialize config to TOML: {}", e))
//     })?;
//     let bytes = toml.into_bytes();
//     file.write_all(&bytes).map_err(CasErr::from)
// }

// pub fn save(name: String, opts: PartialConnOpts) -> Result<(), CasErr> {
//     let mut connection_map = read_conns()?;
//     connection_map.insert(name.clone(), opts);
//     write_conns(connection_map)?;
//     println!("Connection {} saved.", name);
//     Ok(())
// }

pub fn list() -> Result<(), CasErr> {
    let conns = read_conns()?;
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    handle.write("Connections:\n".as_bytes())?;
    if let Some(conns) = conns {
        for key in conns.keys() {
            handle.write("  ".as_bytes())?;
            handle.write(key.as_bytes())?;
            handle.write("\n".as_bytes())?;
        }
    }
    Ok(())
}
//
// pub fn describe(name: String) -> Result<(), CasErr> {
//     let connection_map = read_conns()?;
//     match connection_map.get(&name) {
//         Some(connection) => {
//             println!("Connection {}:\n{}", name, connection);
//             Ok(())
//         }
//         None => Err(CasErr::ConnNotFound),
//     }
// }
//
// pub fn load(name: String) -> Result<PartialConnOpts, CasErr> {
//     let mut connection_map = read_conns()?;
//     match connection_map.remove(&name) {
//         Some(connection) => Ok(connection),
//         None => Err(CasErr::ConnNotFound),
//     }
// }
//
// pub fn delete(name: String) -> Result<(), CasErr> {
//     let mut connection_map = read_conns()?;
//     match connection_map.remove(&name) {
//         Some(_) => {
//             write_conns(connection_map)?;
//             println!("Connection deleted: {}", name);
//             Ok(())
//         }
//         None => Err(CasErr::ConnNotFound),
//     }
// }

fn config_path() -> Result<path::PathBuf, CasErr> {
    let mut config_dir = dirs::config_dir().ok_or(CasErr::ConfigsErr(
        "Cannot locate config directory to save connections.".to_owned(),
    ))?;
    // let mut path = PathBuf::new().push();
    config_dir.push("casql");
    config_dir.push(CONFIG_FILENAME);
    Ok(config_dir)
}
