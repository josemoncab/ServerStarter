use std::path::Path;

mod logger;
mod http;
mod utils;
mod config;

#[tokio::main]
async fn main() {
    let banner = "
███████ ███████ ██████  ██    ██ ███████ ██████  ███████ ████████  █████  ██████  ████████ ███████ ██████
██      ██      ██   ██ ██    ██ ██      ██   ██ ██         ██    ██   ██ ██   ██    ██    ██      ██   ██
███████ █████   ██████  ██    ██ █████   ██████  ███████    ██    ███████ ██████     ██    █████   ██████
     ██ ██      ██   ██  ██  ██  ██      ██   ██      ██    ██    ██   ██ ██   ██    ██    ██      ██   ██
███████ ███████ ██   ██   ████   ███████ ██   ██ ███████    ██    ██   ██ ██   ██    ██    ███████ ██   ██
    ";
    if first_run() {
        println!("\x1b[32m{banner}");
        println!("\t\x1b[32mVersion: 1.0");
        println!("\t\x1b[32mAuthor: josemc");
        println!("\t\x1b[32mGithub: https://github.com/josemoncab/ServerStarter \x1b[0m\n\n");
    }

}

async fn create_server() {
    let scope = utils::user_input("Introduce el tipo de jar que deseas usar",
                                  vec!["proxy", "server"]);
    let mut jar_type = String::new();
    let mut api_versions = vec![];

    if scope == "proxy" {
        jar_type = utils::user_input(
            "Introduce el proxy que deseas usar",
            vec!["velocity"]);
    }
    if scope == "server" {
        jar_type = utils::user_input(
            "Introduce el servidor que deseas usar",
            vec!["paper"]);
    }

    if jar_type == "paper" {
        api_versions = http::get_versions("paper").await;
    }

    if jar_type == "velocity" {
        api_versions = http::get_versions("velocity").await;
    }

    let selected_version = utils::user_input("Introduce la version del servidor ({}):",
                                             api_versions.iter().map(|x| &**x).collect());

    let builds = http::get_builds(&jar_type, &selected_version).await;
    let selected_build = builds.last().expect("TODO: panic message");

    let folder = Path::new("./jars");

    if !folder.exists() {
        std::fs::create_dir(folder).expect("TODO: panic message");
    }

    http::download_file(&jar_type, &selected_version, selected_build).await;

}

async fn check_updates() {

}

fn first_run() -> bool {
    !Path::new("./jars").exists() && !Path::new("options.ini").exists()
}