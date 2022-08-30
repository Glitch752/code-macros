#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

use tauri::{ CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayMenuItem, SystemTrayEvent };
use tauri::api::notification::Notification;
use tauri::Manager;

// use std::collections::HashMap;

use std::thread;

use std::sync::{ Arc, Mutex };

use once_cell::sync::Lazy;

static MACROS: Lazy<Mutex<Macros>> = Lazy::new(|| Mutex::new(Macros::new()));

static MAX_LOOP_ITERATIONS: u64 = 100;

fn get_macros() -> Macros {
    MACROS.lock().unwrap().clone()
}

fn set_macros(macros: Macros) {
    *MACROS.lock().unwrap() = macros;
}

fn main() {
    thread::spawn(move || {
        listen_macro_actions();
    });

    tauri::Builder
        ::default()
        .plugin(
            tauri_plugin_single_instance::init(|app, argv, cwd| {
                println!("{}, {argv:?}, {cwd}", app.package_info().name);
                let window = app.get_window("main").unwrap();
                window.show().unwrap();
            })
        )
        .invoke_handler(tauri::generate_handler![update_macros])
        .system_tray(
            SystemTray::new().with_menu(
                SystemTrayMenu::new()
                    .add_item(CustomMenuItem::new("open".to_string(), "Open"))
                    .add_native_item(SystemTrayMenuItem::Separator)
                    .add_item(CustomMenuItem::new("quit".to_string(), "Quit"))
            )
        )
        .on_system_tray_event(|app, event| {
            match event {
                SystemTrayEvent::MenuItemClick { id, .. } => {
                    // let item_handle = app.tray_handle().get_item(&id);
                    match id.as_str() {
                        "quit" => {
                            let window = app.get_window("main").unwrap();
                            window.close().unwrap();
                        }
                        "open" => {
                            let window = app.get_window("main").unwrap();
                            window.show().unwrap();
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

use inputbot::{ KeybdKey };

use std::collections::HashMap;

fn listen_macro_actions() {
    let keys_pressed: Arc<Mutex<HashMap<KeybdKey, bool>>> = Arc::new(Mutex::new(HashMap::new()));
    KeybdKey::bind_all(move |event| {

        let mut keys_pressed = keys_pressed.lock().unwrap();

        keys_pressed.insert(event, true);

        // Loop through all keys and check if they are pressed. If not, remove them from the map.
        let mut remove = Vec::new();
        for key in keys_pressed.keys() {
            if !KeybdKey::is_pressed(*key) {
                remove.push(*key);
            }
        }
        for key in remove {
            keys_pressed.remove(&key);
        }


        let mut keys_pressed_js: Vec<String> = vec![];
        for (key, value) in keys_pressed.iter() {
            if *value {
                keys_pressed_js.push(js_key(*key));
            }
        }

        'macros: for macro_ in get_macros() {
            // Check if macro_.macro_.initiators is Some
            if macro_.macro_.initiators.is_some() {
                let initiators = macro_.macro_.initiators.as_ref().unwrap();
                // Check if the initiators are pressed
                for initiator in initiators {
                    if initiator.type_ == "keypress" {
                        let keys = initiator.data.keys.as_ref().unwrap();
                        for key in keys {
                            if !keys_pressed_js.contains(key) {
                                continue 'macros;
                            }
                        }
                        run_macro_initiator(initiator.clone(), macro_.clone());
                    }
                }
            }
        }
    });

    // Call this to start listening for bound inputs.
    inputbot::handle_input_events();
}

fn run_macro_initiator(initiator: Initiator, macro_: Macro) {
    println!("Running macro initiator from macro \"{}\"", macro_.name);
    thread::spawn(move || {
        let mut new_variables: Variables = Variables::new();
        execute_macro_code(&initiator.executes, &mut new_variables, &mut false);
    });
}

type Variables = HashMap<String, Variable>;

#[derive(Debug)]
struct Variable {
    value: VariableValue,
}

impl Variable {
    fn new(value: VariableValue) -> Variable<> {
        Variable { value: value }
    }
}

#[derive(Clone, Debug)]
enum VariableValue {
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

fn execute_macro_code(code: &Vec<Execution>, variables: &mut Variables, stop_execution: &mut bool) {
    for execution in code {
        if *stop_execution {
            return;
        }

        match execution.type_.as_str() {
            "wait" => {
                let time = execution.data.time.as_ref().unwrap();
                thread::sleep(std::time::Duration::from_millis((time * 1000.0) as u64));
            }
            "notification" => {
                let title = execution.data.title.as_ref().unwrap();
                let message = execution.data.message.as_ref().unwrap();
                let _ = Notification::new("code-macros")
                    .title(parse_string(title, variables))
                    .body(parse_string(message, variables))
                    .show();
            }
            "fromtoloop" => {
                let from: f64 = *execution.data.from.as_ref().unwrap_or(&f64::from(0));
                let to: f64 = *execution.data.to.as_ref().unwrap_or(&f64::from(4));
                let step: f64 = *execution.data.step.as_ref().unwrap_or(&f64::from(1));
                let mut i: f64 = from;
                let mut iterations: u64 = 0;
                let mut value_variable: Option<String> = None;
                for variable in &execution.variables {
                    if variable.type_ == "value".to_string() {
                        value_variable = Some(variable.name.clone());
                    }
                }
                let variable_name: String = value_variable.unwrap_or("".to_string());

                if to > from {
                    while i <= to {
                        set_variable(variables, variable_name.to_string().clone(), VariableValue::Number(i));
                        execute_macro_code(&execution.code_inside.loop_.as_ref().unwrap_or_default().executes, variables, stop_execution);
                        i += step;
                        iterations += 1;
                        if iterations > MAX_LOOP_ITERATIONS {
                            break;
                        }
                    }
                } else {
                    while i >= to {
                        set_variable(variables, variable_name.to_string().clone(), VariableValue::Number(i));
                        execute_macro_code(&execution.code_inside.loop_.as_ref().unwrap_or_default().executes, variables, stop_execution);
                        i += step;
                        iterations += 1;
                        if iterations > MAX_LOOP_ITERATIONS {
                            break;
                        }
                    }
                }
            },
            "whileloop" => {
                // TODO: Properly implement variables so this loop can be used.
                let condition: &Condition = &execution.data.condition.as_ref().unwrap();

                let mut value_variable: Option<String> = None;
                for variable in &execution.variables {
                    if variable.type_ == "iteration".to_string() {
                        value_variable = Some(variable.name.clone());
                    }
                }
                let variable_name: String = value_variable.unwrap_or("".to_string());

                let mut i: u64 = 0;

                while get_condition_bool(evaluate_condition(condition, variables)) {
                    set_variable(variables, variable_name.to_string().clone(), VariableValue::Number(i as f64));
                    i += 1;
                    execute_macro_code(&execution.code_inside.then.as_ref().unwrap_or_default().executes, variables, stop_execution);
                }
            },
            "if" => {
                // TODO: Properly implement variables so 'if' can be used
                let condition: &Condition = &execution.data.condition.as_ref().unwrap();
                if get_condition_bool(evaluate_condition(condition, variables)) {
                    execute_macro_code(&execution.code_inside.then.as_ref().unwrap_or_default().executes, variables, stop_execution);
                } else {
                    execute_macro_code(&execution.code_inside.else_.as_ref().unwrap_or_default().executes, variables, stop_execution);
                }
            },
            "stop" => {
                *stop_execution = true;
            }
            _ => todo!()
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

use serde::{ Deserialize, Serialize };
// use serde_json::value::Value;

type Macros = Vec<Macro>;

// TODO: Refactor types to use #[serde(tag = "type")]

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Macro {
    name: String,
    description: String,
    macro_: MacroMacro,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct MacroMacro {
    initiators: Option<Vec<Initiator>>,
    functions: Option<Vec<Function>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Initiator {
    type_: String,
    data: InitiatorData,
    executes: Vec<Execution>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct InitiatorData {
    keys: Option<Vec<String>>,
    activate_time: Option<String>,
    time: Option<InitiatorKeypressTime>,
    cron: Option<String>,
    app_path: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct InitiatorKeypressTime {
    min: f64,
    max: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Execution {
    type_: String,
    data: ExecutionData,
    variables: Vec<VariableType>,
    code_inside: ExecutionCodeInside
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct ExecutionData {
    time: Option<f64>,
    title: Option<String>,
    message: Option<String>,
    from: Option<f64>,
    to: Option<f64>,
    step: Option<f64>,
    condition: Option<Condition>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct VariableType {
    type_: String,
    name: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type_")]
#[serde(rename_all = "lowercase")]
enum Condition {
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
    Variable { variable: String }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct ExecutionCodeInside {
    loop_: Option<ExecutionWrapper>,
    then: Option<ExecutionWrapper>,
    else_: Option<ExecutionWrapper>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct ExecutionWrapper {
    executes: Vec<Execution>
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
struct Function {
    name: String,
    parameters: Vec<Parameter>,
    executes: Vec<Execution>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Parameter {
    name: String,
    type_: String,
    default_value: String,
}

#[tauri::command]
fn update_macros(macros: Macros) {
    set_macros(macros);
}

// fn print_macros(macros: Macros) {
//     for macro_ in macros {
//         println!("{}", macro_.name);
//     }
// }

// Turn the key enum into the same format as comes from the macro config
fn js_key(key: KeybdKey) -> String {
    let mut enum_to_key:HashMap<String, String> = HashMap::new();
    enum_to_key.insert("AKey".to_string(),         "a".to_string()        );
    enum_to_key.insert("BKey".to_string(),         "b".to_string()        );
    enum_to_key.insert("CKey".to_string(),         "c".to_string()        );
    enum_to_key.insert("DKey".to_string(),         "d".to_string()        );
    enum_to_key.insert("EKey".to_string(),         "e".to_string()        );
    enum_to_key.insert("FKey".to_string(),         "f".to_string()        );
    enum_to_key.insert("GKey".to_string(),         "g".to_string()        );
    enum_to_key.insert("HKey".to_string(),         "h".to_string()        );
    enum_to_key.insert("IKey".to_string(),         "i".to_string()        );
    enum_to_key.insert("JKey".to_string(),         "j".to_string()        );
    enum_to_key.insert("KKey".to_string(),         "k".to_string()        );
    enum_to_key.insert("LKey".to_string(),         "l".to_string()        );
    enum_to_key.insert("MKey".to_string(),         "m".to_string()        );
    enum_to_key.insert("NKey".to_string(),         "n".to_string()        );
    enum_to_key.insert("OKey".to_string(),         "o".to_string()        );
    enum_to_key.insert("PKey".to_string(),         "p".to_string()        );
    enum_to_key.insert("QKey".to_string(),         "q".to_string()        );
    enum_to_key.insert("RKey".to_string(),         "r".to_string()        );
    enum_to_key.insert("SKey".to_string(),         "s".to_string()        );
    enum_to_key.insert("TKey".to_string(),         "t".to_string()        );
    enum_to_key.insert("UKey".to_string(),         "u".to_string()        );
    enum_to_key.insert("VKey".to_string(),         "v".to_string()        );
    enum_to_key.insert("WKey".to_string(),         "w".to_string()        );
    enum_to_key.insert("XKey".to_string(),         "x".to_string()        );
    enum_to_key.insert("YKey".to_string(),         "y".to_string()        );
    enum_to_key.insert("ZKey".to_string(),         "z".to_string()        );
    enum_to_key.insert("Numrow0Key".to_string(),   "0".to_string()        );
    enum_to_key.insert("Numrow1Key".to_string(),   "1".to_string()        );
    enum_to_key.insert("Numrow2Key".to_string(),   "2".to_string()        );
    enum_to_key.insert("Numrow3Key".to_string(),   "3".to_string()        );
    enum_to_key.insert("Numrow4Key".to_string(),   "4".to_string()        );
    enum_to_key.insert("Numrow5Key".to_string(),   "5".to_string()        );
    enum_to_key.insert("Numrow6Key".to_string(),   "6".to_string()        );
    enum_to_key.insert("Numrow7Key".to_string(),   "7".to_string()        );
    enum_to_key.insert("Numrow8Key".to_string(),   "8".to_string()        );
    enum_to_key.insert("Numrow9Key".to_string(),   "9".to_string()        );
    enum_to_key.insert("Numpad0Key".to_string(),   "0".to_string()        );
    enum_to_key.insert("Numpad1Key".to_string(),   "1".to_string()        );
    enum_to_key.insert("Numpad2Key".to_string(),   "2".to_string()        );
    enum_to_key.insert("Numpad3Key".to_string(),   "3".to_string()        );
    enum_to_key.insert("Numpad4Key".to_string(),   "4".to_string()        );
    enum_to_key.insert("Numpad5Key".to_string(),   "5".to_string()        );
    enum_to_key.insert("Numpad6Key".to_string(),   "6".to_string()        );
    enum_to_key.insert("Numpad7Key".to_string(),   "7".to_string()        );
    enum_to_key.insert("Numpad8Key".to_string(),   "8".to_string()        );
    enum_to_key.insert("Numpad9Key".to_string(),   "9".to_string()        );
    enum_to_key.insert("BackspaceKey".to_string(), "backspace".to_string());
    enum_to_key.insert("TabKey".to_string(),       "tab".to_string()      );
    enum_to_key.insert("EnterKey".to_string(),     "enter".to_string()    );
    enum_to_key.insert("EscapeKey".to_string(),    "escape".to_string()   );
    enum_to_key.insert("SpaceKey".to_string(),     "space".to_string()    );
    enum_to_key.insert("F1Key".to_string(),        "f1".to_string()       );
    enum_to_key.insert("F2Key".to_string(),        "f2".to_string()       );
    enum_to_key.insert("F3Key".to_string(),        "f3".to_string()       );
    enum_to_key.insert("F4Key".to_string(),        "f4".to_string()       );
    enum_to_key.insert("F5Key".to_string(),        "f5".to_string()       );
    enum_to_key.insert("F6Key".to_string(),        "f6".to_string()       );
    enum_to_key.insert("F7Key".to_string(),        "f7".to_string()       );
    enum_to_key.insert("F8Key".to_string(),        "f8".to_string()       );
    enum_to_key.insert("F9Key".to_string(),        "f9".to_string()       );
    enum_to_key.insert("F10Key".to_string(),       "f10".to_string()      );
    enum_to_key.insert("CapsLockKey".to_string(),  "capslock".to_string() );
    enum_to_key.insert("QuoteKey".to_string(),     "'".to_string()        );
    enum_to_key.insert("SemicolonKey".to_string(), ";".to_string()        );
    enum_to_key.insert("CommaKey".to_string(),     ",".to_string()        );
    enum_to_key.insert("PeriodKey".to_string(),    ".".to_string()        );
    enum_to_key.insert("SlashKey".to_string(),     "/".to_string()        );
    enum_to_key.insert("BackslashKey".to_string(), "\\".to_string()       );
    enum_to_key.insert("MinusKey".to_string(),     "-".to_string()        );
    enum_to_key.insert("EqualKey".to_string(),     "=".to_string()        );
    enum_to_key.insert("LBracketKey".to_string(),  "[".to_string()        );
    enum_to_key.insert("RBracketKey".to_string(),  "]".to_string()        );
    enum_to_key.insert("BackquoteKey".to_string(), "`".to_string()        );
    enum_to_key.insert("LShiftKey".to_string(),    "shift".to_string()    );
    enum_to_key.insert("RShiftKey".to_string(),    "shift".to_string()    );
    enum_to_key.insert("LControlKey".to_string(),  "control".to_string()  );
    enum_to_key.insert("RControlKey".to_string(),  "control".to_string()  );

    let key_string = format!("{:?}", key);

    return enum_to_key.get(&key_string).unwrap().to_string();
}