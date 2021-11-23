use crate::args::PartialConnectionParams;
use crate::cas_err::CasErr;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::Write;
use std::path;

const CONFIG_FILENAME: &'static str = "connections.toml";

fn initialise_config() -> Result<fs::File, CasErr> {
    let dir_path = config_dir_path()?;
    fs::DirBuilder::new()
        .recursive(true)
        .create(&dir_path)
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

fn write_conns(conns: HashMap<String, PartialConnectionParams>) -> Result<(), CasErr> {
    let path = config_path()?;
    let mut file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(path)?;
    let toml = toml::to_string(&conns)
        .map_err(|e| CasErr::ConfigsErr(format!("Could not serialize config to TOML: {}", e)))?;
    file.write_all(&toml.into_bytes()).map_err(CasErr::from)
}

pub fn save(name: String, params: PartialConnectionParams) -> Result<(), CasErr> {
    let mut connection_map = match read_conns()? {
        Some(conns) => conns,
        None => {
            initialise_config()?;
            HashMap::new()
        }
    };
    let success_msg = format!("Connection {} saved.\n", &name);
    connection_map.insert(name, params);
    write_conns(connection_map)?;
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    handle.write(success_msg.as_bytes())?;
    Ok(())
}

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

fn config_dir_path() -> Result<path::PathBuf, CasErr> {
    let mut config_dir = dirs::config_dir().ok_or(CasErr::ConfigsErr(
        "Cannot locate config directory to save connections.".to_owned(),
    ))?;
    config_dir.push("casql");
    Ok(config_dir)
}

fn config_path() -> Result<path::PathBuf, CasErr> {
    config_dir_path().map(|mut path| {
        path.push(CONFIG_FILENAME);
        path
    })
}
