use serde::Deserialize;


#[derive(Deserialize, Debug)]
#[serde(rename_all="camelCase")]
#[allow(dead_code)]
pub struct MinimalWorkspace {
    pub id: i32,
    pub name: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all="camelCase")]
#[allow(dead_code)]
pub struct Workspace {
    pub id: i32,
    pub name: String,
    pub monitor: String,
    #[serde(rename = "monitorID")]
    pub monitor_id: u32,
    pub windows: u32,
    pub hasfullscreen: bool,
    pub lastwindow: String,
    pub lastwindowtitle: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all="camelCase")]
#[allow(dead_code)]
pub struct Window {
    pub address: String,
    pub mapped: bool,
    pub hidden: bool,
    pub at: [u32; 2],
    pub size: [u32; 2],
    pub workspace: MinimalWorkspace,
    pub floating: bool,
    pub pseudo: bool,
    pub monitor: u32,
    pub class: String,
    pub title:String,
    pub initial_class: String,
    pub initial_title: String,
    pub pid: u32,
    pub xwayland: bool,
    pub pinned: bool,
    pub fullscreen: u32,
    pub fullscreen_client: u32,
    pub grouped: Vec<u32>,
    pub tags: Vec<String>,
    pub swallowing: String,
    #[serde(rename = "focusHistoryID")]
    pub focus_history_id: u32,
    pub inhibiting_idle: bool
}
