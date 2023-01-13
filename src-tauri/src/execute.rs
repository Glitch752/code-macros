pub mod string_parser;

use serde::{ Deserialize, Serialize };

static MAX_LOOP_ITERATIONS: u64 = 100000;

use super::Macro;
use super::get_app_handle;
use super::initiators::Initiator;

use tauri::AppHandle;
use tauri::{api::notification::Notification, PathResolver, Manager, Window};

use inputbot::{KeySequence, MouseCursor, KeybdKey, MouseButton, get_keybd_key};

use std::fs;
use std::thread;
use std::cmp::Ordering;
use std::{collections::HashMap, path::Path};

use self::string_parser::parse_string;

// TODO: REACTOR: Split this into multiple files for parsing conditions, executions, etc.

// #[derive(Serialize, Deserialize, Clone, Debug)]
// pub struct Execution {
//     pub type_: String,
//     pub data: ExecutionData,
//     pub variables: Vec<VariableType>,
//     pub code_inside: ExecutionCodeInside
// }

// pub struct ExecutionData {
//     pub time: Option<f64>,
//     pub title: Option<String>,
//     pub message: Option<String>,
//     pub start: Option<f64>,
//     pub end: Option<f64>,
//     pub step: Option<f64>,
//     pub condition: Option<Condition>,
//     pub variable: Option<String>,
//     pub data: Option<String>,
//     pub value: Option<f64>,
//     pub file: Option<String>,
//     pub content: Option<Expression>,
//     pub function: Option<String>,
//     pub string: Option<String>,
//     pub key: Option<String>,
//     pub button: Option<String>,
//     pub x: Option<f64>,
//     pub y: Option<f64>
// }

// MAYBE: this is probably be better as an Adjacently tagged enum so we don't need another struct for every data type

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type_")]
#[serde(rename_all = "lowercase")]
pub enum Execution {
    If {
        data: IfData,
        code_inside: ExecutionCodeInside
    },
    Function {
        data: FunctionData,
    },
    FromToLoop {
        data: FromToLoopData,
        variables: Vec<VariableType>,
        code_inside: ExecutionCodeInside
    },
    WhileLoop {
        data: WhileLoopData,
        variables: Vec<VariableType>,
        code_inside: ExecutionCodeInside
    },
    Notification {
        data: NotificationData
    },
    Wait {
        data: WaitData
    },
    SetVariable {
        data: SetVariableData
    },
    TypeString {
        data: TypeStringData
    },
    Stop { },
    #[serde(rename = "movemouserelative")]
    MouseMoveRelative {
        data: MouseMoveRelativeData
    },
    #[serde(rename = "movemouseabsolute")]
    MouseMoveAbsolute {
        data: MouseMoveAbsoluteData
    },
    PressKey {
        data: PressKeyData
    },
    ReleaseKey {
        data: ReleaseKeyData
    },
    PressMouse {
        data: PressMouseData
    },
    ReleaseMouse {
        data: ReleaseMouseData
    },
    ReadFile {
        data: ReadFileData
    },
    WriteFile {
        data: WriteFileData
    },
    DeleteFile {
        data: DeleteFileData
    },
    CreateFolder {
        data: CreateFolderData
    },
    DeleteFolder {
        data: DeleteFolderData
    },
    GetDataType {
        data: GetDataTypeData
    },
    CreateArray {
        data: CreateArrayData
    },
    AddToArray {
        data: AddToArrayData
    },
    RemoveFromArray {
        data: RemoveFromArrayData
    },
    GetArrayLength {
        data: GetArrayLengthData
    },
    LoopArray {
        data: LoopArrayData,
        variables: Vec<VariableType>,
        code_inside: ExecutionCodeInside
    },
    GetArrayIndex {
        data: GetArrayIndexData
    },
    SetArrayIndex {
        data: SetArrayIndexData
    },
    GetFolderContents {
        data: GetFolderContentsData
    },
    Log {
        data: LogData
    },
    ClearLog { },
    SplitString {
        data: SplitStringData
    },
    JoinStrings {
        data: JoinStringsData
    },
    ReverseArray {
        data: ReverseArrayData
    },
    SortArray {
        data: SortArrayData
    },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IfData {
    pub condition: Condition
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FromToLoopData {
    pub start: f64,
    pub end: f64,
    pub step: f64
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FunctionData {
    pub function: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WhileLoopData {
    pub condition: Condition
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NotificationData {
    pub title: String,
    pub message: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WaitData {
    pub time: f64
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SetVariableData {
    pub variable: String,
    pub content: Expression
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TypeStringData {
    pub string: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MouseMoveRelativeData {
    pub x: f64,
    pub y: f64
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MouseMoveAbsoluteData {
    pub x: f64,
    pub y: f64
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PressKeyData {
    pub key: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ReleaseKeyData {
    pub key: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PressMouseData {
    pub button: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ReleaseMouseData {
    pub button: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ReadFileData {
    pub file: String,
    pub variable: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WriteFileData {
    pub file: String,
    pub content: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DeleteFileData {
    pub file: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreateFolderData {
    pub path: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DeleteFolderData {
    pub path: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetDataTypeData {
    pub variable: String,
    pub output: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreateArrayData {
    pub variable: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AddToArrayData {
    pub array: String,
    pub data: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RemoveFromArrayData {
    pub array: String,
    pub index: f64
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetArrayLengthData {
    pub array: String,
    pub output: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LoopArrayData {
    pub array: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetArrayIndexData {
    pub array: String,
    pub index: f64,
    pub output: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SetArrayIndexData {
    pub array: String,
    pub index: f64,
    pub data: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetFolderContentsData {
    pub path: String,
    pub output: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LogData {
    pub message: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SplitStringData {
    pub string: String,
    pub splitter: String,
    pub output: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JoinStringsData {
    pub array: String,
    pub joiner: String,
    pub output: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ReverseArrayData {
    pub array: String,
    pub output: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SortArrayData {
    pub array: String,
    pub output: String
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VariableType {
    pub type_: String,
    pub name: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type_")]
#[serde(rename_all = "lowercase")]
pub enum Condition {
    Boolean { value: bool },
    Comparison {
        left: Box<Condition>, 
        comparison: String, 
        right: Box<Condition> 
    },
    Logical {
        left: Box<Condition>, 
        kind: String, 
        right: Box<Condition> 
    },
    Number { value: f64 },
    Variable { variable: String },
    Expression { expression: Expression }
}


#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type_")]
#[serde(rename_all = "lowercase")]
pub enum Expression {
    Number { value: f64 },
    Variable { variable: String },
    Arithmetic {
        left: Box<Expression>, 
        kind: String, 
        right: Box<Expression> 
    },
    Bitwise {
        left: Box<Expression>, 
        kind: String, 
        right: Box<Expression> 
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ExecutionCodeInside {
    pub loop_: Option<ExecutionWrapper>,
    pub then: Option<ExecutionWrapper>,
    pub else_: Option<ExecutionWrapper>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ExecutionWrapper {
    pub executes: Vec<Execution>
}

impl<'a> Default for &'a ExecutionWrapper {
    fn default() -> &'a ExecutionWrapper {
        static DEFAULT: ExecutionWrapper = ExecutionWrapper {
            executes: Vec::new()
        };
        &DEFAULT
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub executes: Vec<Execution>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Parameter {
    pub name: String,
    pub type_: String,
    pub default_value: String,
}

pub fn run_macro_initiator(initiator: Initiator, macro_: Macro) {
    println!("Running macro initiator from macro \"{}\"", macro_.name);
    thread::spawn(move || {
        let mut new_variables: Variables = Variables::new();
        execute_macro_code(&initiator.executes, &mut new_variables, &mut false, macro_);
    });
}

pub fn run_macro_function(function: Function, macro_: Macro, variables: &mut Variables) {
    println!("Running macro function from macro \"{}\"", macro_.name);
    execute_macro_code(&function.executes, variables, &mut false, macro_);
}


type Variables = HashMap<String, Variable>;

#[derive(Debug, Clone)]
pub struct Variable {
    pub value: VariableValue,
}

impl Variable {
    fn new(value: VariableValue) -> Variable<> {
        Variable { value: value }
    }
}

#[derive(Clone, Debug)]
pub enum VariableValue {
    String(String),
    Number(f64),
    Array(Vec<VariableValue>)
}

impl Ord for VariableValue {
    fn cmp(&self, other: &VariableValue) -> Ordering {
        match (self, other) {
            (VariableValue::String(a), VariableValue::String(b)) => a.cmp(b),
            (VariableValue::Number(a), VariableValue::Number(b)) => a.partial_cmp(b).unwrap(),
            (VariableValue::Array(a), VariableValue::Array(b)) => a.cmp(b),
            _ => panic!("Cannot compare different types")
        }
    }
}

impl PartialOrd for VariableValue {
    fn partial_cmp(&self, other: &VariableValue) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Eq for VariableValue {}

impl PartialEq for VariableValue {
    fn eq(&self, other: &VariableValue) -> bool {
        match (self, other) {
            (VariableValue::String(a), VariableValue::String(b)) => a == b,
            (VariableValue::Number(a), VariableValue::Number(b)) => a == b,
            (VariableValue::Array(a), VariableValue::Array(b)) => a == b,
            _ => false
        }
    }
}

fn get_variable_string(variable_value: VariableValue) -> String {
    match variable_value {
        VariableValue::String(value) => {
            return value;
        },
        VariableValue::Number(value) => {
            return value.to_string();
        },
        VariableValue::Array(value) => {
            let mut converted_value = "[".to_string();
            for i in 0..value.len() {
                converted_value.push_str(&(get_variable_string(value[i].clone())));
                if i != (value.len()-1) {
                    converted_value.push_str(", ");
                }
            }
            converted_value.push_str("]");
            return converted_value;
        }
    }
}

fn get_variable_number(variable_value: VariableValue) -> f64 {
    match variable_value {
        VariableValue::String(_value) => {
            // TODO: Make this return the number in the string if possible.
            return 0.0;
        },
        VariableValue::Number(value) => {
            return value;
        },
        VariableValue::Array(_value) => {
            return 0.0;
        }
    }
}

fn get_variable_vector(variable_value: VariableValue) -> Vec<VariableValue> {
    match variable_value {
        VariableValue::String(value) => {
            return value.chars().map(|c| VariableValue::String(c.to_string())).collect();
        },
        VariableValue::Number(_value) => {
            return Vec::new();
        },
        VariableValue::Array(value) => {
            return value;
        }
    }
}

fn execute_macro_code(code: &Vec<Execution>, variables: &mut Variables, stop_execution: &mut bool, macro_: Macro) {
    for execution in code {
        if *stop_execution {
            return;
        }

        match execution {
            Execution::Wait { data } => {
                thread::sleep(std::time::Duration::from_millis((data.time * 1000.0) as u64));
            }
            Execution::Notification { data } => {
                let _ = Notification::new("code-macros")
                    .title(parse_string(&data.title, variables))
                    .body(parse_string(&data.message, variables))
                    .show();
            }
            Execution::FromToLoop { data, variables: variables_set, code_inside } => {
                let mut i: f64 = data.start;
                let mut iterations: u64 = 0;
                let mut value_variable: Option<String> = None;
                for variable in variables_set {
                    if variable.type_ == "value".to_string() {
                        value_variable = Some(variable.name.clone());
                    }
                }
                let variable_name: String = value_variable.unwrap_or("".to_string());

                if data.end > data.start {
                    while i <= data.end {
                        set_variable(variables, variable_name.to_string().clone(), VariableValue::Number(i));
                        execute_macro_code(&code_inside.loop_.as_ref().unwrap_or_default().executes, variables, stop_execution, macro_.clone());
                        i += data.step;
                        iterations += 1;
                        if iterations > MAX_LOOP_ITERATIONS {
                            break;
                        }
                    }
                } else {
                    while i >= data.end {
                        set_variable(variables, variable_name.to_string().clone(), VariableValue::Number(i));
                        execute_macro_code(&code_inside.loop_.as_ref().unwrap_or_default().executes, variables, stop_execution, macro_.clone());
                        i += data.step;
                        iterations += 1;
                        if iterations > MAX_LOOP_ITERATIONS {
                            break;
                        }
                    }
                }
            }
            Execution::WhileLoop { data, variables: variables_set, code_inside } => {
                let mut value_variable: Option<String> = None;
                for variable in variables_set {
                    if variable.type_ == "iteration".to_string() {
                        value_variable = Some(variable.name.clone());
                    }
                }
                let variable_name: String = value_variable.unwrap_or("".to_string());

                let mut i: u64 = 0;

                while get_condition_bool(evaluate_condition(&data.condition, variables)) {
                    set_variable(variables, variable_name.to_string().clone(), VariableValue::Number(i as f64));
                    i += 1;
                    execute_macro_code(&code_inside.loop_.as_ref().unwrap_or_default().executes, variables, stop_execution, macro_.clone());
                    
                    if i > MAX_LOOP_ITERATIONS {
                        break;
                    }
                }
            }
            Execution::If { data, code_inside } => {
                if get_condition_bool(evaluate_condition(&data.condition, variables)) {
                    execute_macro_code(&code_inside.then.as_ref().unwrap_or_default().executes, variables, stop_execution, macro_.clone());
                } else {
                    execute_macro_code(&code_inside.else_.as_ref().unwrap_or_default().executes, variables, stop_execution, macro_.clone());
                }
            }
            Execution::Stop {  } => {
                *stop_execution = true;
            }
            Execution::SetVariable { data } => {
                set_variable(variables, data.variable.to_string().clone(), VariableValue::Number(
                    get_expression_number(evaluate_expression(&data.content, &mut variables.clone()))
                ));
            }
            Execution::Function { data } => {
                for function in &macro_.clone().macro_.functions.unwrap() {
                    if function.name == *data.function {
                        run_macro_function(function.clone(), macro_.clone(), variables);
                        break;
                    }
                }
            }
            Execution::TypeString { data } => {
                // FIXME: Yes, this is really, really bad code. It is intentionally creating a memory leak.
                let string_static = Box::leak(parse_string(&data.string.clone(), &mut variables.clone()).into_boxed_str());

                KeySequence(string_static).send();
            }
            Execution::MouseMoveRelative { data } => {
                MouseCursor::move_rel(data.x.round() as i32, data.y.round() as i32);
            }
            Execution::MouseMoveAbsolute { data } => {
                MouseCursor::move_abs(data.x.round() as i32, data.y.round() as i32);
            }
            Execution::PressKey { data } => {
                let key_char: char = data.key.chars().nth(0).unwrap();

                // Check if the key pressed requires shift to be held
                if key_char.is_uppercase() || [ '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '_', '+', '{', '}', '|', ':', '"', '<', '>', '?', '~' ].contains(&key_char) {
                    KeybdKey::LShiftKey.press();
                }

                let keybd_key: Option<KeybdKey> = get_keybd_key(key_char);
                
                match keybd_key {
                    Some(press) => {
                        press.press();
                    }
                    None => { unimplemented!() }
                }
            }
            Execution::ReleaseKey { data } => {
                let key_char: char = data.key.chars().nth(0).unwrap();

                // Check if the key pressed requires shift to be held
                if key_char.is_uppercase() || [ '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '_', '+', '{', '}', '|', ':', '"', '<', '>', '?', '~' ].contains(&key_char) {
                    KeybdKey::LShiftKey.release();
                }

                let keybd_key: Option<KeybdKey> = get_keybd_key(key_char);
                
                match keybd_key {
                    Some(press) => {
                        press.release();
                    }
                    None => { unimplemented!() }
                }
            }
            Execution::PressMouse { data } => {
                match data.button.as_str() {
                    "LMB" => {
                        MouseButton::LeftButton.press();
                    }
                    "RMB" => {
                        MouseButton::RightButton.press();
                    }
                    _ => todo!()
                }
            }
            Execution::ReleaseMouse { data } => {
                match data.button.as_str() {
                    "LMB" => {
                        MouseButton::LeftButton.release();
                    }
                    "RMB" => {
                        MouseButton::RightButton.release();
                    }
                    _ => todo!()
                }
            }
            Execution::ReadFile { data } => {
                let file_contents: String = fs::read_to_string(&data.file).unwrap_or("".to_string());

                set_variable(variables, data.variable.to_string().clone(), VariableValue::String(file_contents));
            }
            Execution::WriteFile { data } => {
                let file_contents: String = parse_string(&data.content, &mut variables.clone());

                fs::write(&data.file, file_contents).unwrap();
            }
            Execution::DeleteFile { data } => {
                fs::remove_file(&data.file).unwrap();
            }
            Execution::CreateFolder { data } => {
                fs::create_dir(&data.path).unwrap();
            }
            Execution::DeleteFolder { data } => {
                fs::remove_dir(&data.path).unwrap();
            }
            Execution::GetDataType { data } => {
                let variable_value: Option<&Variable> = get_variable(variables, data.variable.to_string().clone());

                let variable_type = match variable_value {
                    Some(variable) => match variable.value {
                        VariableValue::Number(_) => "number".to_string(),
                        VariableValue::String(_) => "string".to_string(),
                        VariableValue::Array(_) => "array".to_string()
                    }
                    None => "undefined".to_string()
                };

                set_variable(variables, data.output.to_string().clone(), VariableValue::String(variable_type));
            }
            Execution::CreateArray { data } => {
                set_variable(variables, data.variable.to_string().clone(), VariableValue::Array(Vec::new()));
            }
            Execution::AddToArray { data } => {
                let variable_value: Option<&Variable> = get_variable(variables, data.array.to_string().clone());

                let list_content: Vec<VariableValue> = get_variable_vector(variable_value.unwrap_or(&Variable::new(VariableValue::Array(vec![]))).value.clone());

                let new_value: Option<&Variable> = get_variable(variables, data.data.to_string().clone());

                let mut new_list_content: Vec<VariableValue> = list_content.clone();
                new_list_content.push(new_value.unwrap_or(&Variable::new(VariableValue::Number(0.0))).value.clone());

                set_variable(variables, data.array.to_string().clone(), VariableValue::Array(new_list_content));
            }
            Execution::RemoveFromArray { data } => {
                let variable_value: Option<&Variable> = get_variable(variables, data.array.to_string().clone());
                
                let list_content: Vec<VariableValue> = get_variable_vector(variable_value.unwrap_or(&Variable::new(VariableValue::Array(vec![]))).value.clone());

                let mut new_list_content: Vec<VariableValue> = list_content.clone();
                new_list_content.remove(data.index as usize);

                set_variable(variables, data.array.to_string().clone(), VariableValue::Array(new_list_content));
            }
            Execution::GetArrayLength { data } => {
                let variable_value: Option<&Variable> = get_variable(variables, data.array.to_string().clone());

                let list_content: Vec<VariableValue> = get_variable_vector(variable_value.unwrap_or(&Variable::new(VariableValue::Array(vec![]))).value.clone());

                set_variable(variables, data.output.to_string().clone(), VariableValue::Number(list_content.len() as f64));
            }
            Execution::LoopArray { data, variables: variables_set, code_inside } => {
                let variable_value: Option<&Variable> = get_variable(variables, data.array.to_string().clone());

                let list_content: Vec<VariableValue> = get_variable_vector(variable_value.unwrap_or(&Variable::new(VariableValue::Array(vec![]))).value.clone());
                
                let mut value_variable: Option<String> = None;
                for variable in variables_set {
                    if variable.type_ == "item".to_string() {
                        value_variable = Some(variable.name.clone());
                    }
                }
                let variable_name: String = value_variable.unwrap_or("".to_string());

                for (_index, value) in list_content.iter().enumerate() {
                    set_variable(variables, variable_name.clone(), value.clone());

                    execute_macro_code(&code_inside.loop_.as_ref().unwrap_or_default().executes, variables, stop_execution, macro_.clone());
                }
            }
            Execution::GetArrayIndex { data } => {
                let variable_value: Option<&Variable> = get_variable(variables, data.array.to_string().clone());

                let list_content: Vec<VariableValue> = get_variable_vector(variable_value.unwrap_or(&Variable::new(VariableValue::Array(vec![]))).value.clone());

                set_variable(variables, data.output.to_string().clone(), list_content[data.index as usize].clone());
            }
            Execution::SetArrayIndex { data } => {
                let variable_value: Option<&Variable> = get_variable(variables, data.array.to_string().clone());

                let list_content: Vec<VariableValue> = get_variable_vector(variable_value.unwrap_or(&Variable::new(VariableValue::Array(vec![]))).value.clone());

                let new_value: Option<&Variable> = get_variable(variables, data.data.to_string().clone());

                let mut new_list_content: Vec<VariableValue> = list_content.clone();
                new_list_content[data.index as usize] = new_value.unwrap_or(&Variable::new(VariableValue::Number(0.0))).value.clone();

                set_variable(variables, data.array.to_string().clone(), VariableValue::Array(new_list_content));
            }
            Execution::GetFolderContents { data } => {
                let mut list_content: Vec<VariableValue> = Vec::new();

                for entry in fs::read_dir(&data.path).unwrap() {
                    let entry = entry.unwrap();
                    let path = entry.path();
                    let file_path = path.to_str().unwrap().to_string();

                    list_content.push(VariableValue::String(file_path));
                }

                set_variable(variables, data.output.to_string().clone(), VariableValue::Array(list_content));
            }
            Execution::Log { data } => {
                let message: String = data.message.clone();

                let current_log_content: String = get_log_content().unwrap();

                let new_log_content: String = format!("{}{}\n", current_log_content, message);

                write_to_log(new_log_content);
                
                update_log_page();
            }
            Execution::ClearLog { } => {
                write_to_log("".to_string());

                update_log_page();
            }
            Execution::SplitString { data } => {
                let string_content: String = parse_string(&data.string, variables);

                let mut list_content: Vec<VariableValue> = Vec::new();

                // Split the string by data.splitter
                for split in string_content.split(&data.splitter) {
                    list_content.push(VariableValue::String(split.to_string()));
                }

                set_variable(variables, data.output.to_string().clone(), VariableValue::Array(list_content));
            }
            Execution::JoinStrings { data } => {
                let variable_value: Option<&Variable> = get_variable(variables, data.array.to_string().clone());

                let list_content: Vec<VariableValue> = get_variable_vector(variable_value.unwrap_or(&Variable::new(VariableValue::Array(vec![]))).value.clone());

                let list_content_strings: Vec<String> = list_content.iter().map(|x| parse_string(&get_variable_string(x.clone()), variables)).collect();

                let result: String = list_content_strings.join(&data.joiner);

                set_variable(variables, data.output.to_string().clone(), VariableValue::String(result));
            }
            Execution::ReverseArray { data } => {
                let variable_value: Option<&Variable> = get_variable(variables, data.array.to_string().clone());

                let list_content: Vec<VariableValue> = get_variable_vector(variable_value.unwrap_or(&Variable::new(VariableValue::Array(vec![]))).value.clone());

                let mut new_list_content: Vec<VariableValue> = list_content.clone();
                new_list_content.reverse();

                set_variable(variables, data.output.to_string().clone(), VariableValue::Array(new_list_content));
            }
            Execution::SortArray { data } => {
                let variable_value: Option<&Variable> = get_variable(variables, data.array.to_string().clone());

                let list_content: Vec<VariableValue> = get_variable_vector(variable_value.unwrap_or(&Variable::new(VariableValue::Array(vec![]))).value.clone());

                let mut new_list_content: Vec<VariableValue> = list_content.clone();
                new_list_content.sort();

                set_variable(variables, data.output.to_string().clone(), VariableValue::Array(new_list_content));
            }
        }
    }
}

fn evaluate_condition(condition: &Condition, variables: &mut Variables) -> Condition {
    match condition {
        Condition::Boolean { value: _ } => {
            return condition.clone();
        },
        Condition::Number { value: _ } => {
            return condition.clone();
        },
        Condition::Comparison { left, comparison, right } => {
            let left_result: f64 = get_condition_number(evaluate_condition(left, variables));
            let right_result: f64 = get_condition_number(evaluate_condition(right, variables));
            match comparison.as_str() {
                ">" => {
                    return Condition::Boolean{ value: left_result > right_result };
                },
                "<" => {
                    return Condition::Boolean{ value: left_result < right_result };
                },
                ">=" => {
                    return Condition::Boolean{ value: left_result >= right_result };
                },
                "<=" => {
                    return Condition::Boolean{ value: left_result <= right_result };
                },
                "==" => {
                    return Condition::Boolean{ value: left_result == right_result };
                },
                "!==" => {
                    return Condition::Boolean{ value: left_result != right_result };
                },
                _ => todo!()
            }
        },
        Condition::Logical { left, kind, right } => {
            let left_result: bool = get_condition_bool(evaluate_condition(left, variables));
            let right_result: bool = get_condition_bool(evaluate_condition(right, variables));
            match kind.as_str() {
                "and" => {
                    return Condition::Boolean{ value: left_result && right_result };
                },
                "or" => {
                    return Condition::Boolean{ value: left_result || right_result };
                },
                "not" => {
                    return Condition::Boolean{ value: !right_result };
                },
                _ => todo!()
            }
        },
        Condition::Variable { variable } => {
            return Condition::Number{ value: get_variable_number(
                get_variable(variables, variable.to_string()).unwrap_or( &Variable::new(VariableValue::Number(0.0)) ).value.clone()
            ) };
        },
        Condition::Expression { expression } => {
            return Condition::Number{ value: get_expression_number(
                evaluate_expression(expression, variables)
            ) }
        }
    }
}

fn evaluate_expression(expression: &Expression, variables: &mut Variables) -> Expression {
    match expression {
        Expression::Number { value: _ } => {
            return expression.clone();
        },
        Expression::Variable { variable } => {
            return Expression::Number{ value: get_variable_number(
                get_variable(variables, variable.to_string()).unwrap_or( &Variable::new(VariableValue::Number(0.0)) ).value.clone()
            ) };
        },
        Expression::Arithmetic { left, kind, right } => {
            let left_result: f64 = get_expression_number(evaluate_expression(left, variables));
            let right_result: f64 = get_expression_number(evaluate_expression(right, variables));
            match kind.as_str() {
                "addition" => {
                    return Expression::Number{ value: left_result + right_result };
                },
                "subtraction" => {
                    return Expression::Number{ value: left_result - right_result };
                },
                "division" => {
                    return Expression::Number{ value: left_result / right_result };
                },
                "multiplication" => {
                    return Expression::Number{ value: left_result * right_result };
                },
                "modulo" => {
                    return Expression::Number{ value: left_result % right_result };
                },
                "exponent" => {
                    return Expression::Number{ value: left_result.powf(right_result) };
                },
                _ => todo!()
            }
        },
        Expression::Bitwise { left, kind, right } => {
            let left_result: i64 = get_expression_number(evaluate_expression(left, variables)).round() as i64;
            let right_result: i64 = if kind == "not" { 0 } else { get_expression_number(evaluate_expression(right, variables)).round() as i64 };
            match kind.as_str() {
                "and" => {
                    return Expression::Number{ value: (left_result & right_result) as f64 };
                },
                "or" => {
                    return Expression::Number{ value: (left_result | right_result) as f64 };
                },
                "xor" => {
                    return Expression::Number{ value: (left_result ^ right_result) as f64 };
                },
                "not" => {
                    return Expression::Number{ value: (!right_result) as f64 };
                },
                "leftshift" => {
                    return Expression::Number{ value: (left_result << right_result) as f64 };
                },
                "signrightshift" => {
                    return Expression::Number{ value: (left_result >> right_result) as f64 };
                },
                _ => todo!()
            }
        }
    }
}

fn get_expression_number(value: Expression) -> f64 {
    match value {
        Expression::Number { value } => {
            return value;
        },
        _ => {
            return 0.0;
        }
    }
}

fn get_condition_bool(value: Condition) -> bool {
    match value {
        Condition::Boolean { value } => {
            return value;
        },
        Condition::Number { value } => {
            if value == 0.0 {
                return false;
            } else {
                return true;
            }
        },
        _ => {
            return false;
        }
    }
}

fn get_condition_number(value: Condition) -> f64 {
    match value {
        Condition::Boolean { value } => {
            return if value {1.0} else {0.0};
        },
        Condition::Number { value } => {
            return value;
        },
        _ => {
            return 0.0;
        }
    }
}

fn set_variable(variables: &mut Variables, variable: String, value: VariableValue) {
    *variables.entry(variable).or_insert(Variable::new(value.clone())) = Variable::new(value.clone());
}

fn get_variable(variables: &mut Variables, variable: String) -> Option<&Variable> {
    return variables.get(&variable);
}

fn get_log_path() -> Option<String> {
    // TODO: Do this in a better way
    let app_handle: Option<AppHandle> = get_app_handle();

    match app_handle {
        Some(app_handle) => {
            let path_resolver: PathResolver = app_handle.path_resolver();
            let log_path: String = path_resolver.log_dir().unwrap().to_str().unwrap().to_string();

            let path: &Path = Path::new(&log_path);
            let parent: &Path = path.parent().unwrap().parent().unwrap();

            // TODO: Add multiple logs
            let mut folder = parent.to_path_buf();
            folder.push("CodeMacros");
            folder.push("Logs");
            folder.push("log.txt");

            // println!("{}", folder.to_str().unwrap());
            let folder_path: String = folder.to_str().unwrap().to_string();

            return Some(folder_path);
        }
        None => {
            println!("Waiting to get app handle for macro to execute...");
            return None;
        }
    }
}

fn get_log_content() -> Option<String> {
    let log_path: Option<String> = get_log_path();

    match log_path {
        Some(log_path) => {
            let content: String = fs::read_to_string(log_path).unwrap_or(String::from(""));
            return Some(content);
        }
        None => {
            return None;
        }
    }
}

fn write_to_log(content: String) {
    let log_path: Option<String> = get_log_path();

    match log_path {
        Some(log_path) => {
            // Make sure the log_path folder exists
            fs::create_dir_all(Path::new(&log_path).parent().unwrap()).unwrap();
            fs::write(log_path, content).unwrap();
        }
        None => {
            // Do nothing
        }
    }
}

fn update_log_page() {
    let app_handle: Option<AppHandle> = get_app_handle();

    match app_handle {
        Some(app_handle) => {
            // Update the log if the user is currently on the log page
            let window: Window = app_handle.get_window("main").unwrap();
            window.eval("document.updateLog()").unwrap();
        }
        None => {
            // Do nothing
        }
    }
}