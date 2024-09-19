use anyhow::Result;
use log::debug;
use log::error;
use log::info;
use sevenz_rust;
use std::process::Command;

use crate::utils;
use crate::DEFAULT_SHA256_SUFFIX;

fn install_depends(package_name: &str, package_version: &str) -> Result<()> {
    let find_links_str = format!("--find-links=./{}_{}/", package_name, package_version);
    let package = format!("{}=={}", package_name, package_version);
    let c = Command::new("pip")
        .arg("install")
        .arg("--no-index")
        .arg(find_links_str)
        .arg(package)
        .output()?;

    debug!("{}", String::from_utf8_lossy(&c.stdout));
    Ok(())
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
    let poitfile_name_split: Vec<&str> = poitfile_name.split(".poit").collect();
    let target_dir = if poitfile_name_split.len() > 0 {
        poitfile_name_split[0].to_string()
    } else {
        panic!("filename error, standard files should end with poit!");
    };

    // get package version
    let (package_name, package_version) = if package_version == "null" {
        let poit_split_1: Vec<&str> = poitfile_name.split(".poit").collect();
        if poit_split_1.len() > 0 {
            let poit_split_2: Vec<&str> = poit_split_1[0].split("_").collect();
            if poit_split_2.len() == 2 {
                (poit_split_2[0], poit_split_2[1])
            } else {
                panic!("please check your poit file!");
            }
        } else {
            panic!("please check your poit file!");
        }
    } else {
        (poitfile_name, package_version)
    };

    // decompress 7z package
    info!("decompress poit...");
    // let dest = format!("./{}", target_dir);
    utils::create_dir(&target_dir)?;
    sevenz_rust::decompress_file(poitfile_name, &target_dir)?;

    // install all
    info!("installing {package_name}[{package_version}]");
    install_depends(package_name, package_version)?;

    // delete decompress dir
    info!("removing tmp dir...");
    utils::remove_dir(&target_dir)?;
    info!("done!");

    Ok(())
}
