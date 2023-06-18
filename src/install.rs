use sevenz_rust;
use std::process::Command;

use crate::utils;
// use crate::DEFAULT_CONFIG_NAME;
use crate::DEFAULT_SHA256_SUFFIX;

fn install_depends(package_path: &str, package_name: &str) {
    let find_links_str = format!("--find-links=./{}/", package_path);
    let _ = Command::new("pip")
        .arg("install")
        .arg("--no-index")
        .arg(find_links_str)
        .arg(package_name)
        .output()
        .expect("failed to excute pip install");

    // println!("{}", String::from_utf8_lossy(&c.stdout));
}

pub fn install_wheel(poitfile_name: &str) {
    // poitfile_name: vim.poit

    // sha256 check
    println!("Checking...");
    let hash_filename = format!("{}.{}", poitfile_name, DEFAULT_SHA256_SUFFIX);
    let hash_str = utils::file_sha256(poitfile_name).unwrap();
    let contents = utils::read_file_bytes(&hash_filename).unwrap();
    let contents = String::from_utf8_lossy(&contents).to_string();
    if hash_str.trim() != contents.trim() {
        panic!("Check sha256 failed");
    } else {
        println!("Check sha256 success");
    }

    // get target dir name
    let poitfile_name_split: Vec<&str> = poitfile_name.split(".").collect();
    let target_dir = if poitfile_name_split.len() >= 2 {
        poitfile_name_split[0].to_string()
    } else {
        panic!("Filename error, Standard files should end with poit");
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
    install_depends(&target_dir, &target_dir);

    // delete decompress dir
    println!("Removing tmp dir...");
    utils::remove_dir(&target_dir);

    println!("Done");
}
