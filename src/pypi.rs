use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Info {
    pub author: String,
    pub author_email: String,
    pub bugtrack_url: Option<String>,
    pub classifiers: Vec<String>,
    pub description: String,
    pub description_content_type: String,
    pub docs_url: Option<String>,
    pub download_url: String,
    pub downloads: HashMap<String, i32>,
    pub home_page: String,
    pub keywords: String,
    pub license: String,
    pub maintainer: String,
    pub maintainer_email: String,
    pub name: String,
    pub package_url: String,
    pub platform: Option<String>,
    pub project_url: String,
    pub project_urls: HashMap<String, String>,
    pub release_url: String,
    pub requires_dist: Vec<String>,
    pub requires_python: String,
    pub summary: String,
    pub version: String,
    pub yanked: bool,
    pub yanked_reason: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Digests {
    pub blake2b_256: String,
    pub md5: String,
    pub sha256: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Urls {
    pub comment_text: String,
    pub digests: Digests,
    pub downloads: i32,
    pub filename: String,
    pub has_sig: bool,
    pub md5_digest: String,
    pub packagetype: String,
    pub python_version: String,
    pub requires_python: String,
    pub size: usize,
    pub upload_time: String,
    pub upload_time_iso_8601: String,
    pub url: String,
    pub yanked: bool,
    pub yanked_reason: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Pypi {
    pub info: Info,
    pub last_serial: usize,
    pub urls: Vec<Urls>,
    pub vulnerabilities: Vec<String>,
}

// async fn resolve_depends_by_web(
//     package_name: &str,
//     package_version: &str,
// ) -> Result<(), Box<dyn Error>> {
//     let request_url = format!(
//         "https://pypi.org/pypi/{}/{}/json",
//         package_name, package_version
//     );
//     let resp = reqwest::get(&request_url).await?.json::<Pypi>().await?;
//     let info = resp.info;
//     // println!("info {:?}", info);
//     let requires_dist = info.requires_dist;
//     for r in requires_dist {
//         println!("{:?}", r);
//     }
//     Ok(())
// }

#[tokio::test]
async fn test_reqwest() {
    use reqwest;
    use std::error::Error;
    async fn test() -> Result<(), Box<dyn Error>> {
        let package_name = "python-telegram-bot";
        let package_version = "20.1";
        let request_url = format!(
            "https://pypi.org/pypi/{}/{}/json",
            package_name, package_version
        );
        let resp = reqwest::get(&request_url).await?.json::<Pypi>().await?;
        let info = resp.info;
        // println!("info {:?}", info);
        let requires_dist = info.requires_dist;
        for r in requires_dist {
            println!("{:?}", r);
        }
        Ok(())
    }
    test().await.unwrap();
}
