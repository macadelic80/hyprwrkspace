use std::{fs::File, io::Write, path::Path};

use crate::config::reader::structs::{AppConfig, HyprwrkspaceConfig};

fn get_wr_config(name: &String, config: &AppConfig) -> Result<String, String> {
    let mut config_lines: String = Default::default();
    config_lines.push_str(
        format!(
            "windowrule = workspace special:{0}-special, class:({0})\n",
            name
        )
        .as_str(),
    );
    Ok(config_lines)
}

fn get_kb_config(name: &String, config: &AppConfig) -> Result<String, String> {
    let mut config_lines = String::default();
    if !config.shortcut.is_empty() {
        config_lines.push_str(
            format!(
                "bind = SUPER, {}, exec, hyprws focus {}\n",
                config.shortcut, name
            )
            .as_str(),
        );
    }

    if let Some(force_new) = &config.force_new {
        config_lines.push_str(
            format!(
                "bind = SUPER {}, {}, exec, hyprws focus {} --force\n",
                force_new, config.shortcut, name
            )
            .as_str(),
        );
    }

    if let Some(args) = &config.args {
        for arg in args {
            config_lines.push_str(
                format!(
                    "bind = SUPER, {}, exec, hyprws focus {} -- {}\n",
                    arg.key, name, arg.name,
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
    // Générer les bind et windowrule pour chaque application
    for (name, app_config) in &config.applications {
        config_lines.push_str(format!("######START HYPRWS({})\n", name,).as_str());
        let kb_config = get_kb_config(name, app_config)?;
        let wr_config = get_wr_config(name, app_config)?;
        config_lines.push_str(kb_config.as_str());
        config_lines.push('\n');
        config_lines.push_str(wr_config.as_str());
        config_lines.push_str(format!("######END HYPRWS({})\n\n", name,).as_str());
    }

    // Écrire le fichier de configuration
    let hyprwrkspace_conf_path = Path::new(hyprland_config_path);
    match File::create(&hyprwrkspace_conf_path) {
        Ok(mut file) => {
            if let Err(err) = file.write_all(config_lines.as_bytes()) {
                eprintln!("Erreur lors de l'écriture du fichier: {}", err);
            } else {
                println!(
                    "Configuration mise à jour : {}",
                    hyprwrkspace_conf_path.display()
                );
            }
        }
        Err(err) => eprintln!("Impossible de créer le fichier : {}", err),
    }
    Ok(())
}
