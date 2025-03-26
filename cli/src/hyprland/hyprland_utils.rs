use serde::de::DeserializeOwned;
use std::process::Command;
use thiserror::Error;
#[derive(Error, Debug)]
pub enum HyprctlError {
    #[error("Erreur d'exécution de la commande: {0}")]
    CommandError(#[from] std::io::Error),
    #[error("Erreur de désérialisation: {0}")]
    DeserializeError(#[from] serde_json::Error),
    #[error("Erreur d'encodage UTF-8: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),
}

/// Envoie une commande à hyprctl dispatch
///
/// # Arguments
/// * `command` - La commande à envoyer
/// * `args` - Les arguments de la commande
///
/// # Examples
/// ```no_run
/// use hyprland_utils::dispatch_hyprctl;
///
/// dispatch_hyprctl("workspace", &["1"]).expect("Échec du changement de workspace");
/// ```
pub fn dispatch_hyprctl(command: &str, args: Vec<String>) -> Result<String, HyprctlError> {
    let output = Command::new("hyprctl")
        .arg("dispatch")
        .arg(command)
        .args(args)
        .output()?;

    if output.status.success() {
        Ok(String::from_utf8(output.stdout)?)
    } else {
        let error_message = String::from_utf8_lossy(&output.stderr);
        Err(HyprctlError::CommandError(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("hyprctl failed: {}", error_message),
        )))
    }
}

/// Désérialise la sortie d'une commande hyprctl en JSON
///
/// # Arguments
/// * `command` - La commande hyprctl à exécuter
/// * `args` - Les arguments de la commande
///
/// # Examples
/// ```no_run
/// use hyprland_utils::deserialize_hyprctl_output;
/// use crate::types::Window;
///
/// let windows: Vec<Window> = deserialize_hyprctl_output("hyprctl", vec!["clients", "-j"])
///     .expect("Échec de la récupération des fenêtres");
/// ```
pub fn deserialize_hyprctl_output<T>(command: &str, args: Vec<&str>) -> Result<T, HyprctlError>
where
    T: DeserializeOwned,
{
    let output = Command::new(command).args(args).output()?;

    if !output.status.success() {
        let error_message = String::from_utf8_lossy(&output.stderr);
        return Err(HyprctlError::CommandError(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Command failed: {}", error_message),
        )));
    }

    let output_str = String::from_utf8(output.stdout)?;
    Ok(serde_json::from_str(&output_str)?)
}
