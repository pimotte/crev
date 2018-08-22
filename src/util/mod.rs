use common_failures::prelude::*;
use rpassword;
use rprompt;
use std::{env, fs, io, path::PathBuf};

pub mod serde;

use app_dirs::{app_root, get_app_root, AppDataType, AppInfo};

const APP_INFO: AppInfo = AppInfo {
    name: "crev",
    author: "Dawid Ciężarkiewicz",
};

pub fn user_config_path() -> Result<PathBuf> {
    Ok(app_root(AppDataType::UserConfig, &APP_INFO)?.join("crev.yaml"))
}

pub fn project_dir_init() -> Result<()> {
    Ok(fs::create_dir_all(PROJECT_DIR_NAME)?)
}

const PROJECT_DIR_NAME: &str = ".crev";

#[derive(Fail, Debug)]
#[fail(display = "`.crew` project dir not found")]
struct ProjectDirNotFound;

pub fn project_dir_find() -> Result<PathBuf> {
    let mut path = PathBuf::from(".").canonicalize()?;
    loop {
        if path.join(PROJECT_DIR_NAME).is_dir() {
            return Ok(path.join(PROJECT_DIR_NAME));
        }
        path = if let Some(parent) = path.parent() {
            parent.to_owned()
        } else {
            return Err(ProjectDirNotFound.into());
        }
    }
}

pub fn read_passphrase() -> io::Result<String> {
    if let Ok(pass) = env::var("CREV_PASSPHRASE") {
        eprint!("Using passphrase set in CREV_PASSPHRASE\n");
        return Ok(pass);
    }
    eprint!("Enter passphrase to unlock: ");
    rpassword::read_password()
}

pub fn read_new_passphrase() -> io::Result<String> {
    if let Ok(pass) = env::var("CREV_PASSPHRASE") {
        eprint!("Using passphrase set in CREV_PASSPHRASE\n");
        return Ok(pass);
    }
    loop {
        eprint!("Enter new passphrase: ");
        let p1 = rpassword::read_password()?;
        eprint!("Enter new passphrase again: ");
        let p2 = rpassword::read_password()?;
        if p1 == p2 {
            return Ok(p1);
        }
        eprintln!("\nPassphrases don't match, try again.");
    }
}