pub mod keypress;

use serde::{ Deserialize, Serialize };

use super::execute::Execution;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Initiator {
    pub type_: String,
    pub data: InitiatorData,
    pub executes: Vec<Execution>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InitiatorData {
    pub keys: Option<Vec<String>>,
    pub activate_time: Option<String>,
    pub time: Option<InitiatorKeypressTime>,
    pub cron: Option<String>,
    pub app_path: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InitiatorKeypressTime {
    pub min: f64,
    pub max: f64,
}