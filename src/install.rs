use sevenz_rust;
use std::process::Command;

use crate::utils;
// use crate::DEFAULT_CONFIG_NAME;
use crate::DEFAULT_SHA256_SUFFIX;

#[test]
fn test_pip_command() {
    let c = Command::new("pip")
        .arg("index")
        .arg("versions")
        .arg("pymysql")
        .output()
        .expect("failed to excute pip install");
    println!("{}", String::from_utf8_lossy(&c.stdout));
}

fn install_depends(package_name: &str, package_version: &str) {
    let find_links_str = format!("--find-links=./{}_{}/", package_name, package_version);
    let package = format!("{}=={}", package_name, package_version);
    let c = Command::new("pip")
        .arg("install")
        .arg("--no-index")
        .arg(find_links_str)
        .arg(package)
        .output()
        .expect("failed to excute pip install");

    println!("{}", String::from_utf8_lossy(&c.stdout));
}

pub fn install_wheel(poitfile_name: &str, package_version: &str) {
    // poitfile_name: vim.poit

    // sha256 check
    println!("Checking...");
    let hash_filename = format!("{}.{}", poitfile_name, DEFAULT_SHA256_SUFFIX);
    let hash_str = utils::file_sha256(poitfile_name).unwrap();
    let contents = utils::read_file_bytes(&hash_filename).unwrap();
    let contents = String::from_utf8_lossy(&contents).to_string();
    if hash_str.trim() != contents.trim() {
        panic!("check sha256 failed");
    } else {
        println!("Check sha256 success...");
    }

    // get target dir name
    let poitfile_name_split: Vec<&str> = poitfile_name.split(".poit").collect();
    let target_dir = if poitfile_name_split.len() > 0 {
        poitfile_name_split[0].to_string()
    } else {
        panic!("filename error, standard files should end with poit");
    };

    // get package version
    let (package_name, package_version) = if package_version == "null" {
        let poit_split_1: Vec<&str> = poitfile_name.split(".poit").collect();
        if poit_split_1.len() > 0 {
            let poit_split_2: Vec<&str> = poit_split_1[0].split("_").collect();
            if poit_split_2.len() == 2 {
                (poit_split_2[0], poit_split_2[1])
            } else {
                panic!("please check your poit file");
            }
        } else {
            panic!("please check your poit file");
        }
    } else {
        (poitfile_name, package_version)
    };

    // decompress 7z package
    println!("Decompress poit...");
    // let dest = format!("./{}", target_dir);
    utils::create_dir(&target_dir);
    sevenz_rust::decompress_file(poitfile_name, &target_dir).expect("complete");

    // let target_config = format!("{}/{}", target_dir, DEFAULT_CONFIG_NAME);
    // let serde_config = utils::serde_from_file(&target_config).unwrap();
    // let _ = serde_config.data;

    // install all
    // println!("{}, {}", package_name, package_version);
    install_depends(package_name, package_version);

    // delete decompress dir
    println!("Removing tmp dir...");
    utils::remove_dir(&target_dir);

    println!("Done");
}
