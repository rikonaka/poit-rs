use clap::Parser;
use serde::{Deserialize, Serialize};

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
    /// For packaging pip dependencies
    #[arg(short, long, default_value = "null")]
    pack: String,

    /// Install the packaged pip dependencies
    #[arg(short, long, default_value = "null")]
    install: String,

    /// Specify the version of the pip package
    #[arg(short, long, default_value = "null")]
    version: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SerdeConfig {
    data: Vec<String>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    #[test]
    fn append() {
        let mut a = vec![0, 1, 2];
        let mut b = vec![];
        a.append(&mut b);
        // println!("{:?}", a);
        assert_eq!(a, vec![0, 1, 2]);
    }
}

fn main() {
    let args = Args::parse();
    if args.pack != "null" {
        pack::pack_deb(&args.pack, &args.version);
    } else if args.install != "null" {
        install::install_wheel(&args.install, &args.version);
    } else {
        println!("Use --help for more infomation");
    }
}
