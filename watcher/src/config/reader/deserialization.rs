use super::{file_reader::{read_file, wait_for_file_ready}, structs::HyprwrkspaceConfig};


pub fn deserialize_config(text: String) -> Result<HyprwrkspaceConfig, String> {
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

pub fn get_config_from_text(config_path: String) -> Result<HyprwrkspaceConfig, String>{
    if !wait_for_file_ready(&config_path, 10) {
        Err("Le fichier n'est toujours pas prêt après plusieurs tentatives".into())
    } else {
        match read_file(config_path) {
            Ok(config_text) => {
                deserialize_config(config_text)
            },
            Err(err) => {
                Err(format!("Erreur: {}", err))
            }
        }
    }
}
