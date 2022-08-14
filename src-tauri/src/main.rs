#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayMenuItem, SystemTrayEvent};
use tauri::Manager;

use std::collections::HashMap;

fn main() {
  println!("Main ran");

  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![update_macros])
    .system_tray(
      SystemTray::new().with_menu(
        SystemTrayMenu::new()
          .add_item(
            CustomMenuItem::new("quit".to_string(), "Quit")
          )
          .add_native_item(SystemTrayMenuItem::Separator)
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
          _ => {}
        }
      }
      _ => {}
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}


#[tauri::command]
fn update_macros(macros: String) {
  println!("{}", macros);
}