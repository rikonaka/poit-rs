use anyhow::Result;
use log::debug;
use log::error;
use log::info;
use sevenz_rust;
use std::process::Command;

use crate::utils;
use crate::SerdeConfig;
use crate::DEFAULT_CONFIG_NAME;
use crate::DEFAULT_PACKAGE_SUFFIX;
use crate::DEFAULT_SHA256_SUFFIX;

fn download_depends(
    package_name: &str,
    package_version: &str,
    python_version: &str,
    target_dir: &str,
) -> Result<()> {
    // package_name: python-telegram-bot
    // package_version: 20.3
    let mut command = Command::new("pip");
    let command = command.args(["download", "-d", target_dir]); // download to `target_dir`

    let package_name = if package_version == "null" {
        package_name.to_string()
    } else {
        let package_name_with_version = format!("{}=={}", package_name, package_version);
        package_name_with_version
    };
    debug!("package_name: {package_name}");

    let command = if python_version == "null" {
        command
    } else {
        command.args(["--python-version", python_version, "--only-binary=:all:"])
    };

    let command = command.arg(package_name).output()?;
    debug!(
        "download output: {}",
        String::from_utf8_lossy(&command.stdout)
    );

    Ok(())
}

fn package_name_check(package_name: &str) -> bool {
    if package_name.contains("~")
        || package_name.contains(">")
        || package_name.contains("<")
        || package_name.contains("=")
    {
        error!("please use --version to specify the version of the pip package");
        false
    } else {
        true
    }
}

pub fn pack_wheel(package_name: &str, package_version: &str, python_version: &str) -> Result<()> {
    match package_name_check(package_name) {
        true => (),
        false => return Ok(()),
    }

    match utils::create_dir(package_name) {
        Ok(_) => info!("create tmp dir success!"),
        Err(e) => {
            error!("create tmp dir failed!");
            return Err(e.into());
        }
    }

    info!("downloading...");
    let target_dir = package_name;
    download_depends(package_name, package_version, python_version, target_dir)?;

    // serde config
    let serde_config = SerdeConfig {
        python_version: python_version.to_string(),
        package_version: package_version.to_string(),
    };
    utils::serde_to_file(DEFAULT_CONFIG_NAME, serde_config)?;
    utils::move_file_to_dir(package_name, DEFAULT_CONFIG_NAME)?;

    // compress
    info!("saving...");
    let dest = format!("{}.{}", package_name, DEFAULT_PACKAGE_SUFFIX);
    sevenz_rust::compress_to_path(package_name, &dest)?;

    // sha256 hash
    info!("hashing...");
    let hash_str = utils::file_sha256(&dest)?;
    let hash_filename = format!("{}.{}", dest, DEFAULT_SHA256_SUFFIX);
    let _ = utils::write_to_file(&hash_filename, &hash_str);

    // clean dir
    info!("removing tmp dir...");
    utils::remove_dir(package_name)?;
    info!("done!");

    Ok(())
}
