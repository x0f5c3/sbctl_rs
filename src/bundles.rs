use anyhow::{Context, Result};
use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
use serde::{Deserialize, Serialize};

use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub enum Microcode {
    Intel(PathBuf),
    AMD(PathBuf),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Bundle {
    pub output: PathBuf,
    pub microcode: Microcode,
    pub kernel_image: PathBuf,
    pub initramfs: PathBuf,
    pub cmdline: PathBuf,
    pub splash: PathBuf,
    pub os_release: PathBuf,
    pub efi_stub: PathBuf,
    pub esp: PathBuf,
}

pub struct Bundles {
    db: PickleDb,
    dbpath: PathBuf,
}

impl Bundles {
    pub fn new(dbpath: PathBuf) -> Result<Bundles> {
        let db = PickleDb::new(
            dbpath.as_path(),
            PickleDbDumpPolicy::AutoDump,
            SerializationMethod::Bin,
        );
        Ok(Bundles { db, dbpath })
    }
}
