use anyhow::{Context, Result};
use nix::unistd::{access, AccessFlags};
use static_init::dynamic;
use std::path::PathBuf;

const RSA_KEY_SIZE: u32 = 4096;

#[dynamic]
static DATAPATH: PathBuf = PathBuf::from("/usr/share/secureboot");

#[dynamic]
static KEYPATH: PathBuf = DATAPATH.join("keys");

#[dynamic]
static PKKEY: PathBuf = KEYPATH.join("PK").join("PK.key");

#[dynamic]
static PKCERT: PathBuf = KEYPATH.join("PK").join("PK.pem");

#[dynamic]
static KEKKEY: PathBuf = KEYPATH.join("KEK").join("KEK.key");

#[dynamic]
static KEKCERT: PathBuf = KEYPATH.join("KEK").join("KEK.pem");

#[dynamic]
static DBKEY: PathBuf = KEYPATH.join("db").join("db.key");

#[dynamic]
static DBCERT: PathBuf = KEYPATH.join("db").join("db.pem");

#[dynamic]
static DBPATH: PathBuf = DATAPATH.join("files.db");

#[dynamic]
static GUIDPATH: PathBuf = DATAPATH.join("GUID");

pub fn can_verify_files() -> Result<()> {
    access(DBCERT.as_path(), AccessFlags::R_OK).context("Cannot access")
}
