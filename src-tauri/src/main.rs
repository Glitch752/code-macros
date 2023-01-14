#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

use tauri::{ CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayMenuItem, SystemTrayEvent, AppHandle, Manager };

use std::thread;

use std::sync::Mutex;
use once_cell::sync::Lazy;

static MACROS: Lazy<Mutex<Macros>> = Lazy::new(|| Mutex::new(Macros::new()));
static APPHANDLE: Lazy<Mutex<Option<AppHandle>>> = Lazy::new(|| Mutex::new(None));

mod initiators;
mod execute;

use initiators::*;
use initiators::keypress::*;
use initiators::cron::*;

use execute::*;

fn get_macros() -> Macros {
    MACROS.lock().unwrap().clone()
}

fn set_macros(macros: Macros) {
    *MACROS.lock().unwrap() = macros;
}

fn get_app_handle() -> Option<AppHandle> {
    APPHANDLE.lock().unwrap().clone()
}

fn set_app_handle(app_handle: AppHandle) {
    *APPHANDLE.lock().unwrap() = Some(app_handle);
}

fn main() {
    thread::spawn(move || {
        listen_initiator_keypress();
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
        .setup(|app| {
            set_app_handle(app.handle());

            Ok(())
        })
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
            if let SystemTrayEvent::MenuItemClick { id, .. } = event {
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
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

use serde::{ Deserialize, Serialize };

type Macros = Vec<Macro>;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Macro {
    pub name: String,
    pub description: String,
    pub macro_: MacroMacro,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MacroMacro {
    pub initiators: Option<Vec<Initiator>>,
    pub functions: Option<Vec<Function>>,
}

#[tauri::command]
fn update_macros(macros: Macros) {
    println!("Updating macros");
    set_macros(macros);
    listen_initiator_cron();
}