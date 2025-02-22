use notify::event::ModifyKind;
use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Duration;
use serde::Deserialize;
use std::thread;

#[derive(Deserialize)]
struct HyprwrkspaceConfig {
    applications: HashMap<String, AppConfig>,
}


#[derive(Deserialize)]
struct AppConfigArg {
    key: String,
    name: String
}

#[derive(Deserialize)]
struct AppConfig {
    executable: String,
    shortcut: String,
    force_new: Option<String>,
    several_instances: Option<bool>,
    args: Option<Vec<AppConfigArg>>,
}

fn get_config_from_text(text: String) -> Result<HyprwrkspaceConfig, String> {
    let res_config: Result<HyprwrkspaceConfig, toml::de::Error> = toml::from_str(&text);

    match res_config {
        Ok(config) => {
            Ok(config)
        }
        Err(e) => {
            Err(format!("Erreur lors du parsing du fichier TOML {}", e))
        }
    }
}

fn read_file(path: String) -> Result<String, String> {
    match fs::read_to_string(path) {
        Ok(file) => Ok(file),
        Err(err) => Err(format!("Err read_file {}", err))
    }
}

fn wait_for_file_ready(path: &str, max_attempts: u32) -> bool {
    for index in 0..max_attempts {
        if let Ok(metadata) = fs::metadata(path) {
            if metadata.len() > 0 {
                return true;
            } else {
                println!("Pas bon pour la: {}: Metadata {}", index, metadata.len());
            }
        } else {
            println!("Pas bon pour la: {}: Metadata pas bon", index);
        }
        thread::sleep(Duration::from_millis(50));
    }
    false
}

fn process_config_update(config_path: String, _hyprland_config: String) {
    if !wait_for_file_ready(&config_path, 10) {
        println!("Le fichier n'est toujours pas prêt après plusieurs tentatives.");
        return;
    }
    match read_file(config_path) {
        Ok(config_text) => {
            println!("Lecture de {} chars", config_text.len());
            let config_r = get_config_from_text(config_text);
            if let Ok(config) = config_r {
                for (name, app_config) in &config.applications {
                    println!("Nouvelle config : {}: executable: {}, shortcut: {}, force_new: {}, several_instances: {}", 
                             name, app_config.executable, app_config.shortcut, app_config.force_new.as_deref().unwrap_or("no so false".into()), app_config.several_instances.unwrap_or(false));
                    
                    if let Some(args) = &app_config.args {
                        for arg in args {
                            println!("Arg pour {}: key: {} name {}", name, arg.key, arg.name)
                        }
                    } else {
                        println!("No args for {}", name);
                    }
                }
                    println!("[{}] Configuration mise à jour !", &config.applications.len());
            } else if let Err(e) = config_r {
                println!("Erreur config: {}", e)
            }
        },
        Err(err) => {
            println!("Erreur: {}", err);
        }
    }
}

fn main() {
    let config_path = "/home/achraf/.config/hypr/hyprwrkspace.toml";
    let hyprland_config = "/home/achraf/.config/hypr/hyprland.conf";

    let (tx, rx) = channel();
    let config: Config = Default::default();
    let mut watcher: RecommendedWatcher = Watcher::new(
        tx,
        config.with_poll_interval(Duration::from_secs(1))
    ).expect("Erreur de watcher");
    watcher.watch(
        Path::new(config_path),
        RecursiveMode::NonRecursive
    ).expect("Impossible de surveiller le fichier");

    println!("Surveillance de {}...", config_path);
    let mut last_event_time = std::time::Instant::now();
    let debounce_duration = std::time::Duration::from_millis(500);
    
    for event in rx {
        match event {
            Ok(ev) => match ev.kind {
                EventKind::Modify(ModifyKind::Data(_)) => {
                    if last_event_time.elapsed() > debounce_duration {
                        println!("Modification détectée, mise à jour...");
                        process_config_update(config_path.into(), hyprland_config.into());
                        last_event_time = std::time::Instant::now();
                    }
                },
                _ => {

                }
            },
            Err(_) => {

            }
        }
    }
}
