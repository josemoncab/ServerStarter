use serde::Deserialize;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use tokio_stream::StreamExt;

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
    let url = format!("{}/{}/versions/{}/builds/{}/downloads/{jar}",PAPER_API, software, version,
                      build);

    println!("{}", url);
    let mut file = File::create(format!("./jars/{jar}")).await.expect("Failed to create the file");
    let mut stream = reqwest::get(url).await.expect("Failed to get the builds").bytes_stream();

    while let Some(chunk_result) = stream.next().await {
        let chunk = chunk_result.expect("");
        file.write_all(&chunk).await.expect("");
    }

    file.flush().await.expect("");
}