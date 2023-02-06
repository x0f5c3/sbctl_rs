use anyhow::{Context, Result};
use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
use serde::{Deserialize, Serialize};

use std::path::{PathBuf, Path};

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
    pub fn new<P: AsRef<Path>>(dbpath: P) -> Result<Bundles> {
        let db = PickleDb::new(
            dbpath.as_ref(),
            PickleDbDumpPolicy::AutoDump,
            SerializationMethod::Bin,
        );
        Ok(Bundles { db, dbpath: dbpath.as_ref().to_path_buf() })
    }

    pub fn visit_all(&mut self, mut f: impl FnMut(&str, &mut Bundle) -> Result<()>) -> Result<()> {
        for key in self.db.get_all(){
            let mut bundle = self.db.get(&key).context("Failed to get bundle")?;
            f(&key, &mut bundle)?;
            self.db.set(&key, &bundle).context("Failed to set bundle")?;
        }
        Ok(())
    }
}
