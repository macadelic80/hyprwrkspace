pub mod config;

use config::reader::deserialization::get_config_from_text;

use notify::event::ModifyKind;
use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Duration;

fn on_event_kind_modify(config_path: &str) {
    match get_config_from_text(config_path.into()) {
        Ok(config) => {
            println!("[{}] Configuration mise à jour !\nContenu:\n{}", &config.applications.len(), config);
        },
        Err(str) => eprintln!("Error get_config: {}", str),
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
                        on_event_kind_modify(config_path);
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
