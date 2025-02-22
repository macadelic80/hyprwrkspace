use std::process::Command;
use serde::de::DeserializeOwned;

pub fn dispatch_hyprctl(command: &str, args: &[&str]) {
    let output = Command::new("hyprctl")
        .arg("dispatch")
        .arg(command)
        .args(args)
        .output();

    match output {
        Ok(o) => println!("Sortie: {}", String::from_utf8_lossy(&o.stdout)),
        Err(e) => eprintln!("Erreur: {}", e),
    }
}


pub fn deserialize_hyprctl_output<T>(command: &str, args: Vec<&str>) -> Result<T, String>
where
    T: DeserializeOwned {
    match Command::new(command)
    .args(args)
    .output() {
        Ok(config) => {
            let output_str = String::from_utf8_lossy(&config.stdout);
            match serde_json::from_str::<T>(&output_str) {
                Ok(value) => {
                    Ok(value)
                },
                Err(err) => Err(format!("Error parsing config: {}", err))
            }
        },
        Err(err) => {
            Err(format!("Erreur de lecture de {}: {}", command, err))
        }
    }
}
