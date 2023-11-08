use std::fs;
use std::path::Path;
use crate::logger;

/// Return the name of the jar with extension found in the root directory or "jars" directory
pub fn check_jar() -> String {
    let mut jar_name = String::new();

    if Path::new("./jars").exists() {
        jar_name = search_jar("./jars")
    } else {
        jar_name = search_jar("./")
    }
    jar_name
}

fn search_jar(path: &str) -> String {
    let mut jar_name = String::new();
    for file in fs::read_dir(path).unwrap()  {
        let file = file.unwrap();
        match file.path().extension() {
            None => continue,
            file_extension => {
                if file_extension.unwrap() == "jar" {
                    jar_name = file.file_name().into_string().unwrap();
                    break;
                }
            }
        }

    }
    jar_name
}

/// Return the string given by the user. Automatically append the options at the end of the
/// message and check if the user input a valid option
pub fn user_ask(msg: &str, options: Vec<&str>) -> String {
    if !options.is_empty() {
        logger::user(format!("{msg} ({}):", options.join(", ")).as_str());
    } else {
        logger::user(format!("{msg}:").as_str());
    }

    let mut user_in= String::new();
    std::io::stdin().read_line(&mut user_in).unwrap();

    if !options.is_empty() {
        while !options.iter().any(|e| user_in.contains(e)) {
            logger::error("Opcion invalida");
            logger::user(format!("{msg} ({}):", options.join(", ")).as_str());
            user_in.clear();
            std::io::stdin().read_line(&mut user_in).unwrap();
        }
    }

    user_in.trim().to_string()
}

/// Return the string given by the user
pub fn user_input(msg: &str) -> String {
    user_ask(msg, vec![])
}