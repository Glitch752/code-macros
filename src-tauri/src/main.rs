#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayMenuItem, SystemTrayEvent};
use tauri::Manager;

// use std::collections::HashMap;

fn main() {
  println!("Main ran");

  tauri::Builder::default()
    .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
      println!("{}, {argv:?}, {cwd}", app.package_info().name);
      let window = app.get_window("main").unwrap();
      window.show().unwrap();
    }))
    .invoke_handler(tauri::generate_handler![update_macros])
    .system_tray(
      SystemTray::new().with_menu(
        SystemTrayMenu::new()
          .add_item(
            CustomMenuItem::new("open".to_string(), "Open")
          )
          .add_native_item(SystemTrayMenuItem::Separator)
          .add_item(
            CustomMenuItem::new("quit".to_string(), "Quit")
          )
      )
    )
    .on_system_tray_event(|app, event| match event {
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
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
use serde::{Deserialize, Serialize};
use serde_json::value::Value;

type Macros = Vec<Macro>;

#[derive(Serialize, Deserialize)]
struct Macro {
  name: String,
  description: String,
  macro_: MacroMacro,
}

#[derive(Serialize, Deserialize)]
struct MacroMacro {
  initiators: Vec<Initiator>,
  functions: Vec<Function>,
}

#[derive(Serialize, Deserialize)]
struct Initiator {
  type_: String,
  data: Value,
  executes: Vec<Execution>
}

#[derive(Serialize, Deserialize)]
struct Execution {
  type_: String,
  data: Value,
  code_inside: Value
}

#[derive(Serialize, Deserialize)]
struct Function {
  name: String,
  parameters: Vec<Parameter>,
  executes: Vec<Execution>
}

#[derive(Serialize, Deserialize)]
struct Parameter {
  name: String,
  type_: String,
  default_value: String,
}

#[tauri::command]
fn update_macros(macros: Macros) {
  println!("Macros updated");
  for macro_ in macros {
    println!("{}", macro_.name);
  }
}