use serde::{ Deserialize, Serialize };

use std::collections::HashMap;

use tauri::{api::notification::Notification};

static MAX_LOOP_ITERATIONS: u64 = 100000;

use std::thread;

use super::Macro;

use super::initiators::Initiator;

use inputbot::{KeySequence, MouseCursor, KeybdKey, MouseButton, get_keybd_key};

use std::fs;

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

// TODO: This may be better as an Adjacently tagged enum so we don't need another struct for every data type

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
    }
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
}

fn get_variable_string(variable_value: VariableValue) -> String {
    match variable_value {
        VariableValue::String(value) => {
            return value;
        },
        VariableValue::Number(value) => {
            return value.to_string();
        }
    }
}

fn get_variable_number(variable_value: VariableValue) -> f64 {
    match variable_value {
        VariableValue::String(_value) => {
            return 0.0;
        },
        VariableValue::Number(value) => {
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
                        VariableValue::String(_) => "string".to_string()
                    }
                    None => "undefined".to_string()
                };

                set_variable(variables, data.output.to_string().clone(), VariableValue::String(variable_type));
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

fn parse_string<'a>(string: &'a String, variables: &'a mut Variables) -> String {
    let variable_split: Vec<&str> = string.split("{{").collect();
    let mut result = String::from(variable_split[0]);
    let mut index: u64 = 0;
    for split in variable_split {
        index += 1;
        if index == 1 {
            continue;
        }
        let halves: Vec<&str> = split.split("}}").collect();
        let variable_name: String = halves[0].to_string();
        let variable_value = get_variable(variables, variable_name);
        result.push_str(&get_variable_string(
            variable_value.unwrap_or(
                &Variable::new(VariableValue::String("undefined".to_string()))
            ).value.clone()
        ));
        if halves.len() > 1 {
            result.push_str(&halves[1].to_string());
        }
    }

    let characters: std::str::Chars = result.chars();

    let mut skip: bool = false;

    let mut index: usize= 0;
    let mut characters_copy: Vec<char> = characters.clone().collect();
    for character in characters {
        if skip {
            skip = false;
            continue;
        }
        if character == '\\' {
            characters_copy.remove(index);
            skip = true;
        }
        index += 1;
    }

    result = characters_copy.into_iter().collect();

    return result;
}

fn set_variable(variables: &mut Variables, variable: String, value: VariableValue) {
    *variables.entry(variable).or_insert(Variable::new(value.clone())) = Variable::new(value.clone());
}

fn get_variable(variables: &mut Variables, variable: String) -> Option<&Variable> {
    return variables.get(&variable);
}