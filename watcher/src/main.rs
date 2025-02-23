pub mod config;

use config::reader::deserialization::get_config_from_text;

use config::reader::structs::{AppConfig, HyprwrkspaceConfig};
use notify::event::ModifyKind;
use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::str::FromStr;
use std::sync::mpsc::channel;
use std::time::Duration;


/*
bind = SUPER, D, exec, hyprctl dispatch togglespecialworkspace discord
windowrule = workspace special:discord, class:(discord)
*/
fn get_wr_config(name: &String, config: &AppConfig) -> Result<String, String> {
    let mut config_lines: String = Default::default();
    config_lines.push_str(
        format!(
            "windowrule = workspace special:{0}-special, class:({0})\n",
            name
        ).as_str()
    );
    Ok(config_lines)
}

fn get_kb_config(name: &String, config: &AppConfig) -> Result<String, String> {
    let mut config_lines = String::default();
    if !config.shortcut.is_empty() {
        config_lines.push_str(
            format!(
                "bind = SUPER, {}, exec, hyprws focus {}\n",
                config.shortcut,
                name
            ).as_str()
        );
    }

    if let Some(force_new) = &config.force_new {
        config_lines.push_str(
            format!(
                "bind = SUPER {}, {}, exec, hyprws focus {} --force\n",
                force_new,
                config.shortcut,
                name
            ).as_str()
        );
    }

    if let Some(args) = &config.args {
        for arg in args {
            config_lines.push_str(
                format!(
                    "bind = SUPER, {}, exec, hyprws focus {} -- {}\n",
                    arg.key,
                    name,
                    arg.name,
                ).as_str()
            );
        }
    }

    Ok(config_lines)
}


fn try_hyprland_config_update(config: HyprwrkspaceConfig, hyprland_config_path: &str) -> Result<(), String>{
    if config.applications.is_empty() {
        return Err("Empty".into());
    }
    let mut config_lines: String = "#######HPRWS CONFIG\n".into();
    // Générer les bind et windowrule pour chaque application
    for (name, app_config) in &config.applications {
        config_lines.push_str(
            format!(
                "######START HYPRWS({})\n",
                name,
            ).as_str()
        );
        let kb_config = get_kb_config(name, app_config)?;
        let wr_config = get_wr_config(name, app_config)?;
        config_lines.push_str(kb_config.as_str());
        config_lines.push('\n');
        config_lines.push_str(wr_config.as_str());
        config_lines.push_str(
            format!(
                "######END HYPRWS({})\n\n",
                name,
            ).as_str()
        );
        
    }

    // Écrire le fichier de configuration
    let hyprwrkspace_conf_path = Path::new(hyprland_config_path);
    match File::create(&hyprwrkspace_conf_path) {
        Ok(mut file) => {
            if let Err(err) = file.write_all(config_lines.as_bytes()) {
                eprintln!("Erreur lors de l'écriture du fichier: {}", err);
            } else {
                println!("Configuration mise à jour : {}", hyprwrkspace_conf_path.display());
            }
        }
        Err(err) => eprintln!("Impossible de créer le fichier : {}", err),
    }
    Ok(())
}

fn on_event_kind_modify(config_path: &str, hyprland_config_path: &str) {
    match get_config_from_text(config_path.into()) {
        Ok(config) => {
            println!("[{}] Configuration mise à jour !\nContenu:\n{}", &config.applications.len(), config);
            try_hyprland_config_update(config, hyprland_config_path);
        },
        Err(str) => eprintln!("Error get_config: {}", str),
    }

}


fn main() {
    let config_path = "/home/achraf/.config/hypr/hyprwrkspace.toml";
    let hyprland_config = "/home/achraf/.config/hypr/hyprws.conf";

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
                        on_event_kind_modify(config_path, hyprland_config);
                        // process_config_update(config_path.into(), hyprland_config.into());
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
