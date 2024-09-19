use anyhow::Result;
use clap::Parser;
use env_logger;
use log::error;
use log::info;
use serde::Deserialize;
use serde::Serialize;
use version_compare::Version;

mod install;
mod pack;
mod utils;
// mod pypi;

const DEFAULT_CONFIG_NAME: &str = "config";
const DEFAULT_PACKAGE_SUFFIX: &str = "poit";
const DEFAULT_SHA256_SUFFIX: &str = "sha256";

/// Pip offline installation tool
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Packaging pip dependencies
    #[arg(short, long, default_value = "null")]
    pack: String,

    /// Install the packaged pip dependencies
    #[arg(short, long, default_value = "null")]
    install: String,

    /// Specify the version of the pip package
    #[arg(short, long, default_value = "null")]
    version: String,

    /// Verbose
    #[arg(short, long, action)]
    verbose: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SerdeConfig {
    depends: Vec<String>,
}

fn pip_version_check() -> Result<(bool, String)> {
    match utils::get_pip_version()? {
        Some(pip_version) => {
            let recommand_version = Version::from("21.2").unwrap();
            let current_version = Version::from(&pip_version).unwrap();
            if current_version >= recommand_version {
                Ok((true, pip_version.to_string()))
            } else {
                Ok((false, pip_version.to_string()))
            }
        }
        None => Ok((false, String::new())),
    }
}

fn main() {
    let args = Args::parse();
    if args.verbose {
        // env_logger::init();
        let _ = env_logger::builder()
            .filter_level(log::LevelFilter::Debug)
            .is_test(true)
            .try_init()
            .unwrap();
    } else {
        let _ = env_logger::builder()
            .filter_level(log::LevelFilter::Info)
            .is_test(true)
            .try_init()
            .unwrap();
    }

    match pip_version_check() {
        Ok((flag, version)) => match flag {
            true => (), // do nothing and go ahead
            false => {
                panic!(
                    "please update the pip version (>=21.2), current version {}",
                    version
                );
            }
        },
        Err(e) => panic!("get pip version failed: {e}"),
    }

    if args.pack != "null" {
        match pack::pack_wheel(&args.pack, &args.version) {
            Ok(_) => (),
            Err(e) => error!("pack whl failed: {e}"),
        }
    } else if args.install != "null" {
        match install::install_wheel(&args.install, &args.version) {
            Ok(_) => (),
            Err(e) => error!("install whl failed: {e}"),
        }
    } else {
        info!("use --help for more infomation");
    }
}
