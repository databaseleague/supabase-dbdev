use anyhow::Context;
use sqlx::postgres::PgConnection;
use sqlx::Connection;
use std::fs::{File, OpenOptions};
use std::path::Path;

pub async fn get_connection(connection_str: &str) -> anyhow::Result<PgConnection> {
    PgConnection::connect(connection_str)
        .await
        .context("Failed to establish PostgreSQL connection")
}

pub fn is_valid_extension_name(_name: &str) -> bool {
    true
}

pub fn is_valid_version(_version: &str) -> bool {
    true
}

#[cfg(target_family = "unix")]
pub(crate) fn create_file(path: &Path) -> Result<File, std::io::Error> {
    use std::os::unix::fs::OpenOptionsExt;

    let mut options = OpenOptions::new();
    // read/write permissions for owner, none for other
    options.create(true).write(true).mode(0o600);
    options.open(path)
}

#[cfg(not(target_family = "unix"))]
pub(crate) fn create_file(path: &Path) -> Result<File, std::io::Error> {
    let mut options = OpenOptions::new();
    options.create(true).write(true);
    options.open(path)
}
