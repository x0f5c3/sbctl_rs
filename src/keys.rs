use anyhow::{Context, Result};
use nix::unistd::{access, AccessFlags};
use rcgen::{Certificate, KeyUsagePurpose};
use rust_embed::RustEmbed;
use static_init::dynamic;
use std::path::PathBuf;

#[derive(RustEmbed)]
#[folder = "certs/"]
struct DefaultKeys;

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

pub struct KeyCert {
    cert: Certificate,
    priv_key: String,
}

impl KeyCert {
    pub fn new(name: String) -> Result<Self> {
        let from_date = time::OffsetDateTime::now_utc();
        let to_date = from_date + time::Duration::weeks(260);
        let mut cert_params = rcgen::CertificateParams::new(vec![name]);
        cert_params.not_before = from_date;
        cert_params.alg = &rcgen::PKCS_RSA_SHA256;
        cert_params.not_after = to_date;
        cert_params.key_usages = vec![KeyUsagePurpose::DigitalSignature];
        let cert = rcgen::Certificate::from_params(cert_params)?;
        let priv_key = cert.serialize_private_key_pem();
        Ok(KeyCert { cert, priv_key })
    }
    pub fn enroll
}

pub fn can_verify_files() -> Result<()> {
    access(DBCERT.as_path(), AccessFlags::R_OK).context("Cannot access")
}
