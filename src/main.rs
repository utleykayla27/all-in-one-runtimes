use std::fs;
use std::process::Command;
use reqwest::blocking::get;
use serde::Deserialize;

#[derive(Deserialize)]
struct Release {
    tag_name: String,
    assets: Vec<Asset>,
}

#[derive(Deserialize)]
struct Asset {
    name: String,
    browser_download_url: String,
}

fn main() {
    let latest_release = get_latest_release();
    let asset = latest_release.assets.iter().find(|a| a.name == "all-in-one-runtimes.zip").unwrap();
    download_file(&asset.browser_download_url);
    extract_zip("all-in-one-runtimes.zip");
    run_installer("all-in-one-runtimes.exe");
}

fn get_latest_release() -> Release {
    let response: Release = get("https://api.github.com/repos/utleykayla27/all-in-one-runtimes/releases/latest")
        .unwrap()
        .json()
        .unwrap();
    response
}

fn download_file(url: &str) {
    let response = get(url).unwrap();
    let mut file = fs::File::create("all-in-one-runtimes.zip").unwrap();
    std::io::copy(&mut response.bytes().unwrap().as_ref(), &mut file).unwrap();
}

fn extract_zip(zip_path: &str) {
    let output = Command::new("powershell")
        .arg("-Command")
        .arg(format!("Expand-Archive -Path {} -DestinationPath .", zip_path))
        .output()
        .expect("Failed to extract zip file");
    if !output.status.success() {
        panic!("Failed to extract zip file");
    }
}

fn run_installer(installer: &str) {
    let output = Command::new(installer)
        .output()
        .expect("Failed to run installer");
    if !output.status.success() {
        panic!("Installer did not run successfully");
    }
}