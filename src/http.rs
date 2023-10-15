use std::io::Cursor;
use serde::{Deserialize};

#[derive(Deserialize, Debug)]
struct ProjectInfo {
    project_id: String,
    project_name: String,
    version_groups: Vec<String>,
    versions: Vec<String>
}

#[derive(Deserialize, Debug)]
struct VersionInfo {
    project_id: String,
    project_name: String,
    version: String,
    builds: Vec<i32>
}
const PAPER_API: &str = "https://api.papermc.io/v2/projects";

pub async fn get_versions(software: &str) -> Vec<String> {
    let response = reqwest::get(format!("{}/{}", PAPER_API, software)).await.expect("Failed \
    to make a GET request").json::<ProjectInfo>()
        .await.expect("Failed to parse response body to JSON");
    response.versions
}

pub async fn get_builds(software: &str, version: &str) -> Vec<String> {
    let response = reqwest::get(format!("{}/{}/versions/{}", PAPER_API, software, version)).await
        .expect("Failed to get the builds").json::<VersionInfo>().await.expect("Failed to parse \
        to JSON");
    response.builds.iter().map(|x| x.to_string()).collect()
}

pub async fn download_file(software: &str, version: &str, build: &str) {
    let jar = format!("{software}-{version}-{build}.jar");
    let mut file = std::fs::File::create(format!("./jars/{jar}")).expect("Failed to create the \
    file");
    let response = reqwest::get(format!("{}/{}/versions/{}/builds/{}/dowloads/{jar}",PAPER_API, software, version, build)).await
        .expect("Failed to get the builds");
    let mut content = Cursor::new(response.bytes().await.expect("Failed to read bytes from \
    response"));
    std::io::copy(&mut content, &mut file).expect("Failed to create the file");
}