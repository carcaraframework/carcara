use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// An enum to represent the two types of execution blocks
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ExecutionBlock {
    Step(Step),
    Async(AsyncBlock),
}

// The `step` block, containing a single action
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Step {
    pub id: Option<String>,
    pub description: Option<String>,
    #[serde(flatten)]
    pub action: Action,
    pub failure: Option<Vec<Step>>,
}

// The `async` block, containing a list of `steps`
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct AsyncBlock {
    #[serde(rename = "async")]
    pub description: Option<String>,
    pub steps: Vec<Step>,
    pub failure: Option<Vec<Step>>,
}

// The `Action` enum, representing all possible actions
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Action {
    Flyto(UrlParams),
    Strike(SelectorParams),
    Type(TypeParams),
    Press(PressParams),
    Observe(WaitParams),
    Upload(UploadParams),
    Download(DownloadParams),
    Capture(ScreenshotParams),
    Execute(ScriptParams),
    Examine(ExamineParams),
    Cookie(CookieParams),
    Logs(LogsParams),
}

// Structs for the parameters of each action below
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct UrlParams {
    pub url: String,
    pub timeout: Option<u64>,
}

// ... (todas as outras structs de parâmetros como SelectorParams, TypeParams, etc. continuam aqui)
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SelectorParams { pub selector: String, pub timeout: Option<u64>, pub human_delay: Option<bool>, }
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct TypeParams { pub selector: String, pub text: String, pub human_delay: Option<bool>, }
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PressParams { pub key: String, pub modifiers: Option<Vec<String>>, }
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct WaitParams { pub selector: Option<String>, pub duration: Option<u64>, pub timeout: Option<u64>, }
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct UploadParams { pub selector: String, pub path: String, pub timeout: Option<u64>, }
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct DownloadParams { pub path: String, pub timeout: Option<u64>, }
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ScreenshotParams { pub path: String, pub full_page: Option<bool>, }
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ScriptParams { pub script: String, pub store_as: Option<String>, }
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ExamineParams { pub selector: String, pub property: String, pub store_as: String, }
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CookieParams { pub operation: String, pub name: String, pub value: Option<String>, pub store_as: Option<String>, }
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct LogsParams { pub path: String, }


// Represents the entire Carcarafile structure, now expecting an array of ExecutionBlocks
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Carcarafile(pub Vec<ExecutionBlock>);
