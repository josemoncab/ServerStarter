use std::fs;
use std::path::Path;
use crate::logger;

/// Return the name of the jar with extension found in the root directory or "jars" directory
pub fn check_jar() -> String {
    let mut jar_name = String::new();

    if Path::new("./jars").exists() {
        for file in fs::read_dir("./jars").unwrap()  {
            let file = file.unwrap();
            if file.path().extension().unwrap() == "jar" {
                jar_name = file.file_name().into_string().unwrap();
                break;
            }
        }
    } else {
        for file in fs::read_dir("./").unwrap()  {
            let file = file.unwrap();
            if file.path().extension().is_none() { continue }
            if file.path().extension().unwrap() == "jar" {
                jar_name = file.file_name().into_string().unwrap();
                break;
            }
        }
    }
    jar_name
}

/// Return the string given by the user. Automatically append the options at the end of the message
pub fn user_input(msg: &str, options: Vec<&str>) -> String {
    logger::user(format!("{msg} ({}):", options.join(", ")).as_str());

    let mut user_in= String::new();
    std::io::stdin().read_line(&mut user_in).unwrap();

    if !options.is_empty() {
        while !options.iter().any(|e| user_in.contains(e)) {
            logger::error("Opcion invalida");
            logger::user(msg);
            user_in.clear();
            std::io::stdin().read_line(&mut user_in).unwrap();
        }
    }

    user_in.trim().to_string()
}