use std::{fs::File, io::Write, path::Path};

use crate::config::reader::structs::{AppConfig, HyprwrkspaceConfig};

fn get_kb_config(name: &String, config: &AppConfig) -> Result<String, String> {
    let mut config_lines = String::default();
    let executable = config.executable.replace('"', "\\\"");
    if !config.shortcut.is_empty() {
        config_lines.push_str(
            format!(
                "bind = SUPER, {}, exec, hyprws focus {} --command=\"{}\"\n",
                config.shortcut, name, executable
            )
            .as_str(),
        );
    }

    if let Some(force_new) = &config.force_new {
        config_lines.push_str(
            format!(
                "bind = SUPER {}, {}, exec, hyprws focus {} --command=\"{}\" --force\n",
                force_new, config.shortcut, name, executable
            )
            .as_str(),
        );
    }

    if let Some(args) = &config.args {
        for arg in args {
            config_lines.push_str(
                format!(
                    "bind = SUPER, {}, exec, hyprws focus {} --command=\"{}\" -- {}\n",
                    arg.key, name, executable, arg.name,
                )
                .as_str(),
            );
        }
    }

    Ok(config_lines)
}

pub fn try_hyprland_config_update(
    config: HyprwrkspaceConfig,
    hyprland_config_path: &str,
) -> Result<(), String> {
    if config.applications.is_empty() {
        return Err("Empty".into());
    }
    let mut config_lines: String = "#######HPRWS CONFIG\n".into();
    for (name, app_config) in &config.applications {
        config_lines.push_str(format!("######START HYPRWS({})\n", name,).as_str());
        let kb_config = get_kb_config(name, app_config)?;
        config_lines.push_str(kb_config.as_str());
        config_lines.push('\n');
        config_lines.push_str(format!("######END HYPRWS({})\n\n", name,).as_str());
    }

    let hyprwrkspace_conf_path = Path::new(hyprland_config_path);
    match File::create(&hyprwrkspace_conf_path) {
        Ok(mut file) => {
            if let Err(err) = file.write_all(config_lines.as_bytes()) {
                eprintln!("Error on file writing: {}", err);
            } else {
                println!(
                    "Config file hyprws.conf updated : {}",
                    hyprwrkspace_conf_path.display()
                );
            }
        }
        Err(err) => eprintln!("Can't create hyprws.conf : {}", err),
    }
    Ok(())
}
