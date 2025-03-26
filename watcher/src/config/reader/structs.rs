use serde::Deserialize;
use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
};

#[derive(Deserialize)]
pub struct HyprwrkspaceConfig {
    pub applications: HashMap<String, AppConfig>,
}

#[derive(Deserialize)]
pub struct AppConfigArg {
    pub key: String,
    pub name: String,
}

#[derive(Deserialize)]
pub struct AppConfig {
    pub executable: String,
    pub shortcut: String,
    pub force_new: Option<String>,
    pub several_instances: Option<bool>,
    pub args: Option<Vec<AppConfigArg>>,
}

impl Display for HyprwrkspaceConfig {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        for (name, app) in &self.applications {
            writeln!(f, "Application: {}", name)?;
            writeln!(f, "  - Executable: {}", app.executable)?;
            writeln!(f, "  - Shortcut: {}", app.shortcut)?;
            writeln!(
                f,
                "  - Force New: {}",
                app.force_new.as_deref().unwrap_or("None")
            )?;
            writeln!(
                f,
                "  - Several Instances: {}",
                app.several_instances.unwrap_or(false)
            )?;

            if let Some(args) = &app.args {
                writeln!(f, "  - Arguments:")?;
                for arg in args {
                    writeln!(f, "    - {}: {}", arg.key, arg.name)?;
                }
            } else {
                writeln!(f, "  - Arguments: None")?;
            }
        }
        Ok(())
    }
}
