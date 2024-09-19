use anyhow::Result;
use glob::glob;
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
    target_dir: &str,
) -> Result<Vec<String>> {
    // package_name: python-telegram-bot
    // package_version: 20.3
    let mut download_whl = Vec::new();
    if package_version == "null" {
        let command = Command::new("pip")
            .arg("download")
            .arg(package_name)
            .output()?;
        debug!(
            "download output: {}",
            String::from_utf8_lossy(&command.stdout)
        );
    } else {
        let package_name_with_version = format!("{}=={}", package_name, package_version);
        let command = Command::new("pip")
            .arg("download")
            .arg(package_name_with_version)
            .output()?;
        debug!(
            "download output: {}",
            String::from_utf8_lossy(&command.stdout)
        );
    }

    for entry in glob("*.whl").expect("failed to read glob pattern") {
        match entry {
            Ok(path) => {
                // println!("{:?}", path.display());
                let package_full_name = path.to_string_lossy().to_string();
                debug!("move {package_full_name} to {target_dir}");
                utils::move_file_to_dir(&target_dir, &package_full_name)?;
                download_whl.push(package_full_name);
            }
            Err(e) => return Err(e.into()),
        }
    }
    Ok(download_whl)
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

pub fn pack_wheel(package_name: &str, package_version: &str) -> Result<()> {
    match package_name_check(package_name) {
        true => (),
        false => return Ok(()),
    }

    match utils::create_dir(package_name) {
        Ok(_) => info!("create tmp dir success"),
        Err(e) => {
            error!("create tmp dir failed");
            return Err(e.into());
        }
    }

    info!("downloading...");
    let depends_all_vec = download_depends(package_name, package_version, package_name)?;
    debug!("depends all: {:?}", depends_all_vec);

    // serde config
    let serde_config = SerdeConfig {
        depends: depends_all_vec,
    };
    utils::serde_to_file(DEFAULT_CONFIG_NAME, serde_config)?;
    utils::move_file_to_dir(package_name, DEFAULT_CONFIG_NAME)?;

    // compress
    info!("saving...");
    let dest = if package_version == "null" {
        format!("{}.{}", package_name, DEFAULT_PACKAGE_SUFFIX)
    } else {
        format!(
            "{}_{}.{}",
            package_name, package_version, DEFAULT_PACKAGE_SUFFIX
        )
    };
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
