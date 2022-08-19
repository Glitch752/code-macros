#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

use tauri::{ CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayMenuItem, SystemTrayEvent };
use tauri::Manager;

// use std::collections::HashMap;

use std::thread;

use std::sync::{ Arc, Mutex };

use once_cell::sync::Lazy;

static MACROS: Lazy<Mutex<Macros>> = Lazy::new(|| Mutex::new(Macros::new()));

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

        println!("{:?}", keys_pressed);

        print_macros(get_macros());
    });

    // Call this to start listening for bound inputs.
    inputbot::handle_input_events();
}

use serde::{ Deserialize, Serialize };
use serde_json::value::Value;

type Macros = Vec<Macro>;

#[derive(Serialize, Deserialize, Clone)]
struct Macro {
    name: String,
    description: String,
    macro_: MacroMacro,
}

impl std::fmt::Debug for Macro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\nName: {:?}\nDescription: {:?}\nMacro:{:?}\n",
            self.name,
            self.description,
            self.macro_
        )
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct MacroMacro {
    initiators: Option<Vec<Initiator>>,
    functions: Option<Vec<Function>>,
}

impl std::fmt::Debug for MacroMacro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n Initiators: {:?} \n Functions: {:?}\n", self.initiators, self.functions)
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct Initiator {
    type_: String,
    data: Value,
    executes: Vec<Execution>,
}

impl std::fmt::Debug for Initiator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\n Type: {:?} \n Data: {:?} \n Executes: {:?}",
            self.type_,
            self.data,
            self.executes
        )
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct Execution {
    type_: String,
    data: Value,
    code_inside: Value,
}

impl std::fmt::Debug for Execution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\n Type: {:?} \n Data: {:?} \n Code inside: {:?}",
            self.type_,
            self.data,
            self.code_inside
        )
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct Function {
    name: String,
    parameters: Vec<Parameter>,
    executes: Vec<Execution>,
}

impl std::fmt::Debug for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\n Name: {:?} \n Parameters: {:?} \n Executes: {:?}",
            self.name,
            self.parameters,
            self.executes
        )
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct Parameter {
    name: String,
    type_: String,
    default_value: String,
}

impl std::fmt::Debug for Parameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\n Name: {:?} \n Type: {:?} \n Default value: {:?}",
            self.name,
            self.type_,
            self.default_value
        )
    }
}

#[tauri::command]
fn update_macros(macros: Macros) {
    set_macros(macros);
}

fn print_macros(macros: Macros) {
    for macro_ in macros {
        println!("{}", macro_.name);
    }
}