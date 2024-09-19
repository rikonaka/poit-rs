use anyhow::Result;
use log::debug;
use log::error;
use log::info;
use log::warn;
use sevenz_rust;
use std::process::Command;
use version_compare::Version;

use crate::utils;
use crate::DEFAULT_CONFIG_NAME;
use crate::DEFAULT_SHA256_SUFFIX;

fn install_depends(package_name: &str, package_version: &str) -> Result<()> {
    let find_links = format!("--find-links=./{package_name}/");
    let package = if package_version != "null" {
        format!("{}=={}", package_name, package_version)
    } else {
        package_name.to_string()
    };
    let mut command = Command::new("pip");
    let command = command
        .args(["install", "--no-index", &find_links, &package])
        .output()?;
    debug!(
        "install output: {}",
        String::from_utf8_lossy(&command.stdout)
    );
    Ok(())
}

fn check_python_version(python_version: &str) -> Result<(bool, String)> {
    let command = Command::new("python3").arg("--version").output()?;
    let command_str = String::from_utf8_lossy(&command.stdout);
    debug!("python version output: {}", command_str);
    let command_str_split: Vec<&str> = command_str.split(" ").collect();
    let command_python_version = command_str_split[1];

    let python_version_split: Vec<&str> = python_version.split(".").collect();
    let python_version = python_version_split[0..2].join(".");
    let local_python_version_split: Vec<&str> = command_python_version.split(".").collect();
    let local_python_version = local_python_version_split[0..2].join(".");
    let l_version = Version::from(&python_version).unwrap();
    let r_version = Version::from(&local_python_version).unwrap();

    if l_version == r_version {
        Ok((true, command_python_version.trim().to_string()))
    } else {
        Ok((false, command_python_version.trim().to_string()))
    }
}

pub fn install_wheel(poitfile_name: &str, package_version: &str) -> Result<()> {
    // sha256 check
    info!("checking...");
    let hash_filename = format!("{}.{}", poitfile_name, DEFAULT_SHA256_SUFFIX);
    let hash_str = utils::file_sha256(poitfile_name)?;
    let contents = utils::read_file_bytes(&hash_filename)?;
    let contents = String::from_utf8_lossy(&contents).to_string();
    if hash_str.trim() != contents.trim() {
        error!("calc hash: {hash_str}, file hash: {contents}");
        panic!("check sha256 failed!");
    } else {
        info!("check sha256 success...");
    }

    // get target dir name
    let poitfile_name_split: Vec<&str> = poitfile_name.split(".").collect();
    let target_dir = if poitfile_name_split.len() >= 2 {
        poitfile_name_split[0].to_string()
    } else {
        panic!("wrong file name, standard files should end with aoit");
    };

    // decompress 7z package
    info!("decompress poit...");
    // let dest = format!("./{}", target_dir);
    utils::create_dir(&target_dir)?;
    sevenz_rust::decompress_file(poitfile_name, &target_dir)?;

    let config_file_path = format!("{}/{}", target_dir, DEFAULT_CONFIG_NAME);
    let serde_config = utils::serde_from_file(&config_file_path)?;
    match check_python_version(&serde_config.python_version)? {
        (true, _) => (),
        (false, local_version) => warn!(
            "package python version not match, package version {}, local version {}",
            serde_config.python_version, local_version
        ),
    }

    // install all
    info!("installing: {}", target_dir);
    install_depends(&target_dir, package_version)?;

    // delete decompress dir
    info!("removing tmp dir...");
    utils::remove_dir(&target_dir)?;
    info!("done!");

    Ok(())
}
