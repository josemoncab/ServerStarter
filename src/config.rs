use ini::Ini;

#[derive(Debug)]
pub struct Settings<'a> {
    console_title: String,
    mc_version: String,
    software: Software<'a>,
    min_ram: String,
    max_ram: String,
    jar: String,
    auto_restart: bool,
    restart_timeout: i32,
    java: String,
    java_arguments: String
}

impl Settings<'_> {
    pub fn new(console_title: String, mc_version: String, software: String,
               min_ram: String, max_ram: String, jar: String, auto_restart: bool,
               restart_timeout: i32, java: String, java_arguments: String) -> Self {
        Self {
            console_title,
            mc_version,
            software: Software::parse(software.as_str()),
            min_ram,
            max_ram,
            jar,
            auto_restart,
            restart_timeout,
            java,
            java_arguments }
    }

    pub fn load() -> Self {
        let configs = Ini::load_from_file("options.ini").expect("Failed to load settings from \
        file!");

        let section = configs.general_section();


        let should_restart = match section.get("auto_restart").expect("Failed to get auto_restart field") {
            "true" => true,
            _ => false
        };

        Settings {
            console_title: section.get("console_title").expect("Failed to get console_title field").to_string(),
            mc_version: section.get("mc_version").expect("Failed to get mc_version field").to_string(),
            software: Software::parse(section.get("software").expect("Failed to get software \
            field")),
            min_ram: section.get("min_ram").expect("Failed to get min_ram field").to_string(),
            max_ram: section.get("mc_version").expect("Failed to get max_ram field").to_string(),
            jar: section.get("jar").expect("Failed to get jar field").to_string(),
            auto_restart: should_restart,
            restart_timeout: section.get("restart_timeout").expect("Failed to get restart_timeout field")
                .parse().expect("Failed to convert str to int"),
            java: section.get("java").expect("Failed to get java field").to_string(),
            java_arguments: section.get("java_arguments").expect("Failed to get java_arguments field").to_string(),
        }
    }
    pub fn write(&self) {
        let mut configs = Ini::new();
        configs.with_general_section()
            .set("console_title", self.console_title.clone())
            .set("mc_version", self.mc_version.clone())
            .set("software", self.software.0)
            .set("min_ram", self.min_ram.clone())
            .set("max_ram", self.max_ram.clone())
            .set("jar", self.jar.clone())
            .set("auto_restart", self.auto_restart.to_string())
            .set("restart_timeout", self.restart_timeout.to_string())
            .set("java", self.java.clone())
            .set("java_arguments", self.java_arguments.clone());
        configs.write_to_file("options.ini").expect("Failed to write options to file");
    }
}

#[derive(Debug)]
struct Software<'a>(&'a str);

impl<'a> Software<'a> {
    pub const PAPER: Software<'a> = Software("paper");
    pub const VELOCITY: Software<'a> = Software("velocity");

    pub fn parse(input: &str) -> Self {
        match input {
            "paper" => Software::PAPER,
            "velocity" => Software::VELOCITY,
            _ => Software("None")
        }
    }
}