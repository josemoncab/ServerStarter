use std::fs::File;
use std::io::{BufRead, BufWriter, Write};
use std::path::Path;
use std::process::{Command, Stdio};

use crate::config::Settings;
use crate::utils::user_ask;

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

    logger::info("Comprobando la existencia de un jar valido...");

    if utils::check_jar().is_empty() {
        logger::warn("No se ha podido encontrar un jar para iniciar el servidor. Iniciando la \
        descarga...");
        create_server().await;
    } else {
        logger::success("Jar encontrado!");
        let settings = Settings::load();
        check_updates(settings).await;
    }

    start_server().await;
}

async fn create_server() {
    let scope = utils::user_ask("Introduce el tipo de jar que deseas usar",
                                  vec!["proxy", "server"]);
    let mut jar_type = String::new();
    let mut api_versions = vec![];

    if scope == "proxy" {
        jar_type = utils::user_ask(
            "Introduce el proxy que deseas usar",
            vec!["velocity"]);
    }
    if scope == "server" {
        jar_type = utils::user_ask(
            "Introduce el servidor que deseas usar",
            vec!["paper"]);
    }

    if jar_type == "paper" {
        api_versions = http::get_versions("paper").await;
    }

    if jar_type == "velocity" {
        api_versions = http::get_versions("velocity").await;
    }

    let selected_version = utils::user_ask("Introduce la version del servidor",
                                             api_versions.iter().map(|x| &**x).collect());

    let builds = http::get_builds(&jar_type, &selected_version).await;
    let selected_build = builds.last().expect("TODO: panic message");

    let folder = Path::new("./jars");

    if !folder.exists() {
        std::fs::create_dir(folder).expect("TODO: panic message");
    }

    logger::info("Iniciando la descarga del jar...");
    http::download_file(&jar_type, &selected_version, selected_build).await;
    logger::success("Jar descargado con exito!");

    let jar_file = format!("{}-{}-{}.jar", &jar_type, &selected_version, &selected_build);

    logger::info("Creando archivo de opciones. Este archivo puede ser editado por el usuario.");
    let settings = config::Settings::new(
        utils::user_input("Introduce un titulo para la consola"),
        selected_version,
        jar_type,
        utils::user_input("Introduce la cantidad minima de ram con la letra en mayúscula, \
        (Ejemplo: 2G)"),
        utils::user_input("Introduce la cantidad maxima de ram con la letra en mayúscula, \
        (Ejemplo: 4G)"),
        jar_file,
        true,
        10,
        utils::user_input("Introduce el ejecutable de java (o vacio para usar el java por \
        defecto del sistema)"),
        String::from("-XX:+UseG1GC -XX:+ParallelRefProcEnabled -XX:MaxGCPauseMillis=200 \
        -XX:+UnlockExperimentalVMOptions -XX:+DisableExplicitGC -XX:+AlwaysPreTouch \
        -XX:G1NewSizePercent=30 -XX:G1MaxNewSizePercent=40 -XX:G1HeapRegionSize=8M \
        -XX:G1ReservePercent=20 -XX:G1HeapWastePercent=5 -XX:G1MixedGCCountTarget=4 \
        -XX:InitiatingHeapOccupancyPercent=15 -XX:G1MixedGCLiveThresholdPercent=90 \
        -XX:G1RSetUpdatingPauseTimePercent=5 -XX:SurvivorRatio=32 -XX:+PerfDisableSharedMem -XX:MaxTenuringThreshold=1 -Dusing.aikars.flags=https://mcflags.emc.gs -Daikars.new.flags=true")
    );
    settings.write();
    logger::success("Configuraciones guardadas!");
}

async fn check_updates<'a>(settings: Settings<'a>) {
    let latest_version = http::get_versions(settings.software.0).await.last().expect("").clone();
    if latest_version != settings.mc_version {
        let should_update = utils::user_ask("Hay una nueva version de minecraft. ¿Deseas \
        actualizar?", vec!["y", "n"]);

        /*match should_update.as_str() {
            "y" => update(),
            _ => logger::error("")
        }*/
    }
}

async fn start_server() {
    check_eula().await;
    logger::info("Iniciando el servidor de minecraft...");
    let settings = Settings::load();

    let aikar_flags = settings.java_arguments.split_whitespace().collect::<Vec<&str>>();
    let mut command = Command::new(format!(r#"{}\bin\java.exe"#, settings.java))
        .arg(format!("-Xms{}", settings.min_ram))
        .arg(format!("-Xmx{}", settings.max_ram))
        .args(aikar_flags)
        .arg("-jar")
        .arg(format!(r#".\jars\{}"#, settings.jar))
        .arg("--nogui")
        .stdout(Stdio::inherit())
        .spawn().expect("TODO: panic message");

    command.wait().expect("TODO: panic message");
}

// TODO: Handle case when eula.txt is found but false
async fn check_eula() {
    let path = Path::new("eula.txt");
    if path.exists() {
        logger::success("Archivo eula encontrado!");
        return;
    }
    logger::warn("Archivo eula no encontrado! Generando uno nuevo...");
    if user_ask("¿Deseas aceptar el eula?", vec!["y", "n"]) == "y" {
        let file = File::create("eula.txt").expect("Error while creating eula.txt file");
        let mut writer = BufWriter::new(file);
        writer.write_all(b"eula=true").expect("Error while writing to eula file");
        writer.flush().expect("Error while saving the eula file");
        logger::success("Archivo eula aceptado!");
        return;
    }
    logger::error("No has aceptado el eula de minecraft. No podras abrir el servidor hasta que lo aceptes");
}

fn first_run() -> bool {
    !Path::new("./jars").exists() && !Path::new("options.ini").exists()
}