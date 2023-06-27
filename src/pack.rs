use glob::glob;
use sevenz_rust;
use std::process::Command;

use crate::utils;
use crate::SerdeConfig;
use crate::DEFAULT_CONFIG_NAME;
use crate::DEFAULT_PACKAGE_SUFFIX;
use crate::DEFAULT_SHA256_SUFFIX;

fn download_depends(package_name: &str, package_version: &str, target_dir: &str) -> Vec<String> {
    // package_name: python-telegram-bot
    // package_version: 20.3
    let mut download_whl = Vec::new();
    if package_version == "null" {
        let command = match Command::new("pip")
            .arg("download")
            .arg(package_name)
            .output()
        {
            Ok(c) => c,
            Err(e) => {
                println!("Please install pip first");
                panic!("Failed to execute pip download: {}", e);
            }
        };
        println!("Downloading: {}", String::from_utf8_lossy(&command.stdout));
    } else {
        let package_name_with_version = format!("{}=={}", package_name, package_version);
        let command = match Command::new("pip")
            .arg("download")
            .arg(package_name_with_version)
            .output()
        {
            Ok(c) => c,
            Err(e) => {
                println!("Please install pip first");
                panic!("Failed to execute pip download: {}", e);
            }
        };
        println!("Downloading: {}", String::from_utf8_lossy(&command.stdout));
    }

    for entry in glob("*.whl").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                // println!("{:?}", path.display());
                let package_full_name = path.to_string_lossy().to_string();
                utils::move_file_to_dir(&package_full_name, &target_dir);
                download_whl.push(package_full_name);
            }
            Err(e) => println!("{:?}", e),
        }
    }
    download_whl
}

fn package_name_check(package_name: &str) {
    if package_name.contains("~=")
        || package_name.contains(">")
        || package_name.contains("<")
        || package_name.contains("=")
    {
        panic!("Please use --version to specify the version of the pip package");
    }
}

pub fn pack_deb(package_name: &str, package_version: &str) {
    package_name_check(package_name);
    // let target_dir = format!("./{}", package_name);
    match utils::create_dir(package_name) {
        true => println!("Create tmp dir success"),
        false => {
            println!("Create tmp dir failed");
            return;
        }
    }

    println!("Downloading...");
    let depends_all_vec = download_depends(package_name, package_version, package_name);
    // println!("{}", package_full_name);

    // serde config
    let serde_config = SerdeConfig {
        data: depends_all_vec,
    };
    utils::serde_to_file(DEFAULT_CONFIG_NAME, serde_config);
    utils::move_file_to_dir(DEFAULT_CONFIG_NAME, package_name);

    // compress
    println!("Saving...");
    let dest = if package_version == "null" {
        format!("{}.{}", package_name, DEFAULT_PACKAGE_SUFFIX)
    } else {
        format!(
            "{}_{}.{}",
            package_name, package_version, DEFAULT_PACKAGE_SUFFIX
        )
    };
    sevenz_rust::compress_to_path(package_name, &dest).expect("compress ok");

    // sha256 hash
    println!("Hashing...");
    let hash_str = utils::file_sha256(&dest).unwrap();
    let hash_filename = format!("{}.{}", dest, DEFAULT_SHA256_SUFFIX);
    let _ = utils::write_to_file(&hash_filename, &hash_str);

    // clean dir
    println!("Removing tmp dir...");
    utils::remove_dir(package_name);
    println!("Done");
}
